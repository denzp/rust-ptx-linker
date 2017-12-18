; ModuleID = 'nvptx-module'
source_filename = "nvptx-module"
target datalayout = "e-i64:64-v16:16-v32:32-n16:32:64"
target triple = "nvptx64-nvidia-cuda"

@const_0 = available_externally unnamed_addr constant [2 x double] [double 5.000000e-01, double 1.500000e+00], align 8
@const_3 = available_externally unnamed_addr constant [2 x double] [double 2.000000e+00, double 3.000000e+00], align 8

; Function Attrs: nounwind
define ptx_kernel void @line(double* nocapture readonly %src, double* nocapture %dst, i64 %line) unnamed_addr #0 {
start:
  %0 = tail call i32 @llvm.nvvm.read.ptx.sreg.ntid.x()
  %1 = tail call i32 @llvm.nvvm.read.ptx.sreg.ctaid.x()
  %2 = mul i32 %1, %0
  %3 = tail call i32 @llvm.nvvm.read.ptx.sreg.tid.x()
  %4 = add i32 %2, %3
  %5 = sext i32 %4 to i64
  %6 = getelementptr inbounds double, double* %src, i64 %5
  %7 = load double, double* %6, align 8
  %8 = getelementptr inbounds [2 x double], [2 x double]* @const_0, i64 0, i64 %line
  %9 = load double, double* %8, align 8
  %10 = fmul double %7, %9
  %11 = getelementptr inbounds [2 x double], [2 x double]* @const_3, i64 0, i64 %line
  %12 = load double, double* %11, align 8
  %13 = getelementptr inbounds double, double* %dst, i64 %5
  %14 = fadd double %10, %12
  store double %14, double* %13, align 8
  ret void
}

; Function Attrs: nounwind readnone
declare i32 @llvm.nvvm.read.ptx.sreg.ntid.x() unnamed_addr #1

; Function Attrs: nounwind readnone
declare i32 @llvm.nvvm.read.ptx.sreg.ctaid.x() unnamed_addr #1

; Function Attrs: nounwind readnone
declare i32 @llvm.nvvm.read.ptx.sreg.tid.x() unnamed_addr #1

attributes #0 = { nounwind }
attributes #1 = { nounwind readnone }
