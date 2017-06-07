; ModuleID = 'nvptx-module'
source_filename = "nvptx-module"
target datalayout = "e-i64:64-v16:16-v32:32-n16:32:64"
target triple = "nvptx64-nvidia-cuda"

; Function Attrs: norecurse nounwind readnone
define ptx_kernel signext i32 @bar() unnamed_addr #0 {
  ret i32 42
}

; Function Attrs: norecurse nounwind readnone
define void @rust_begin_unwind() unnamed_addr #0 {
  ret void
}

; Function Attrs: norecurse nounwind readnone
define i32 @_ZN3lib3foo17h6be2379489c2179bE() unnamed_addr #0 {
  ret i32 42
}

attributes #0 = { norecurse nounwind readnone "no-frame-pointer-elim"="true" }

!llvm.module.flags = !{!0}

!0 = !{i32 2, !"Debug Info Version", i32 3}
