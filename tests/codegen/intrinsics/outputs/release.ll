; ModuleID = 'nvptx-module'
source_filename = "nvptx-module"
target datalayout = "e-i64:64-v16:16-v32:32-n16:32:64"
target triple = "nvptx64-nvidia-cuda"

module asm ".section .rustc"

%InputPixel = type { [0 x i8], i8, [0 x i8], i8, [0 x i8], i8, [0 x i8] }

; Function Attrs: nounwind
define ptx_kernel void @rgb2gray(%InputPixel* nocapture readonly %src, i8* nocapture %dst, i32 zeroext %width) unnamed_addr #0 {
start:
  %0 = tail call i32 @llvm.nvvm.read.ptx.sreg.ntid.y()
  %1 = tail call i32 @llvm.nvvm.read.ptx.sreg.ctaid.y()
  %2 = mul i32 %1, %0
  %3 = tail call i32 @llvm.nvvm.read.ptx.sreg.tid.y()
  %4 = add i32 %2, %3
  %5 = tail call i32 @llvm.nvvm.read.ptx.sreg.ntid.x()
  %6 = tail call i32 @llvm.nvvm.read.ptx.sreg.ctaid.x()
  %7 = mul i32 %6, %5
  %8 = tail call i32 @llvm.nvvm.read.ptx.sreg.tid.x()
  %9 = mul i32 %4, %width
  %10 = add i32 %8, %9
  %11 = add i32 %10, %7
  %12 = sext i32 %11 to i64
  %13 = getelementptr inbounds %InputPixel, %InputPixel* %src, i64 %12, i32 0, i64 0
  %14 = load i8, i8* %13, align 1
  %15 = zext i8 %14 to i16
  %16 = getelementptr inbounds %InputPixel, %InputPixel* %src, i64 %12, i32 3
  %17 = load i8, i8* %16, align 1
  %18 = zext i8 %17 to i16
  %19 = add nuw nsw i16 %18, %15
  %20 = getelementptr inbounds %InputPixel, %InputPixel* %src, i64 %12, i32 5
  %21 = load i8, i8* %20, align 1
  %22 = zext i8 %21 to i16
  %23 = add nuw nsw i16 %19, %22
  %24 = udiv i16 %23, 3
  %25 = getelementptr inbounds i8, i8* %dst, i64 %12
  %26 = trunc i16 %24 to i8
  store i8 %26, i8* %25, align 1
  ret void
}

; Function Attrs: nounwind readnone
declare i32 @llvm.nvvm.read.ptx.sreg.ntid.y() unnamed_addr #1

; Function Attrs: nounwind readnone
declare i32 @llvm.nvvm.read.ptx.sreg.ctaid.y() unnamed_addr #1

; Function Attrs: nounwind readnone
declare i32 @llvm.nvvm.read.ptx.sreg.tid.y() unnamed_addr #1

; Function Attrs: nounwind readnone
declare i32 @llvm.nvvm.read.ptx.sreg.ntid.x() unnamed_addr #1

; Function Attrs: nounwind readnone
declare i32 @llvm.nvvm.read.ptx.sreg.ctaid.x() unnamed_addr #1

; Function Attrs: nounwind readnone
declare i32 @llvm.nvvm.read.ptx.sreg.tid.x() unnamed_addr #1

attributes #0 = { nounwind }
attributes #1 = { nounwind readnone }
