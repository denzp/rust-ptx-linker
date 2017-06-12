#include <llvm-c/Core.h>
#include <llvm/IR/Function.h>
#include <llvm/IR/Module.h>
#include <sstream>

std::string FindExternalReferences(const llvm::Module &module) {
  std::ostringstream external_refs_stream;

  for (auto &function : module) {
    if (function.isDeclaration()) {
      external_refs_stream << function.getName().data() << ";";
    }
  }

  return external_refs_stream.str();
}

// Returns `true` (in terms of LLVM C Api `true` == `1` to be correct) if some
// external references are found. Also writes semicolon (";") separated
// list to the `out_messages`.
extern "C" LLVMBool IsExternalReferencesExists(LLVMModuleRef module,
                                               char **out_messages) {
  auto module_ptr = llvm::unwrap(module);
  auto external_refs = FindExternalReferences(*module_ptr);

  if (external_refs.size() == 0) {
    return 0;
  }

  // remove trailing ";"
  external_refs.pop_back();

  *out_messages = strdup(external_refs.c_str());

  return 1;
}
