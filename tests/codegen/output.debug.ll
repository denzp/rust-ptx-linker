; ModuleID = 'nvptx-module'
source_filename = "nvptx-module"
target datalayout = "e-i64:64-v16:16-v32:32-n16:32:64"
target triple = "nvptx64-nvidia-cuda"

; Function Attrs: nounwind
define ptx_kernel signext i32 @bar() unnamed_addr #0 {
  %1 = call i32 @_ZN3lib3foo17h6be2379489c2179bE()
  br label %2

; <label>:2:                                      ; preds = %0
  ret i32 %1
}

; Function Attrs: nounwind
define void @rust_begin_unwind() unnamed_addr #0 {
  ret void
}

define i32 @_ZN3lib3foo17h6be2379489c2179bE() unnamed_addr #1 {
  ret i32 42
}

attributes #0 = { nounwind "no-frame-pointer-elim"="true" }
attributes #1 = { "no-frame-pointer-elim"="true" }

!llvm.module.flags = !{!0}

!0 = !{i32 2, !"Debug Info Version", i32 3}
