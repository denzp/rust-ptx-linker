#include "visitors.h"

using llvm::Module;
using llvm::BasicBlock;

bool CallVisitor::runOnBasicBlock(BasicBlock &block) {
  for (auto &instruction : block) {
    auto *call = llvm::dyn_cast_or_null<llvm::CallInst>(&instruction);

    if (call && call->getCalledFunction()) {
      OnCall(block.getParent(), call->getCalledFunction());
    }
  }

  return false;
}

bool GlobalValueVisitor::runOnModule(Module &module) {
  for (auto &fn : module.functions()) {
    OnGlobalValue(&fn);
  }

  for (auto &var : module.globals()) {
    OnGlobalValue(&var);
  }

  return false;
}

char CallVisitor::ID = 0;
char GlobalValueVisitor::ID = 0;
