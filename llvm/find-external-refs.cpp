#include "helpers/pass-runner.h"
#include "helpers/visitors.h"
#include <llvm-c/Core.h>
#include <sstream>

using llvm::Function;

class UndefinedReferencesFinder : public CallVisitor {
public:
  typedef std::vector<std::string>::const_iterator iterator;

public:
  unsigned count() const { return undefined_refs.size(); }
  iterator begin() const { return undefined_refs.begin(); }
  iterator end() const { return undefined_refs.end(); }

private:
  std::vector<std::string> undefined_refs;

  void OnCall(Function *caller, Function *callee) override {
    if (callee->isDeclaration()) {
      undefined_refs.push_back(callee->getName());
    }
  }
};

class UndefinedReferencesPrinter {
public:
  UndefinedReferencesPrinter(UndefinedReferencesFinder::iterator refs_begin,
                             UndefinedReferencesFinder::iterator refs_end) {
    std::ostringstream writer;
    writer << *refs_begin++;

    while (refs_begin != refs_end) {
      writer << ";" << *refs_begin++;
    }

    buffer = writer.str();
  }

  const char *c_str() const { return buffer.c_str(); }

private:
  std::string buffer;
};

// Returns count of external references that are found.
// Also writes semicolon (";") separated list to the `out_messages`.
extern "C" unsigned FindExternalReferences(LLVMModuleRef wrapped_module,
                                           char **out_messages) {
  auto &module = *llvm::unwrap(wrapped_module);
  Runner<UndefinedReferencesFinder> finder(module);

  if (finder->count() > 0) {
    UndefinedReferencesPrinter printer(finder->begin(), finder->end());

    *out_messages = strdup(printer.c_str());
  }

  return finder->count();
}
