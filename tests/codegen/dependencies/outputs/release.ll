; ModuleID = 'nvptx-module'
source_filename = "nvptx-module"
target datalayout = "e-i64:64-v16:16-v32:32-n16:32:64"
target triple = "nvptx64-nvidia-cuda"

; Function Attrs: norecurse nounwind
define ptx_kernel void @top_level_kernel(double* nocapture readonly %x, double* nocapture %y, double %a) unnamed_addr #0 {
start:
  %0 = load double, double* %x, align 8
  %1 = fmul double %0, %0
  %2 = fmul double %1, %a
  store double %2, double* %y, align 8
  ret void
}

; Function Attrs: norecurse nounwind
define ptx_kernel void @dummy_math_kernel(double* nocapture readonly %x, double* nocapture %y) unnamed_addr #0 {
start:
  %0 = load double, double* %x, align 8
  %1 = fmul double %0, %0
  store double %1, double* %y, align 8
  ret void
}

; Function Attrs: norecurse nounwind
define ptx_kernel void @dummy_utils_kernel(double* nocapture readonly %x1, double* nocapture readonly %x2, double* nocapture %y) unnamed_addr #0 {
start:
  %0 = load double, double* %x1, align 8
  %1 = load double, double* %x2, align 8
  %2 = fmul double %0, %1
  store double %2, double* %y, align 8
  ret void
}

attributes #0 = { norecurse nounwind }
