; ModuleID = 'nvptx-module'
source_filename = "nvptx-module"
target datalayout = "e-i64:64-v16:16-v32:32-n16:32:64"
target triple = "nvptx64-nvidia-cuda"

; Function Attrs: norecurse nounwind
define ptx_kernel void @top_level_kernel(double* nocapture readonly, double* nocapture, double) unnamed_addr #0 {
  %4 = load double, double* %0, align 8
  %5 = tail call double @dummy_mul(double %4, double %4) #2
  %6 = fmul double %5, %2
  store double %6, double* %1, align 8
  ret void
}

; Function Attrs: norecurse nounwind
define ptx_kernel void @dummy_math_kernel(double* nocapture readonly, double* nocapture) unnamed_addr #0 {
  %3 = load double, double* %0, align 8
  %4 = tail call double @dummy_mul(double %3, double %3) #2
  store double %4, double* %1, align 8
  ret void
}

; Function Attrs: noinline norecurse nounwind readnone
define double @dummy_mul(double, double) unnamed_addr #1 {
  %3 = tail call double @dummy_mul_inner(double %0, double %1)
  ret double %3
}

; Function Attrs: noinline norecurse nounwind readnone
define double @dummy_mul_inner(double, double) unnamed_addr #1 {
  %3 = fmul double %0, %1
  ret double %3
}

; Function Attrs: norecurse nounwind
define ptx_kernel void @dummy_utils_kernel(double* nocapture readonly, double* nocapture readonly, double* nocapture) unnamed_addr #0 {
  %4 = load double, double* %0, align 8
  %5 = load double, double* %1, align 8
  %6 = tail call double @dummy_mul(double %4, double %5)
  store double %6, double* %2, align 8
  ret void
}

attributes #0 = { norecurse nounwind }
attributes #1 = { noinline norecurse nounwind readnone }
attributes #2 = { nounwind }
