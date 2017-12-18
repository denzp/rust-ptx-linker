#pragma once

#include <llvm/IR/Function.h>
#include <llvm/IR/Instructions.h>
#include <llvm/IR/Module.h>
#include <llvm/Pass.h>

class CallVisitor : public llvm::BasicBlockPass {
public:
  CallVisitor() : BasicBlockPass(ID) {}

protected:
  virtual void OnCall(llvm::Function *caller, llvm::Function *callee) = 0;

private:
  static char ID;

  bool runOnBasicBlock(llvm::BasicBlock &block) override;
};

class GlobalValueVisitor : public llvm::ModulePass {
public:
  GlobalValueVisitor() : ModulePass(ID) {}

protected:
  virtual void OnGlobalValue(llvm::GlobalValue *value) = 0;

private:
  static char ID;

  bool runOnModule(llvm::Module &module) override;
};

class ModuleVisitor : public llvm::ModulePass {
public:
  ModuleVisitor() : ModulePass(ID) {}

protected:
  virtual void OnModule(llvm::Module *module) = 0;

private:
  static char ID;

  bool runOnModule(llvm::Module &module) override;
};
