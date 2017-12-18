#include "helpers/pass-runner.h"
#include "helpers/visitors.h"
#include <algorithm>
#include <llvm-c/Core.h>
#include <llvm/IR/Constants.h>
#include <llvm/IR/LegacyPassManager.h>
#include <llvm/Transforms/IPO.h>
#include <llvm/Transforms/Scalar.h>
#include <set>

using llvm::Function;
using llvm::GlobalValue;
using llvm::GlobalVariable;
using llvm::Module;
using llvm::isa;

typedef std::set<Function *> FunctionSet;
typedef std::set<GlobalVariable *> VariableSet;

class UsedFunctionsFinder : public CallVisitor {
public:
  typedef FunctionSet::const_iterator iterator;

public:
  iterator begin() const { return used_functions.begin(); }
  iterator end() const { return used_functions.end(); }

private:
  FunctionSet used_functions;

  void OnCall(Function *caller, Function *callee) override {
    // TODO: check if callee is kernel? is it legal?
    if (caller->getCallingConv() == llvm::CallingConv::PTX_Kernel) {
      used_functions.insert(callee);
    }
  }
};

class Internalizer : public GlobalValueVisitor {
public:
  void stripExceptFunctions(UsedFunctionsFinder::iterator preserved_fns_begin,
                            UsedFunctionsFinder::iterator preserved_fns_end) {
    FunctionSet internal;

    auto internal_inserter = std::inserter(internal, internal.begin());
    std::set_difference(available_functions.begin(), available_functions.end(),
                        preserved_fns_begin, preserved_fns_end,
                        internal_inserter);

    removeInternalFunctions(internal);
  }

  void stripDeadGlobalVars(Module &module) {
    for (auto *var : available_variables) {
      var->setLinkage(GlobalValue::AvailableExternallyLinkage);
    }

    llvm::legacy::PassManager manager;
    manager.add(llvm::createGlobalDCEPass());
    manager.run(module);
  }

protected:
  FunctionSet available_functions;
  VariableSet available_variables;

  void OnGlobalValue(GlobalValue *value) override {
    auto *func = llvm::dyn_cast_or_null<Function>(value);
    auto *var = llvm::dyn_cast_or_null<GlobalVariable>(value);

    if (func && func->getCallingConv() != llvm::CallingConv::PTX_Kernel) {
      available_functions.insert(func);
      return;
    }

    if (var) {
      available_variables.insert(var);
      return;
    }
  }

  void removeInternalFunctions(FunctionSet internal_fns) {
    for (auto *fn : internal_fns) {
      fn->replaceAllUsesWith(llvm::UndefValue::get(fn->getType()));
      fn->eraseFromParent();
    }
  }
};

class StripModuleAsm : public ModuleVisitor {
protected:
  void OnModule(Module *module) override { module->setModuleInlineAsm(""); }
};

// Remove every function but kernels and their dependent functions.
extern "C" void StripInternalFunctions(LLVMModuleRef wrapped_module) {
  auto &module = *llvm::unwrap(wrapped_module);

  Runner<UsedFunctionsFinder> alive_functions(module);
  Runner<Internalizer> internalizer(module);
  Runner<StripModuleAsm> asm_stripper(module);

  internalizer->stripExceptFunctions(alive_functions->begin(),
                                     alive_functions->end());

  internalizer->stripDeadGlobalVars(module);
}
