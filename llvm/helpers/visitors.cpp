#include "visitors.h"

using llvm::BasicBlock;
using llvm::Module;

bool CallVisitor::runOnBasicBlock(BasicBlock &block) {
  auto touched = false;

  for (auto &instruction : block) {
    auto *call = llvm::dyn_cast_or_null<llvm::CallInst>(&instruction);

    if (call && call->getCalledFunction()) {
      touched |= OnCall(block.getParent(), call->getCalledFunction());
    }
  }

  return touched;
}

bool GlobalValueVisitor::runOnModule(Module &module) {
  auto touched = false;

  for (auto &fn : module.functions()) {
    touched |= OnGlobalValue(&fn);
  }

  for (auto &var : module.globals()) {
    touched |= OnGlobalValue(&var);
  }

  return touched;
}

bool ModuleVisitor::runOnModule(Module &module) { return OnModule(&module); }

char CallVisitor::ID = 0;
char GlobalValueVisitor::ID = 0;
char ModuleVisitor::ID = 0;
