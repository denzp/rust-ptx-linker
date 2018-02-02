; ModuleID = 'nvptx-module'
source_filename = "nvptx-module"
target datalayout = "e-i64:64-v16:16-v32:32-n16:32:64"
target triple = "nvptx64-nvidia-cuda"

; Function Attrs: inlinehint nounwind
define hidden double* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h8005b29a2720eb4cE"(double*, i64) unnamed_addr #0 !dbg !5 {
start:
  %tmp_ret = alloca double*, align 8
  %count = alloca i64, align 8
  %self = alloca double*, align 8
  store double* %0, double** %self
  call void @llvm.dbg.declare(metadata double** %self, metadata !17, metadata !19), !dbg !20
  store i64 %1, i64* %count
  call void @llvm.dbg.declare(metadata i64* %count, metadata !21, metadata !19), !dbg !20
  %2 = load double*, double** %self, !dbg !22
  %3 = load i64, i64* %count, !dbg !22
  %4 = getelementptr inbounds double, double* %2, i64 %3, !dbg !22
  store double* %4, double** %tmp_ret, !dbg !22
  %5 = load double*, double** %tmp_ret, !dbg !22
  br label %bb1, !dbg !22

bb1:                                              ; preds = %start
  ret double* %5, !dbg !23
}

; Function Attrs: nounwind readnone speculatable
declare void @llvm.dbg.declare(metadata, metadata, metadata) #1

; Function Attrs: inlinehint nounwind
define hidden double* @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$6offset17h605c7797bd870d7aE"(double*, i64) unnamed_addr #0 !dbg !24 {
start:
  %tmp_ret = alloca double*, align 8
  %count = alloca i64, align 8
  %self = alloca double*, align 8
  store double* %0, double** %self
  call void @llvm.dbg.declare(metadata double** %self, metadata !28, metadata !19), !dbg !29
  store i64 %1, i64* %count
  call void @llvm.dbg.declare(metadata i64* %count, metadata !30, metadata !19), !dbg !29
  %2 = load double*, double** %self, !dbg !31
  %3 = load i64, i64* %count, !dbg !31
  %4 = getelementptr inbounds double, double* %2, i64 %3, !dbg !31
  store double* %4, double** %tmp_ret, !dbg !31
  %5 = load double*, double** %tmp_ret, !dbg !31
  br label %bb1, !dbg !31

bb1:                                              ; preds = %start
  ret double* %5, !dbg !32
}

; Function Attrs: nounwind
define ptx_kernel void @top_level_kernel(double*, double*, double) unnamed_addr #2 !dbg !33 {
start:
  %a = alloca double, align 8
  %y = alloca double*, align 8
  %x = alloca double*, align 8
  store double* %0, double** %x
  call void @llvm.dbg.declare(metadata double** %x, metadata !37, metadata !19), !dbg !38
  store double* %1, double** %y
  call void @llvm.dbg.declare(metadata double** %y, metadata !39, metadata !19), !dbg !38
  store double %2, double* %a
  call void @llvm.dbg.declare(metadata double* %a, metadata !40, metadata !19), !dbg !38
  %3 = load double*, double** %x, !dbg !41
  %4 = call double* @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$6offset17h605c7797bd870d7aE"(double* %3, i64 0), !dbg !41
  br label %bb1, !dbg !41

bb1:                                              ; preds = %start
  %5 = load double, double* %4, !dbg !41
  %6 = call double @dummy_square(double %5), !dbg !41
  br label %bb2, !dbg !41

bb2:                                              ; preds = %bb1
  %7 = load double, double* %a, !dbg !41
  %8 = load double*, double** %y, !dbg !41
  %9 = call double* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h8005b29a2720eb4cE"(double* %8, i64 0), !dbg !41
  br label %bb3, !dbg !41

bb3:                                              ; preds = %bb2
  %10 = fmul double %6, %7, !dbg !41
  store double %10, double* %9, !dbg !41
  ret void, !dbg !42
}

; Function Attrs: nounwind
define double @dummy_square(double %x) unnamed_addr #3 {
start:
  %0 = tail call double @dummy_mul(double %x, double %x)
  ret double %0
}

; Function Attrs: nounwind
define ptx_kernel void @dummy_math_kernel(double* nocapture readonly %x, double* nocapture %y) unnamed_addr #3 {
start:
  %0 = load double, double* %x, align 8
  %1 = tail call double @dummy_mul(double %0, double %0) #3
  store double %1, double* %y, align 8
  ret void
}

; Function Attrs: norecurse nounwind readnone
define double @dummy_mul(double %x1, double %x2) unnamed_addr #4 {
start:
  %0 = fmul double %x1, %x2
  ret double %0
}

; Function Attrs: norecurse nounwind
define ptx_kernel void @dummy_utils_kernel(double* nocapture readonly %x1, double* nocapture readonly %x2, double* nocapture %y) unnamed_addr #5 {
start:
  %0 = load double, double* %x1, align 8
  %1 = load double, double* %x2, align 8
  %2 = fmul double %0, %1
  store double %2, double* %y, align 8
  ret void
}

attributes #0 = { inlinehint nounwind "no-frame-pointer-elim"="true" }
attributes #1 = { nounwind readnone speculatable }
attributes #2 = { nounwind "no-frame-pointer-elim"="true" }
attributes #3 = { nounwind }
attributes #4 = { norecurse nounwind readnone }
attributes #5 = { norecurse nounwind }

!llvm.dbg.cu = !{!0, !3}
!llvm.module.flags = !{!4}

!0 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !1, producer: "clang LLVM (rustc version 1.24.0-nightly (9fe7aa353 2017-12-11))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, enums: !2)
!1 = !DIFile(filename: "src/lib.rs", directory: "/home/den/rust-ptx-linker/examples/depenencies")
!2 = !{}
!3 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !1, producer: "clang LLVM (rustc version 1.24.0-nightly (9fe7aa353 2017-12-11))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, enums: !2)
!4 = !{i32 2, !"Debug Info Version", i32 3}
!5 = distinct !DISubprogram(name: "offset<f64>", linkageName: "_ZN4core3ptr8{{impl}}11offset<f64>E", scope: !7, file: !6, line: 1260, type: !10, isLocal: false, isDefinition: true, scopeLine: 1260, flags: DIFlagPrototyped, isOptimized: false, unit: !0, templateParams: !15, variables: !2)
!6 = !DIFile(filename: "/home/den/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/ptr.rs", directory: "")
!7 = !DINamespace(name: "{{impl}}", scope: !8)
!8 = !DINamespace(name: "ptr", scope: !9)
!9 = !DINamespace(name: "core", scope: null)
!10 = !DISubroutineType(types: !11)
!11 = !{!12, !12, !14}
!12 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut f64", baseType: !13, size: 64, align: 64)
!13 = !DIBasicType(name: "f64", size: 64, encoding: DW_ATE_float)
!14 = !DIBasicType(name: "isize", size: 64, encoding: DW_ATE_signed)
!15 = !{!16}
!16 = !DITemplateTypeParameter(name: "T", type: !13)
!17 = !DILocalVariable(name: "self", arg: 1, scope: !5, file: !18, line: 1, type: !12)
!18 = !DIFile(filename: "src/lib.rs", directory: "")
!19 = !DIExpression()
!20 = !DILocation(line: 1, scope: !5)
!21 = !DILocalVariable(name: "count", arg: 2, scope: !5, file: !18, line: 1, type: !14)
!22 = !DILocation(line: 1261, scope: !5)
!23 = !DILocation(line: 1262, scope: !5)
!24 = distinct !DISubprogram(name: "offset<f64>", linkageName: "_ZN4core3ptr8{{impl}}11offset<f64>E", scope: !7, file: !6, line: 622, type: !25, isLocal: false, isDefinition: true, scopeLine: 622, flags: DIFlagPrototyped, isOptimized: false, unit: !0, templateParams: !15, variables: !2)
!25 = !DISubroutineType(types: !26)
!26 = !{!27, !27, !14}
!27 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const f64", baseType: !13, size: 64, align: 64)
!28 = !DILocalVariable(name: "self", arg: 1, scope: !24, file: !18, line: 1, type: !27)
!29 = !DILocation(line: 1, scope: !24)
!30 = !DILocalVariable(name: "count", arg: 2, scope: !24, file: !18, line: 1, type: !14)
!31 = !DILocation(line: 623, scope: !24)
!32 = !DILocation(line: 624, scope: !24)
!33 = distinct !DISubprogram(name: "top_level_kernel", linkageName: "_ZN7example16top_level_kernelE", scope: !34, file: !1, line: 8, type: !35, isLocal: false, isDefinition: true, scopeLine: 8, flags: DIFlagPrototyped, isOptimized: false, unit: !3, templateParams: !2, variables: !2)
!34 = !DINamespace(name: "example", scope: null)
!35 = !DISubroutineType(types: !36)
!36 = !{null, !27, !12, !13}
!37 = !DILocalVariable(name: "x", arg: 1, scope: !33, file: !1, line: 1, type: !27)
!38 = !DILocation(line: 1, scope: !33)
!39 = !DILocalVariable(name: "y", arg: 2, scope: !33, file: !1, line: 1, type: !12)
!40 = !DILocalVariable(name: "a", arg: 3, scope: !33, file: !1, line: 1, type: !13)
!41 = !DILocation(line: 9, scope: !33)
!42 = !DILocation(line: 10, scope: !33)
