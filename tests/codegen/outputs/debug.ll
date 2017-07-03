; ModuleID = 'nvptx-module'
source_filename = "nvptx-module"
target datalayout = "e-p:64:64:64-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64-f32:32:32-f64:64:64-v16:16:16-v32:32:32-v64:64:64-v128:128:128-n16:32:64"
target triple = "nvptx64-nvidia-cuda"

; Function Attrs: nounwind
define ptx_kernel void @top_level_kernel(double* nocapture readonly, double* nocapture, double) unnamed_addr #0 {
start:
  %3 = load double, double* %0, align 8
  %4 = tail call double @dummy_square(double %3) #0
  %5 = fmul double %4, %2
  store double %5, double* %1, align 8
  ret void
}

define double @dummy_square(double) unnamed_addr {
start:
  %1 = tail call double @dummy_mul(double %0, double %0)
  ret double %1
}

; Function Attrs: nounwind
define ptx_kernel void @dummy_math_kernel(double* nocapture readonly, double* nocapture) unnamed_addr #0 {
start:
  %2 = load double, double* %0, align 8
  %3 = tail call double @dummy_mul(double %2, double %2) #0
  store double %3, double* %1, align 8
  ret void
}

; Function Attrs: norecurse nounwind readnone
define double @dummy_mul(double, double) unnamed_addr #1 {
start:
  %2 = fmul double %0, %1
  ret double %2
}

; Function Attrs: norecurse nounwind
define ptx_kernel void @dummy_utils_kernel(double* nocapture readonly, double* nocapture readonly, double* nocapture) unnamed_addr #2 {
start:
  %3 = load double, double* %0, align 8
  %4 = load double, double* %1, align 8
  %5 = fmul double %3, %4
  store double %5, double* %2, align 8
  ret void
}

attributes #0 = { nounwind }
attributes #1 = { norecurse nounwind readnone }
attributes #2 = { norecurse nounwind }
