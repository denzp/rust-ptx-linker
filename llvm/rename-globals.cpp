#include "helpers/pass-runner.h"
#include "helpers/visitors.h"
#include <algorithm>
#include <llvm-c/Core.h>
#include <llvm/IR/Constants.h>
#include <llvm/IR/LegacyPassManager.h>
#include <llvm/Transforms/IPO.h>
#include <llvm/Transforms/Scalar.h>

using llvm::Function;
using llvm::GlobalValue;
using llvm::GlobalVariable;
using llvm::isa;
using llvm::Module;

class GlobalVariableRenamer : public GlobalValueVisitor {
protected:
  bool OnGlobalValue(GlobalValue *value) override {
    auto *var = llvm::dyn_cast_or_null<GlobalVariable>(value);

    if (var) {
      auto name = var->getName().str();

      std::replace(name.begin(), name.end(), '.', '_');
      var->setName(name);
    }

    return false;
  }
};

// Global variables: replace "." with "_" in names
extern "C" void RenameGlobalVariables(LLVMModuleRef wrapped_module) {
  auto &module = *llvm::unwrap(wrapped_module);

  Runner<GlobalVariableRenamer> renamer(module);
}
