; ModuleID = 'nvptx-module'
source_filename = "nvptx-module"
target datalayout = "e-i64:64-v16:16-v32:32-n16:32:64"
target triple = "nvptx64-nvidia-cuda"

; Function Attrs: inlinehint nounwind
define hidden double* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h3cba4ed5c3f99324E"(double*, i64) unnamed_addr #0 !dbg !5 {
  %3 = alloca double*, align 8
  %4 = alloca i64, align 8
  %5 = alloca double*, align 8
  store double* %0, double** %5, align 8
  call void @llvm.dbg.declare(metadata double** %5, metadata !17, metadata !DIExpression()), !dbg !19
  store i64 %1, i64* %4, align 8
  call void @llvm.dbg.declare(metadata i64* %4, metadata !20, metadata !DIExpression()), !dbg !19
  %6 = load double*, double** %5, align 8, !dbg !21
  %7 = load i64, i64* %4, align 8, !dbg !21
  %8 = getelementptr inbounds double, double* %6, i64 %7, !dbg !21
  store double* %8, double** %3, align 8, !dbg !21
  %9 = load double*, double** %3, align 8, !dbg !21
  br label %10, !dbg !21

; <label>:10:                                     ; preds = %2
  ret double* %9, !dbg !22
}

; Function Attrs: nounwind readnone speculatable
declare void @llvm.dbg.declare(metadata, metadata, metadata) #1

; Function Attrs: inlinehint nounwind
define hidden double* @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$6offset17hfc4334ae278ada4bE"(double*, i64) unnamed_addr #0 !dbg !23 {
  %3 = alloca double*, align 8
  %4 = alloca i64, align 8
  %5 = alloca double*, align 8
  store double* %0, double** %5, align 8
  call void @llvm.dbg.declare(metadata double** %5, metadata !27, metadata !DIExpression()), !dbg !28
  store i64 %1, i64* %4, align 8
  call void @llvm.dbg.declare(metadata i64* %4, metadata !29, metadata !DIExpression()), !dbg !28
  %6 = load double*, double** %5, align 8, !dbg !30
  %7 = load i64, i64* %4, align 8, !dbg !30
  %8 = getelementptr inbounds double, double* %6, i64 %7, !dbg !30
  store double* %8, double** %3, align 8, !dbg !30
  %9 = load double*, double** %3, align 8, !dbg !30
  br label %10, !dbg !30

; <label>:10:                                     ; preds = %2
  ret double* %9, !dbg !31
}

; Function Attrs: nounwind
define ptx_kernel void @top_level_kernel(double*, double*, double) unnamed_addr #2 !dbg !32 {
  %4 = alloca double, align 8
  %5 = alloca double*, align 8
  %6 = alloca double*, align 8
  store double* %0, double** %6, align 8
  call void @llvm.dbg.declare(metadata double** %6, metadata !36, metadata !DIExpression()), !dbg !37
  store double* %1, double** %5, align 8
  call void @llvm.dbg.declare(metadata double** %5, metadata !38, metadata !DIExpression()), !dbg !37
  store double %2, double* %4, align 8
  call void @llvm.dbg.declare(metadata double* %4, metadata !39, metadata !DIExpression()), !dbg !37
  %7 = load double*, double** %6, align 8, !dbg !40
  %8 = call double* @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$6offset17hfc4334ae278ada4bE"(double* %7, i64 0), !dbg !40
  br label %9, !dbg !40

; <label>:9:                                      ; preds = %3
  %10 = load double, double* %8, align 8, !dbg !40
  %11 = call double @dummy_square(double %10), !dbg !40
  br label %12, !dbg !40

; <label>:12:                                     ; preds = %9
  %13 = load double, double* %4, align 8, !dbg !40
  %14 = load double*, double** %5, align 8, !dbg !40
  %15 = call double* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h3cba4ed5c3f99324E"(double* %14, i64 0), !dbg !40
  br label %16, !dbg !40

; <label>:16:                                     ; preds = %12
  %17 = fmul double %11, %13, !dbg !40
  store double %17, double* %15, align 8, !dbg !40
  ret void, !dbg !41
}

; Function Attrs: nounwind
define double @dummy_square(double) unnamed_addr #3 {
  %2 = tail call double @dummy_mul(double %0, double %0)
  ret double %2
}

; Function Attrs: nounwind
define ptx_kernel void @dummy_math_kernel(double* nocapture readonly, double* nocapture) unnamed_addr #3 {
  %3 = load double, double* %0, align 8
  %4 = tail call double @dummy_mul(double %3, double %3) #3
  store double %4, double* %1, align 8
  ret void
}

; Function Attrs: noinline norecurse nounwind readnone
define double @dummy_mul(double, double) unnamed_addr #4 {
  %3 = tail call double @dummy_mul_inner(double %0, double %1)
  ret double %3
}

; Function Attrs: noinline norecurse nounwind readnone
define double @dummy_mul_inner(double, double) unnamed_addr #4 {
  %3 = fmul double %0, %1
  ret double %3
}

; Function Attrs: norecurse nounwind
define ptx_kernel void @dummy_utils_kernel(double* nocapture readonly, double* nocapture readonly, double* nocapture) unnamed_addr #5 {
  %4 = load double, double* %0, align 8
  %5 = load double, double* %1, align 8
  %6 = tail call double @dummy_mul(double %4, double %5)
  store double %6, double* %2, align 8
  ret void
}

attributes #0 = { inlinehint nounwind "no-frame-pointer-elim"="true" }
attributes #1 = { nounwind readnone speculatable }
attributes #2 = { nounwind "no-frame-pointer-elim"="true" }
attributes #3 = { nounwind }
attributes #4 = { noinline norecurse nounwind readnone }
attributes #5 = { norecurse nounwind }

!llvm.dbg.cu = !{!0, !3}
!llvm.module.flags = !{!4}

!0 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !1, producer: "clang LLVM (rustc version 1.26.0-nightly (521d91c6b 2018-03-14))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, enums: !2)
!1 = !DIFile(filename: "src/lib.rs", directory: "/home/den/rust-ptx-linker/examples/depenencies")
!2 = !{}
!3 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !1, producer: "clang LLVM (rustc version 1.26.0-nightly (521d91c6b 2018-03-14))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, enums: !2)
!4 = !{i32 2, !"Debug Info Version", i32 3}
!5 = distinct !DISubprogram(name: "offset<f64>", linkageName: "_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h3cba4ed5c3f99324E", scope: !7, file: !6, line: 1239, type: !10, isLocal: true, isDefinition: true, scopeLine: 1239, flags: DIFlagPrototyped, isOptimized: false, unit: !0, templateParams: !15, variables: !2)
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
!19 = !DILocation(line: 1, scope: !5)
!20 = !DILocalVariable(name: "count", arg: 2, scope: !5, file: !18, line: 1, type: !14)
!21 = !DILocation(line: 1240, scope: !5)
!22 = !DILocation(line: 1241, scope: !5)
!23 = distinct !DISubprogram(name: "offset<f64>", linkageName: "_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$6offset17hfc4334ae278ada4bE", scope: !7, file: !6, line: 621, type: !24, isLocal: true, isDefinition: true, scopeLine: 621, flags: DIFlagPrototyped, isOptimized: false, unit: !0, templateParams: !15, variables: !2)
!24 = !DISubroutineType(types: !25)
!25 = !{!26, !26, !14}
!26 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const f64", baseType: !13, size: 64, align: 64)
!27 = !DILocalVariable(name: "self", arg: 1, scope: !23, file: !18, line: 1, type: !26)
!28 = !DILocation(line: 1, scope: !23)
!29 = !DILocalVariable(name: "count", arg: 2, scope: !23, file: !18, line: 1, type: !14)
!30 = !DILocation(line: 622, scope: !23)
!31 = !DILocation(line: 623, scope: !23)
!32 = distinct !DISubprogram(name: "top_level_kernel", linkageName: "top_level_kernel", scope: !33, file: !1, line: 8, type: !34, isLocal: false, isDefinition: true, scopeLine: 8, flags: DIFlagPrototyped, isOptimized: false, unit: !3, templateParams: !2, variables: !2)
!33 = !DINamespace(name: "example", scope: null)
!34 = !DISubroutineType(types: !35)
!35 = !{null, !26, !12, !13}
!36 = !DILocalVariable(name: "x", arg: 1, scope: !32, file: !1, line: 1, type: !26)
!37 = !DILocation(line: 1, scope: !32)
!38 = !DILocalVariable(name: "y", arg: 2, scope: !32, file: !1, line: 1, type: !12)
!39 = !DILocalVariable(name: "a", arg: 3, scope: !32, file: !1, line: 1, type: !13)
!40 = !DILocation(line: 9, scope: !32)
!41 = !DILocation(line: 10, scope: !32)
