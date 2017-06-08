; ModuleID = 'nvptx-module'
source_filename = "nvptx-module"
target datalayout = "e-p:64:64:64-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64-f32:32:32-f64:64:64-v16:16:16-v32:32:32-v64:64:64-v128:128:128-n16:32:64"
target triple = "nvptx64-nvidia-cuda"

; Function Attrs: nounwind
define ptx_kernel void @top_level_kernel(double* nocapture readonly, double* nocapture, double) unnamed_addr #0 {
  %4 = load double, double* %0, align 8
  %5 = tail call double @dummy_square(double %4) #0
  %6 = fmul double %5, %2
  store double %6, double* %1, align 8
  ret void
}

; Function Attrs: norecurse noreturn nounwind readnone
define void @rust_begin_unwind() unnamed_addr #1 {
  br label %1

; <label>:1:                                      ; preds = %1, %0
  br label %1
}

define double @dummy_square(double) unnamed_addr {
  %2 = tail call double @dummy_mul(double %0, double %0)
  ret double %2
}

; Function Attrs: nounwind
define ptx_kernel void @dummy_math_kernel(double* nocapture readonly, double* nocapture) unnamed_addr #0 {
  %3 = load double, double* %0, align 8
  %4 = tail call double @dummy_mul(double %3, double %3) #0
  store double %4, double* %1, align 8
  ret void
}

; Function Attrs: norecurse nounwind readnone
define double @dummy_mul(double, double) unnamed_addr #2 {
  %3 = fmul double %0, %1
  ret double %3
}

; Function Attrs: norecurse nounwind
define ptx_kernel void @dummy_utils_kernel(double* nocapture readonly, double* nocapture readonly, double* nocapture) unnamed_addr #3 {
  %4 = load double, double* %0, align 8
  %5 = load double, double* %1, align 8
  %6 = fmul double %4, %5
  store double %6, double* %2, align 8
  ret void
}

attributes #0 = { nounwind }
attributes #1 = { norecurse noreturn nounwind readnone }
attributes #2 = { norecurse nounwind readnone }
attributes #3 = { norecurse nounwind }
