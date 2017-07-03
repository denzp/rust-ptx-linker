#pragma once

#include <llvm/IR/LegacyPassManager.h>
#include <llvm/IR/Module.h>
#include <llvm/Pass.h>

template <typename P> class Runner {
public:
  template <typename... Args> Runner(llvm::Module &module, Args &&... params) {
    instance = new P(params...);

    manager.add(instance);
    manager.run(module);
  }

  P *operator->() { return instance; }

private:
  P *instance;
  llvm::legacy::PassManager manager;
};