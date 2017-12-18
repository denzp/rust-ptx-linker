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
  call void @llvm.dbg.declare(metadata double** %self, metadata !18, metadata !20), !dbg !21
  store i64 %1, i64* %count
  call void @llvm.dbg.declare(metadata i64* %count, metadata !22, metadata !20), !dbg !21
  %2 = load double*, double** %self, !dbg !23
  %3 = load i64, i64* %count, !dbg !23
  %4 = getelementptr inbounds double, double* %2, i64 %3, !dbg !23
  store double* %4, double** %tmp_ret, !dbg !23
  %5 = load double*, double** %tmp_ret, !dbg !23
  br label %bb1, !dbg !23

bb1:                                              ; preds = %start
  ret double* %5, !dbg !24
}

; Function Attrs: nounwind readnone
declare void @llvm.dbg.declare(metadata, metadata, metadata) #1

; Function Attrs: inlinehint nounwind
define hidden double* @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$6offset17h605c7797bd870d7aE"(double*, i64) unnamed_addr #0 !dbg !25 {
start:
  %tmp_ret = alloca double*, align 8
  %count = alloca i64, align 8
  %self = alloca double*, align 8
  store double* %0, double** %self
  call void @llvm.dbg.declare(metadata double** %self, metadata !29, metadata !20), !dbg !30
  store i64 %1, i64* %count
  call void @llvm.dbg.declare(metadata i64* %count, metadata !31, metadata !20), !dbg !30
  %2 = load double*, double** %self, !dbg !32
  %3 = load i64, i64* %count, !dbg !32
  %4 = getelementptr inbounds double, double* %2, i64 %3, !dbg !32
  store double* %4, double** %tmp_ret, !dbg !32
  %5 = load double*, double** %tmp_ret, !dbg !32
  br label %bb1, !dbg !32

bb1:                                              ; preds = %start
  ret double* %5, !dbg !33
}

; Function Attrs: nounwind
define ptx_kernel void @top_level_kernel(double*, double*, double) unnamed_addr #2 !dbg !34 {
start:
  %a = alloca double, align 8
  %y = alloca double*, align 8
  %x = alloca double*, align 8
  store double* %0, double** %x
  call void @llvm.dbg.declare(metadata double** %x, metadata !38, metadata !20), !dbg !39
  store double* %1, double** %y
  call void @llvm.dbg.declare(metadata double** %y, metadata !40, metadata !20), !dbg !39
  store double %2, double* %a
  call void @llvm.dbg.declare(metadata double* %a, metadata !41, metadata !20), !dbg !39
  %3 = load double*, double** %x, !dbg !42
  %4 = call double* @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$6offset17h605c7797bd870d7aE"(double* %3, i64 0), !dbg !42
  br label %bb1, !dbg !42

bb1:                                              ; preds = %start
  %5 = load double, double* %4, !dbg !42
  %6 = call double @dummy_square(double %5), !dbg !42
  br label %bb2, !dbg !42

bb2:                                              ; preds = %bb1
  %7 = load double, double* %a, !dbg !42
  %8 = load double*, double** %y, !dbg !42
  %9 = call double* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h8005b29a2720eb4cE"(double* %8, i64 0), !dbg !42
  br label %bb3, !dbg !42

bb3:                                              ; preds = %bb2
  %10 = fmul double %6, %7, !dbg !42
  store double %10, double* %9, !dbg !42
  ret void, !dbg !43
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
attributes #1 = { nounwind readnone }
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
!5 = distinct !DISubprogram(name: "offset<f64>", linkageName: "_ZN4core3ptr8{{impl}}11offset<f64>E", scope: !7, file: !6, line: 1260, type: !11, isLocal: false, isDefinition: true, scopeLine: 1260, flags: DIFlagPrototyped, isOptimized: false, unit: !0, templateParams: !16, variables: !2)
!6 = !DIFile(filename: "/home/den/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/ptr.rs", directory: "")
!7 = !DINamespace(name: "{{impl}}", scope: !9, file: !8)
!8 = !DIFile(filename: "<unknown>", directory: "")
!9 = !DINamespace(name: "ptr", scope: !10, file: !8)
!10 = !DINamespace(name: "core", scope: null, file: !8)
!11 = !DISubroutineType(types: !12)
!12 = !{!13, !13, !15}
!13 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut f64", baseType: !14, size: 64, align: 64)
!14 = !DIBasicType(name: "f64", size: 64, encoding: DW_ATE_float)
!15 = !DIBasicType(name: "isize", size: 64, encoding: DW_ATE_signed)
!16 = !{!17}
!17 = !DITemplateTypeParameter(name: "T", type: !14)
!18 = !DILocalVariable(name: "self", arg: 1, scope: !5, file: !19, line: 1, type: !13)
!19 = !DIFile(filename: "src/lib.rs", directory: "")
!20 = !DIExpression()
!21 = !DILocation(line: 1, scope: !5)
!22 = !DILocalVariable(name: "count", arg: 2, scope: !5, file: !19, line: 1, type: !15)
!23 = !DILocation(line: 1261, scope: !5)
!24 = !DILocation(line: 1262, scope: !5)
!25 = distinct !DISubprogram(name: "offset<f64>", linkageName: "_ZN4core3ptr8{{impl}}11offset<f64>E", scope: !7, file: !6, line: 622, type: !26, isLocal: false, isDefinition: true, scopeLine: 622, flags: DIFlagPrototyped, isOptimized: false, unit: !0, templateParams: !16, variables: !2)
!26 = !DISubroutineType(types: !27)
!27 = !{!28, !28, !15}
!28 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const f64", baseType: !14, size: 64, align: 64)
!29 = !DILocalVariable(name: "self", arg: 1, scope: !25, file: !19, line: 1, type: !28)
!30 = !DILocation(line: 1, scope: !25)
!31 = !DILocalVariable(name: "count", arg: 2, scope: !25, file: !19, line: 1, type: !15)
!32 = !DILocation(line: 623, scope: !25)
!33 = !DILocation(line: 624, scope: !25)
!34 = distinct !DISubprogram(name: "top_level_kernel", linkageName: "_ZN7example16top_level_kernelE", scope: !35, file: !1, line: 8, type: !36, isLocal: false, isDefinition: true, scopeLine: 8, flags: DIFlagPrototyped, isOptimized: false, unit: !3, templateParams: !2, variables: !2)
!35 = !DINamespace(name: "example", scope: null, file: !8)
!36 = !DISubroutineType(types: !37)
!37 = !{null, !28, !13, !14}
!38 = !DILocalVariable(name: "x", arg: 1, scope: !34, file: !1, line: 1, type: !28)
!39 = !DILocation(line: 1, scope: !34)
!40 = !DILocalVariable(name: "y", arg: 2, scope: !34, file: !1, line: 1, type: !13)
!41 = !DILocalVariable(name: "a", arg: 3, scope: !34, file: !1, line: 1, type: !14)
!42 = !DILocation(line: 9, scope: !34)
!43 = !DILocation(line: 10, scope: !34)
