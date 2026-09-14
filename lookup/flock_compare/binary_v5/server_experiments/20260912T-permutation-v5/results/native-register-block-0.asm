
/home/mscong1/mikros-experiments/20260912T-native64/permutation-binary-v4/iteration-v5/source/target/release/compare：     文件格式 elf64-x86-64


Disassembly of section .text:

000000000012c5f0 <mikros_permutation_binary_v5::fast_sumcheck::native::block>:
  12c5f0:	push   %rbp
  12c5f1:	mov    %rsp,%rbp
  12c5f4:	push   %r15
  12c5f6:	push   %r14
  12c5f8:	push   %r13
  12c5fa:	push   %r12
  12c5fc:	push   %rbx
  12c5fd:	and    $0xffffffffffffffc0,%rsp
  12c601:	sub    $0xfc0,%rsp
  12c608:	movq   $0x0,(%rsp)
  12c610:	sub    $0x1000,%rsp
  12c617:	movq   $0x0,(%rsp)
  12c61f:	sub    $0x1000,%rsp
  12c626:	movq   $0x0,(%rsp)
  12c62e:	sub    $0x1000,%rsp
  12c635:	movq   $0x0,(%rsp)
  12c63d:	sub    $0x4c0,%rsp
  12c644:	mov    %r9,0x90(%rsp)
  12c64c:	mov    %r8,0x18(%rsp)
  12c651:	mov    %rcx,0x20(%rsp)
  12c656:	mov    %rdx,%r14
  12c659:	mov    %rsi,%r13
  12c65c:	mov    %rdi,0xb8(%rsp)
  12c664:	mov    0x28(%rbp),%rbx
  12c668:	mov    0x20(%rbp),%r15
  12c66c:	mov    0x18(%rbp),%r12
  12c670:	lea    0x180(%rsp),%rdi
  12c678:	mov    $0x2a0,%edx
  12c67d:	xor    %esi,%esi
  12c67f:	call   *0xaeb23(%rip)        # 1db1a8 <memset@GLIBC_2.2.5>
  12c685:	mov    0x0(%r13),%rax
  12c689:	test   %rax,%rax
  12c68c:	je     12c751 <mikros_permutation_binary_v5::fast_sumcheck::native::block+0x161>
  12c692:	cmp    $0x1,%eax
  12c695:	jne    12c7bd <mikros_permutation_binary_v5::fast_sumcheck::native::block+0x1cd>
  12c69b:	mov    0x10(%r13),%rdx
  12c69f:	lea    (%r14,%r14,1),%rdi
  12c6a3:	mov    0x18(%rsp),%rax
  12c6a8:	lea    (%rax,%r14,1),%rsi
  12c6ac:	add    %rsi,%rsi
  12c6af:	cmp    %rdi,%rsi
  12c6b2:	mov    0x10(%rbp),%r11
  12c6b6:	jb     12d749 <mikros_permutation_binary_v5::fast_sumcheck::native::block+0x1159>
  12c6bc:	cmp    %rdx,%rsi
  12c6bf:	ja     12d749 <mikros_permutation_binary_v5::fast_sumcheck::native::block+0x1159>
  12c6c5:	mov    0x20(%r13),%rdx
  12c6c9:	lea    (%r11,%r14,1),%rsi
  12c6cd:	add    %rsi,%rsi
  12c6d0:	cmp    %rdi,%rsi
  12c6d3:	jb     12d766 <mikros_permutation_binary_v5::fast_sumcheck::native::block+0x1176>
  12c6d9:	cmp    %rdx,%rsi
  12c6dc:	ja     12d766 <mikros_permutation_binary_v5::fast_sumcheck::native::block+0x1176>
  12c6e2:	mov    0x18(%r13),%rax
  12c6e6:	mov    0x8(%r13),%r13
  12c6ea:	shl    $0x4,%rdi
  12c6ee:	cmp    $0x4,%rbx
  12c6f2:	mov    %rax,0xe8(%rsp)
  12c6fa:	jae    12c81e <mikros_permutation_binary_v5::fast_sumcheck::native::block+0x22e>
  12c700:	vxorps %xmm9,%xmm9,%xmm9
  12c705:	vpxor  %xmm0,%xmm0,%xmm0
  12c709:	vmovdqa %ymm0,0x140(%rsp)
  12c712:	vpxor  %xmm0,%xmm0,%xmm0
  12c716:	xor    %esi,%esi
  12c718:	vpxor  %xmm1,%xmm1,%xmm1
  12c71c:	vpxor  %xmm3,%xmm3,%xmm3
  12c720:	vpxor  %xmm4,%xmm4,%xmm4
  12c724:	vpxor  %xmm5,%xmm5,%xmm5
  12c728:	vpxor  %xmm6,%xmm6,%xmm6
  12c72c:	vpxor  %xmm7,%xmm7,%xmm7
  12c730:	vpxor  %xmm8,%xmm8,%xmm8
  12c735:	vpxor  %xmm2,%xmm2,%xmm2
  12c739:	vxorps %xmm10,%xmm10,%xmm10
  12c73e:	vmovaps %ymm10,0x120(%rsp)
  12c747:	vxorps %xmm10,%xmm10,%xmm10
  12c74c:	jmp    12cc72 <mikros_permutation_binary_v5::fast_sumcheck::native::block+0x682>
  12c751:	mov    0x10(%r13),%rdx
  12c755:	mov    0x18(%rsp),%rax
  12c75a:	lea    (%rax,%r14,1),%rsi
  12c75e:	shl    $0x2,%r14
  12c762:	shl    $0x2,%rsi
  12c766:	cmp    %r14,%rsi
  12c769:	jb     12d756 <mikros_permutation_binary_v5::fast_sumcheck::native::block+0x1166>
  12c76f:	cmp    %rdx,%rsi
  12c772:	ja     12d756 <mikros_permutation_binary_v5::fast_sumcheck::native::block+0x1166>
  12c778:	mov    0x18(%rsp),%rcx
  12c77d:	lea    0x0(,%rcx,4),%rsi
  12c785:	shl    $0x4,%r14
  12c789:	add    0x8(%r13),%r14
  12c78d:	mov    %r14,%rdi
  12c790:	mov    0x20(%rsp),%rdx
  12c795:	mov    0x90(%rsp),%r8
  12c79d:	mov    0x10(%rbp),%r9
  12c7a1:	lea    0x180(%rsp),%rax
  12c7a9:	push   %rax
  12c7aa:	push   %rbx
  12c7ab:	push   %r15
  12c7ad:	push   %r12
  12c7af:	call   12b350 <mikros_permutation_binary_v5::fast_sumcheck::native::dense::<true>>
  12c7b4:	add    $0x20,%rsp
  12c7b8:	jmp    12d39a <mikros_permutation_binary_v5::fast_sumcheck::native::block+0xdaa>
  12c7bd:	mov    0x8(%r13),%rax
  12c7c1:	mov    %rax,0x98(%rsp)
  12c7c9:	mov    0x10(%r13),%rax
  12c7cd:	mov    %rax,0x30(%rsp)
  12c7d2:	lea    0x440(%rsp),%rdi
  12c7da:	xor    %r13d,%r13d
  12c7dd:	mov    $0x4000,%edx
  12c7e2:	xor    %esi,%esi
  12c7e4:	call   *0xae9be(%rip)        # 1db1a8 <memset@GLIBC_2.2.5>
  12c7ea:	mov    0x10(%rbp),%r9
  12c7ee:	test   %r9,%r9
  12c7f1:	je     12d23a <mikros_permutation_binary_v5::fast_sumcheck::native::block+0xc4a>
  12c7f7:	mov    0x18(%rsp),%r10
  12c7fc:	lea    0xff(%r10),%rax
  12c803:	shr    $0x8,%rax
  12c807:	lea    0xff(%r9),%r13
  12c80e:	shr    $0x8,%r13
  12c812:	cmp    %rax,%r13
  12c815:	cmovae %rax,%r13
  12c819:	jmp    12d23f <mikros_permutation_binary_v5::fast_sumcheck::native::block+0xc4f>
  12c81e:	vmovdqa (%r12),%xmm0
  12c824:	vmovq  (%r12),%xmm1
  12c82a:	vmovdqa64 -0xd76f4(%rip),%zmm9        # 55140 <anon.e906d5a2e45a0236484a7009067c1ee4.23.llvm.6966055675071790356+0xc7>
  12c834:	vpermi2q %zmm1,%zmm0,%zmm9
  12c83a:	shl    $0x5,%r14
  12c83e:	lea    0xc0(%rax,%r14,1),%rax
  12c846:	lea    0xc0(%r13,%r14,1),%rcx
  12c84e:	vpxor  %xmm2,%xmm2,%xmm2
  12c852:	xor    %esi,%esi
  12c854:	vpbroadcastq -0xdf066(%rip),%zmm10        # 4d7f8 <anon.25c1abbc0d37fd1a879b913e2262f7b9.7.llvm.8827852567942549522+0x578>
  12c85e:	xor    %edx,%edx
  12c860:	vpxor  %xmm8,%xmm8,%xmm8
  12c865:	vpxor  %xmm7,%xmm7,%xmm7
  12c869:	vpxor  %xmm6,%xmm6,%xmm6
  12c86d:	vpxor  %xmm5,%xmm5,%xmm5
  12c871:	vpxor  %xmm4,%xmm4,%xmm4
  12c875:	vpxor  %xmm3,%xmm3,%xmm3
  12c879:	vpxor  %xmm1,%xmm1,%xmm1
  12c87d:	vpxor  %xmm0,%xmm0,%xmm0
  12c881:	mov    0x90(%rsp),%r9
  12c889:	mov    0x20(%rsp),%r10
  12c88e:	xchg   %ax,%ax
  12c890:	mov    %rsi,%r8
  12c893:	vmovdqu64 -0xc0(%rcx,%rdx,4),%zmm11
  12c89b:	vmovdqu64 -0x80(%rcx,%rdx,4),%zmm12
  12c8a3:	vmovdqu64 -0x40(%rcx,%rdx,4),%zmm13
  12c8ab:	vmovdqu64 (%rcx,%rdx,4),%zmm14
  12c8b2:	vshufi64x2 $0x88,%zmm12,%zmm11,%zmm15
  12c8b9:	vshufi64x2 $0xdd,%zmm12,%zmm11,%zmm11
  12c8c0:	vpxorq %zmm15,%zmm11,%zmm11
  12c8c6:	vpclmulhqlqdq %zmm11,%zmm9,%zmm12
  12c8cd:	vpclmullqhqdq %zmm11,%zmm9,%zmm16
  12c8d4:	vpxorq %zmm12,%zmm16,%zmm12
  12c8da:	vpclmulhqhqdq %zmm11,%zmm9,%zmm16
  12c8e1:	vpslldq $0x8,%zmm16,%zmm17
  12c8e8:	vpclmulhqlqdq %zmm10,%zmm16,%zmm16
  12c8ef:	vpternlogq $0x96,%zmm17,%zmm12,%zmm16
  12c8f6:	vpclmullqlqdq %zmm11,%zmm9,%zmm11
  12c8fd:	vpslldq $0x8,%zmm16,%zmm12
  12c904:	vpclmulhqlqdq %zmm10,%zmm16,%zmm16
  12c90b:	vpxorq %zmm16,%zmm11,%zmm11
  12c911:	vpternlogq $0x96,%zmm12,%zmm15,%zmm11
  12c918:	vshufi64x2 $0x88,%zmm14,%zmm13,%zmm12
  12c91f:	vshufi64x2 $0xdd,%zmm14,%zmm13,%zmm13
  12c926:	vpxorq %zmm12,%zmm13,%zmm13
  12c92c:	vpclmulhqlqdq %zmm13,%zmm9,%zmm14
  12c933:	vpclmullqhqdq %zmm13,%zmm9,%zmm15
  12c93a:	vpxorq %zmm14,%zmm15,%zmm14
  12c940:	vpclmulhqhqdq %zmm13,%zmm9,%zmm15
  12c947:	vpslldq $0x8,%zmm15,%zmm16
  12c94e:	vpclmulhqlqdq %zmm10,%zmm15,%zmm15
  12c955:	vpternlogq $0x96,%zmm16,%zmm14,%zmm15
  12c95c:	vpclmullqlqdq %zmm13,%zmm9,%zmm13
  12c963:	vpslldq $0x8,%zmm15,%zmm14
  12c96a:	vpclmulhqlqdq %zmm10,%zmm15,%zmm15
  12c971:	vpxorq %zmm15,%zmm13,%zmm13
  12c977:	vpternlogq $0x96,%zmm14,%zmm12,%zmm13
  12c97e:	vmovdqu64 -0xc0(%rax,%rdx,4),%zmm12
  12c986:	vmovdqu64 -0x80(%rax,%rdx,4),%zmm14
  12c98e:	vmovdqu64 -0x40(%rax,%rdx,4),%zmm15
  12c996:	vmovdqu64 (%rax,%rdx,4),%zmm16
  12c99d:	vshufi64x2 $0x88,%zmm14,%zmm12,%zmm17
  12c9a4:	vshufi64x2 $0xdd,%zmm14,%zmm12,%zmm12
  12c9ab:	vpxorq %zmm17,%zmm12,%zmm12
  12c9b1:	vpclmulhqlqdq %zmm12,%zmm9,%zmm14
  12c9b8:	vpclmullqhqdq %zmm12,%zmm9,%zmm18
  12c9bf:	vpxorq %zmm14,%zmm18,%zmm14
  12c9c5:	vpclmulhqhqdq %zmm12,%zmm9,%zmm18
  12c9cc:	vpslldq $0x8,%zmm18,%zmm19
  12c9d3:	vpclmulhqlqdq %zmm10,%zmm18,%zmm18
  12c9da:	vpternlogq $0x96,%zmm19,%zmm14,%zmm18
  12c9e1:	vpclmullqlqdq %zmm12,%zmm9,%zmm12
  12c9e8:	vpslldq $0x8,%zmm18,%zmm14
  12c9ef:	vpclmulhqlqdq %zmm10,%zmm18,%zmm18
  12c9f6:	vpxorq %zmm18,%zmm12,%zmm12
  12c9fc:	vpternlogq $0x96,%zmm14,%zmm17,%zmm12
  12ca03:	vshufi64x2 $0x88,%zmm16,%zmm15,%zmm14
  12ca0a:	vshufi64x2 $0xdd,%zmm16,%zmm15,%zmm15
  12ca11:	vpxorq %zmm14,%zmm15,%zmm15
  12ca17:	vpclmulhqlqdq %zmm15,%zmm9,%zmm16
  12ca1e:	vpclmullqhqdq %zmm15,%zmm9,%zmm17
  12ca25:	vpxorq %zmm16,%zmm17,%zmm16
  12ca2b:	vpclmulhqhqdq %zmm15,%zmm9,%zmm17
  12ca32:	vpslldq $0x8,%zmm17,%zmm18
  12ca39:	vpclmulhqlqdq %zmm10,%zmm17,%zmm17
  12ca40:	vpternlogq $0x96,%zmm18,%zmm16,%zmm17
  12ca47:	vpclmullqlqdq %zmm15,%zmm9,%zmm15
  12ca4e:	vpslldq $0x8,%zmm17,%zmm16
  12ca55:	vpclmulhqlqdq %zmm10,%zmm17,%zmm17
  12ca5c:	vpxorq %zmm17,%zmm15,%zmm15
  12ca62:	vpternlogq $0x96,%zmm16,%zmm14,%zmm15
  12ca69:	vmovdqu64 %zmm11,(%r10,%rdx,2)
  12ca70:	vmovdqu64 %zmm13,0x40(%r10,%rdx,2)
  12ca78:	vmovdqu64 %zmm12,(%r9,%rdx,2)
  12ca7f:	vmovdqu64 %zmm15,0x40(%r9,%rdx,2)
  12ca87:	vshufi64x2 $0x88,%zmm13,%zmm11,%zmm14
  12ca8e:	vshufi64x2 $0xdd,%zmm13,%zmm11,%zmm11
  12ca95:	vshufi64x2 $0x88,%zmm15,%zmm12,%zmm13
  12ca9c:	vshufi64x2 $0xdd,%zmm15,%zmm12,%zmm12
  12caa3:	vmovdqu64 (%r15,%rdx,1),%zmm15
  12caaa:	vpclmulhqlqdq %zmm13,%zmm14,%zmm16
  12cab1:	vpclmullqhqdq %zmm13,%zmm14,%zmm17
  12cab8:	vpxorq %zmm16,%zmm17,%zmm16
  12cabe:	vpclmulhqhqdq %zmm13,%zmm14,%zmm17
  12cac5:	vpslldq $0x8,%zmm17,%zmm18
  12cacc:	vpclmulhqlqdq %zmm10,%zmm17,%zmm17
  12cad3:	vpternlogq $0x96,%zmm18,%zmm16,%zmm17
  12cada:	vpclmullqlqdq %zmm13,%zmm14,%zmm16
  12cae1:	vpslldq $0x8,%zmm17,%zmm18
  12cae8:	vpclmulhqlqdq %zmm10,%zmm17,%zmm17
  12caef:	vpternlogq $0x96,%zmm16,%zmm18,%zmm17
  12caf6:	vpclmullqlqdq %zmm15,%zmm17,%zmm16
  12cafd:	vpxorq %zmm2,%zmm16,%zmm2
  12cb03:	vpclmulhqhqdq %zmm15,%zmm17,%zmm16
  12cb0a:	vpxorq %zmm8,%zmm16,%zmm8
  12cb10:	vpclmulhqlqdq %zmm15,%zmm17,%zmm16
  12cb17:	vpclmullqhqdq %zmm15,%zmm17,%zmm17
  12cb1e:	vpternlogq $0x96,%zmm17,%zmm16,%zmm7
  12cb25:	vpclmulhqlqdq %zmm12,%zmm11,%zmm16
  12cb2c:	vpclmullqhqdq %zmm12,%zmm11,%zmm17
  12cb33:	vpxorq %zmm16,%zmm17,%zmm16
  12cb39:	vpclmulhqhqdq %zmm12,%zmm11,%zmm17
  12cb40:	vpslldq $0x8,%zmm17,%zmm18
  12cb47:	vpclmulhqlqdq %zmm10,%zmm17,%zmm17
  12cb4e:	vpternlogq $0x96,%zmm18,%zmm16,%zmm17
  12cb55:	vpclmullqlqdq %zmm12,%zmm11,%zmm16
  12cb5c:	vpslldq $0x8,%zmm17,%zmm18
  12cb63:	vpclmulhqlqdq %zmm10,%zmm17,%zmm17
  12cb6a:	vpternlogq $0x96,%zmm16,%zmm18,%zmm17
  12cb71:	vpclmullqlqdq %zmm15,%zmm17,%zmm16
  12cb78:	vpxorq %zmm6,%zmm16,%zmm6
  12cb7e:	vpclmulhqhqdq %zmm15,%zmm17,%zmm16
  12cb85:	vpxorq %zmm5,%zmm16,%zmm5
  12cb8b:	vpclmulhqlqdq %zmm15,%zmm17,%zmm16
  12cb92:	vpclmullqhqdq %zmm15,%zmm17,%zmm17
  12cb99:	vpternlogq $0x96,%zmm17,%zmm16,%zmm4
  12cba0:	vpxorq %zmm14,%zmm11,%zmm11
  12cba6:	vpxorq %zmm13,%zmm12,%zmm12
  12cbac:	vpclmulhqlqdq %zmm12,%zmm11,%zmm13
  12cbb3:	vpclmullqhqdq %zmm12,%zmm11,%zmm14
  12cbba:	vpxorq %zmm13,%zmm14,%zmm13
  12cbc0:	vpclmulhqhqdq %zmm12,%zmm11,%zmm14
  12cbc7:	vpslldq $0x8,%zmm14,%zmm16
  12cbce:	vpclmulhqlqdq %zmm10,%zmm14,%zmm14
  12cbd5:	vpternlogq $0x96,%zmm16,%zmm13,%zmm14
  12cbdc:	vpclmullqlqdq %zmm12,%zmm11,%zmm11
  12cbe3:	vpslldq $0x8,%zmm14,%zmm12
  12cbea:	vpclmulhqlqdq %zmm10,%zmm14,%zmm13
  12cbf1:	vpternlogq $0x96,%zmm11,%zmm12,%zmm13
  12cbf8:	vpclmullqlqdq %zmm15,%zmm13,%zmm11
  12cbff:	vpxorq %zmm3,%zmm11,%zmm3
  12cc05:	vpclmulhqhqdq %zmm15,%zmm13,%zmm11
  12cc0c:	vpxorq %zmm1,%zmm11,%zmm1
  12cc12:	vpclmulhqlqdq %zmm15,%zmm13,%zmm11
  12cc19:	vpclmullqhqdq %zmm15,%zmm13,%zmm12
  12cc20:	vpternlogq $0x96,%zmm12,%zmm11,%zmm0
  12cc27:	add    $0x4,%rsi
  12cc2b:	add    $0x8,%r8
  12cc2f:	add    $0x40,%rdx
  12cc33:	cmp    %rbx,%r8
  12cc36:	jbe    12c890 <mikros_permutation_binary_v5::fast_sumcheck::native::block+0x2a0>
  12cc3c:	vmovaps 0x3c0(%rsp),%ymm9
  12cc45:	vmovaps %ymm9,0x140(%rsp)
  12cc4e:	vmovaps 0x3e0(%rsp),%ymm9
  12cc57:	vmovaps %ymm9,0x120(%rsp)
  12cc60:	vmovaps 0x400(%rsp),%xmm9
  12cc69:	vmovaps 0x410(%rsp),%xmm10
  12cc72:	mov    0x18(%rsp),%rax
  12cc77:	add    %rax,%rax
  12cc7a:	mov    %rax,0xe0(%rsp)
  12cc82:	add    %rdi,%r13
  12cc85:	lea    (%r11,%r11,1),%rax
  12cc89:	mov    %rax,0xd8(%rsp)
  12cc91:	vmovdqa64 %zmm2,0x180(%rsp)
  12cc99:	vmovdqa64 %zmm8,0x1c0(%rsp)
  12cca1:	vmovdqa64 %zmm7,0x200(%rsp)
  12cca9:	vmovdqa64 %zmm6,0x240(%rsp)
  12ccb1:	vmovdqa64 %zmm5,0x280(%rsp)
  12ccb9:	vmovdqa64 %zmm4,0x2c0(%rsp)
  12ccc1:	vmovdqa64 %zmm3,0x300(%rsp)
  12ccc9:	vmovdqa64 %zmm1,0x340(%rsp)
  12ccd1:	add    %rdi,0xe8(%rsp)
  12ccd9:	vmovdqa64 %zmm0,0x380(%rsp)
  12cce1:	shl    $0x4,%rbx
  12cce5:	add    %r15,%rbx
  12cce8:	vmovaps 0x410(%rsp),%xmm0
  12ccf1:	movq   $0x0,0x28(%rsp)
  12ccfa:	mov    %rbx,0x110(%rsp)
  12cd02:	mov    %r13,0x108(%rsp)
  12cd0a:	test   %rsi,%rsi
  12cd0d:	jne    12d1dc <mikros_permutation_binary_v5::fast_sumcheck::native::block+0xbec>
  12cd13:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  12cd20:	cmp    %rbx,%r15
  12cd23:	je     12d1ff <mikros_permutation_binary_v5::fast_sumcheck::native::block+0xc0f>
  12cd29:	mov    0x28(%rsp),%rax
  12cd2e:	lea    0x0(,%rax,4),%r14
  12cd36:	lea    0x4(,%rax,4),%rsi
  12cd3e:	lea    0x3(,%rax,4),%rax
  12cd46:	cmp    0xe0(%rsp),%rax
  12cd4e:	jae    12d701 <mikros_permutation_binary_v5::fast_sumcheck::native::block+0x1111>
  12cd54:	cmp    0xd8(%rsp),%rsi
  12cd5c:	ja     12d725 <mikros_permutation_binary_v5::fast_sumcheck::native::block+0x1135>
  12cd62:	vmovaps %xmm0,0xc0(%rsp)
  12cd6b:	vmovaps %xmm10,0x160(%rsp)
  12cd74:	vmovaps %xmm9,0x170(%rsp)
  12cd7d:	mov    %r15,0x38(%rsp)
  12cd82:	shl    $0x4,%r14
  12cd86:	mov    0x0(%r13,%r14,1),%rbx
  12cd8b:	mov    0x8(%r13,%r14,1),%r15
  12cd90:	vmovaps 0x10(%r13,%r14,1),%xmm0
  12cd97:	vxorps 0x0(%r13,%r14,1),%xmm0,%xmm0
  12cd9e:	vmovaps %xmm0,0x440(%rsp)
  12cda7:	lea    0x60(%rsp),%rdi
  12cdac:	mov    %r12,%rsi
  12cdaf:	lea    0x440(%rsp),%rdx
  12cdb7:	mov    0xae282(%rip),%rax        # 1db040 <_DYNAMIC+0x5f8>
  12cdbe:	vzeroupper 
  12cdc1:	call   *%rax
  12cdc3:	xor    0x60(%rsp),%rbx
  12cdc8:	mov    %rbx,0xb0(%rsp)
  12cdd0:	xor    0x68(%rsp),%r15
  12cdd5:	mov    %r13,%rax
  12cdd8:	mov    0x20(%r13,%r14,1),%rbx
  12cddd:	mov    0x28(%r13,%r14,1),%r13
  12cde2:	vmovaps 0x30(%rax,%r14,1),%xmm0
  12cde9:	vxorps 0x20(%rax,%r14,1),%xmm0,%xmm0
  12cdf0:	vmovaps %xmm0,0x440(%rsp)
  12cdf9:	lea    0x60(%rsp),%rdi
  12cdfe:	mov    %r12,%rsi
  12ce01:	lea    0x440(%rsp),%rdx
  12ce09:	call   *0xae231(%rip)        # 1db040 <_DYNAMIC+0x5f8>
  12ce0f:	xor    0x60(%rsp),%rbx
  12ce14:	mov    %rbx,0xa8(%rsp)
  12ce1c:	xor    0x68(%rsp),%r13
  12ce21:	mov    %r13,0xa0(%rsp)
  12ce29:	mov    0xe8(%rsp),%rbx
  12ce31:	mov    %r12,%rsi
  12ce34:	mov    (%rbx,%r14,1),%r12
  12ce38:	mov    0x8(%rbx,%r14,1),%r13
  12ce3d:	vmovaps 0x10(%rbx,%r14,1),%xmm0
  12ce44:	vxorps (%rbx,%r14,1),%xmm0,%xmm0
  12ce4a:	vmovaps %xmm0,0x440(%rsp)
  12ce53:	lea    0x60(%rsp),%rdi
  12ce58:	lea    0x440(%rsp),%rdx
  12ce60:	call   *0xae1da(%rip)        # 1db040 <_DYNAMIC+0x5f8>
  12ce66:	xor    0x60(%rsp),%r12
  12ce6b:	mov    %r12,0x98(%rsp)
  12ce73:	xor    0x68(%rsp),%r13
  12ce78:	mov    %r13,0x30(%rsp)
  12ce7d:	mov    0x20(%rbx,%r14,1),%r12
  12ce82:	mov    0x28(%rbx,%r14,1),%r13
  12ce87:	vmovaps 0x30(%rbx,%r14,1),%xmm0
  12ce8e:	vxorps 0x20(%rbx,%r14,1),%xmm0,%xmm0
  12ce95:	vmovaps %xmm0,0x440(%rsp)
  12ce9e:	lea    0x60(%rsp),%rdi
  12cea3:	mov    0x18(%rbp),%rsi
  12cea7:	lea    0x440(%rsp),%rdx
  12ceaf:	call   *0xae18b(%rip)        # 1db040 <_DYNAMIC+0x5f8>
  12ceb5:	mov    0x28(%rsp),%rax
  12ceba:	lea    (%rax,%rax,1),%rdi
  12cebe:	mov    0x18(%rsp),%rsi
  12cec3:	cmp    %rsi,%rdi
  12cec6:	jae    12d7b9 <mikros_permutation_binary_v5::fast_sumcheck::native::block+0x11c9>
  12cecc:	xor    0x60(%rsp),%r12
  12ced1:	xor    0x68(%rsp),%r13
  12ced6:	mov    %rdi,%rcx
  12ced9:	shl    $0x4,%rcx
  12cedd:	mov    0x20(%rsp),%r8
  12cee2:	mov    0xb0(%rsp),%r10
  12ceea:	mov    %r10,(%r8,%rcx,1)
  12ceee:	mov    %r15,0x8(%r8,%rcx,1)
  12cef3:	mov    %rdi,%rax
  12cef6:	or     $0x1,%rax
  12cefa:	cmp    %rsi,%rax
  12cefd:	mov    0x90(%rsp),%r9
  12cf05:	mov    0x10(%rbp),%rsi
  12cf09:	jae    12d792 <mikros_permutation_binary_v5::fast_sumcheck::native::block+0x11a2>
  12cf0f:	mov    %rax,%rdx
  12cf12:	shl    $0x4,%rdx
  12cf16:	mov    0xa8(%rsp),%r11
  12cf1e:	mov    %r11,(%r8,%rdx,1)
  12cf22:	mov    0xa0(%rsp),%r11
  12cf2a:	mov    %r11,0x8(%r8,%rdx,1)
  12cf2f:	cmp    %rsi,%rdi
  12cf32:	jae    12d773 <mikros_permutation_binary_v5::fast_sumcheck::native::block+0x1183>
  12cf38:	mov    0x98(%rsp),%r8
  12cf40:	mov    %r8,(%r9,%rcx,1)
  12cf44:	mov    0x30(%rsp),%rdi
  12cf49:	mov    %rdi,0x8(%r9,%rcx,1)
  12cf4e:	cmp    %rsi,%rax
  12cf51:	jae    12d7d8 <mikros_permutation_binary_v5::fast_sumcheck::native::block+0x11e8>
  12cf57:	mov    0x38(%rsp),%rax
  12cf5c:	add    $0x10,%rax
  12cf60:	mov    %rax,0xc0(%rsp)
  12cf68:	incq   0x28(%rsp)
  12cf6d:	mov    %r12,(%r9,%rdx,1)
  12cf71:	mov    %r13,0x8(%r9,%rdx,1)
  12cf76:	mov    %r10,0x40(%rsp)
  12cf7b:	mov    %r15,0x48(%rsp)
  12cf80:	mov    %r8,0x440(%rsp)
  12cf88:	mov    %rdi,0x448(%rsp)
  12cf90:	lea    0x60(%rsp),%rdi
  12cf95:	lea    0x40(%rsp),%rsi
  12cf9a:	lea    0x440(%rsp),%rdx
  12cfa2:	mov    0xae097(%rip),%rax        # 1db040 <_DYNAMIC+0x5f8>
  12cfa9:	mov    %r15,0x118(%rsp)
  12cfb1:	call   *%rax
  12cfb3:	mov    0xa8(%rsp),%r14
  12cfbb:	mov    %r14,0x40(%rsp)
  12cfc0:	mov    0xa0(%rsp),%r15
  12cfc8:	mov    %r15,0x48(%rsp)
  12cfcd:	mov    %r12,0x440(%rsp)
  12cfd5:	mov    %r13,0x448(%rsp)
  12cfdd:	lea    0x70(%rsp),%rdi
  12cfe2:	lea    0x40(%rsp),%rbx
  12cfe7:	mov    %rbx,%rsi
  12cfea:	lea    0x440(%rsp),%rdx
  12cff2:	call   *0xae048(%rip)        # 1db040 <_DYNAMIC+0x5f8>
  12cff8:	xor    0xb0(%rsp),%r14
  12d000:	xor    0x118(%rsp),%r15
  12d008:	mov    %r14,0x40(%rsp)
  12d00d:	mov    %r15,0x48(%rsp)
  12d012:	xor    0x98(%rsp),%r12
  12d01a:	xor    0x30(%rsp),%r13
  12d01f:	mov    %r12,0x440(%rsp)
  12d027:	mov    %r13,0x448(%rsp)
  12d02f:	lea    0x80(%rsp),%rdi
  12d037:	mov    %rbx,%rsi
  12d03a:	lea    0x440(%rsp),%rdx
  12d042:	call   *0xadff8(%rip)        # 1db040 <_DYNAMIC+0x5f8>
  12d048:	vmovups 0x60(%rsp),%ymm0
  12d04e:	vmovdqu 0x70(%rsp),%ymm1
  12d054:	vmovdqu %ymm1,0x450(%rsp)
  12d05d:	vmovaps %ymm0,0x440(%rsp)
  12d066:	vmovaps 0x440(%rsp),%xmm0
  12d06f:	lea    0x70(%rsp),%rax
  12d074:	vmovups %xmm0,-0x8(%rax)
  12d079:	vmovups 0x60(%rsp),%xmm0
  12d07f:	vmovaps %xmm0,0x40(%rsp)
  12d085:	mov    0x70(%rsp),%rax
  12d08a:	mov    %rax,0x50(%rsp)
  12d08f:	lea    0x48(%rsp),%r15
  12d094:	vmovups (%r15),%xmm0
  12d099:	vmovaps %xmm0,0xf0(%rsp)
  12d0a2:	lea    0x60(%rsp),%r15
  12d0a7:	mov    %r15,%rdi
  12d0aa:	lea    0xf0(%rsp),%rsi
  12d0b2:	mov    0x38(%rsp),%r14
  12d0b7:	mov    %r14,%rdx
  12d0ba:	mov    0xade4f(%rip),%r13        # 1daf10 <_DYNAMIC+0x4c8>
  12d0c1:	vzeroupper 
  12d0c4:	call   *%r13
  12d0c7:	vmovaps 0x140(%rsp),%ymm0
  12d0d0:	vxorps 0x60(%rsp),%ymm0,%ymm0
  12d0d6:	vmovaps %ymm0,0x140(%rsp)
  12d0df:	lea    0x450(%rsp),%r12
  12d0e7:	vmovups (%r12),%xmm0
  12d0ed:	lea    0x70(%rsp),%rbx
  12d0f2:	vmovups %xmm0,-0x8(%rbx)
  12d0f7:	mov    0x70(%rsp),%rax
  12d0fc:	mov    %rax,0x50(%rsp)
  12d101:	vmovups 0x60(%rsp),%xmm0
  12d107:	vmovaps %xmm0,0x40(%rsp)
  12d10d:	lea    0x48(%rsp),%rax
  12d112:	vmovups (%rax),%xmm0
  12d116:	vmovaps %xmm0,0xf0(%rsp)
  12d11f:	mov    %r15,%rdi
  12d122:	lea    0xf0(%rsp),%r15
  12d12a:	mov    %r15,%rsi
  12d12d:	mov    %r14,%rdx
  12d130:	vzeroupper 
  12d133:	call   *%r13
  12d136:	vmovaps 0x120(%rsp),%ymm0
  12d13f:	vxorps 0x60(%rsp),%ymm0,%ymm0
  12d145:	vmovaps %ymm0,0x120(%rsp)
  12d14e:	vmovups 0x10(%r12),%xmm0
  12d155:	vmovups %xmm0,-0x8(%rbx)
  12d15a:	mov    0x70(%rsp),%rax
  12d15f:	mov    %rax,0x50(%rsp)
  12d164:	vmovups 0x60(%rsp),%xmm0
  12d16a:	vmovaps %xmm0,0x40(%rsp)
  12d170:	lea    0x48(%rsp),%rax
  12d175:	vmovups (%rax),%xmm0
  12d179:	vmovaps %xmm0,0xf0(%rsp)
  12d182:	lea    0x60(%rsp),%rdi
  12d187:	mov    %r15,%rsi
  12d18a:	mov    %r14,%rdx
  12d18d:	vzeroupper 
  12d190:	call   *%r13
  12d193:	vmovaps 0x170(%rsp),%xmm9
  12d19c:	vxorps 0x60(%rsp),%xmm9,%xmm9
  12d1a2:	vmovaps 0x160(%rsp),%xmm0
  12d1ab:	vxorps 0x70(%rsp),%xmm0,%xmm0
  12d1b1:	mov    0xc0(%rsp),%r15
  12d1b9:	xor    %esi,%esi
  12d1bb:	vmovaps %xmm0,%xmm10
  12d1bf:	mov    0x18(%rbp),%r12
  12d1c3:	mov    0x110(%rsp),%rbx
  12d1cb:	mov    0x108(%rsp),%r13
  12d1d3:	test   %rsi,%rsi
  12d1d6:	je     12cd20 <mikros_permutation_binary_v5::fast_sumcheck::native::block+0x730>
  12d1dc:	mov    %rbx,%rax
  12d1df:	sub    %r15,%rax
  12d1e2:	shr    $0x4,%rax
  12d1e6:	cmp    %rax,%rsi
  12d1e9:	jae    12d1ff <mikros_permutation_binary_v5::fast_sumcheck::native::block+0xc0f>
  12d1eb:	mov    %rsi,%rax
  12d1ee:	shl    $0x4,%rax
  12d1f2:	add    %rax,%r15
  12d1f5:	add    %rsi,0x28(%rsp)
  12d1fa:	jmp    12cd29 <mikros_permutation_binary_v5::fast_sumcheck::native::block+0x739>
  12d1ff:	vmovaps %xmm0,0x410(%rsp)
  12d208:	vmovaps %xmm9,0x400(%rsp)
  12d211:	vmovaps 0x140(%rsp),%ymm0
  12d21a:	vmovaps %ymm0,0x3c0(%rsp)
  12d223:	vmovaps 0x120(%rsp),%ymm0
  12d22c:	vmovaps %ymm0,0x3e0(%rsp)
  12d235:	jmp    12d39a <mikros_permutation_binary_v5::fast_sumcheck::native::block+0xdaa>
  12d23a:	mov    0x18(%rsp),%r10
  12d23f:	test   %rbx,%rbx
  12d242:	mov    0x90(%rsp),%r8
  12d24a:	mov    0x20(%rsp),%rcx
  12d24f:	mov    0x30(%rsp),%rsi
  12d254:	je     12d39a <mikros_permutation_binary_v5::fast_sumcheck::native::block+0xdaa>
  12d25a:	lea    0x7f(%rbx),%r11
  12d25e:	shr    $0x7,%r11
  12d262:	cmp    %r13,%r11
  12d265:	cmovae %r13,%r11
  12d269:	test   %r11,%r11
  12d26c:	je     12d39a <mikros_permutation_binary_v5::fast_sumcheck::native::block+0xdaa>
  12d272:	mov    %r15,%rdi
  12d275:	mov    %r14,%rdx
  12d278:	shl    $0x2,%rdx
  12d27c:	mov    %r10,%r14
  12d27f:	mov    %r9,%r15
  12d282:	mov    %rbx,%r12
  12d285:	data16 cs nopw 0x0(%rax,%rax,1)
  12d290:	mov    %r11,0xb0(%rsp)
  12d298:	mov    %rdx,0x28(%rsp)
  12d29d:	mov    %rdi,0x38(%rsp)
  12d2a2:	mov    %rcx,0x20(%rsp)
  12d2a7:	sub    $0x100,%r14
  12d2ae:	mov    %r14,0xa8(%rsp)
  12d2b6:	mov    $0x100,%eax
  12d2bb:	cmovae %rax,%r10
  12d2bf:	mov    %r10,0x18(%rsp)
  12d2c4:	sub    $0x100,%r15
  12d2cb:	cmovae %rax,%r9
  12d2cf:	mov    %r9,0xa0(%rsp)
  12d2d7:	sub    $0x80,%r12
  12d2de:	mov    $0x80,%eax
  12d2e3:	cmovae %rax,%rbx
  12d2e7:	lea    0x0(,%r10,4),%r14d
  12d2ef:	mov    0x98(%rsp),%rdi
  12d2f7:	lea    0x440(%rsp),%rcx
  12d2ff:	mov    %r8,%r13
  12d302:	mov    %r14,%r8
  12d305:	call   *0xae04d(%rip)        # 1db358 <_DYNAMIC+0x910>
  12d30b:	lea    0x440(%rsp),%rdi
  12d313:	mov    %r14,%rsi
  12d316:	mov    0xa8(%rsp),%r14
  12d31e:	lea    0x180(%rsp),%rax
  12d326:	mov    0x20(%rsp),%rdx
  12d32b:	mov    0x18(%rsp),%rcx
  12d330:	mov    %r13,%r8
  12d333:	mov    0xa0(%rsp),%r9
  12d33b:	push   %rax
  12d33c:	push   %rbx
  12d33d:	push   0x48(%rsp)
  12d341:	push   0x18(%rbp)
  12d344:	call   12b350 <mikros_permutation_binary_v5::fast_sumcheck::native::dense::<true>>
  12d349:	mov    0xd0(%rsp),%r11
  12d351:	mov    0x50(%rsp),%rsi
  12d356:	mov    0x48(%rsp),%rdx
  12d35b:	mov    0x58(%rsp),%rdi
  12d360:	mov    0x40(%rsp),%rcx
  12d365:	mov    %r13,%r8
  12d368:	add    $0x20,%rsp
  12d36c:	add    $0x1000,%rcx
  12d373:	add    $0x1000,%r8
  12d37a:	add    $0x800,%rdi
  12d381:	add    $0x400,%rdx
  12d388:	mov    %r12,%rbx
  12d38b:	mov    %r15,%r9
  12d38e:	mov    %r14,%r10
  12d391:	dec    %r11
  12d394:	jne    12d290 <mikros_permutation_binary_v5::fast_sumcheck::native::block+0xca0>
  12d39a:	mov    0xb8(%rsp),%rax
  12d3a2:	vmovdqa 0x190(%rsp),%xmm1
  12d3ab:	vmovdqa 0x1b0(%rsp),%xmm0
  12d3b4:	vmovdqa 0x1d0(%rsp),%xmm2
  12d3bd:	vmovdqa 0x1f0(%rsp),%xmm3
  12d3c6:	vpxor  0x1a0(%rsp),%xmm0,%xmm0
  12d3cf:	vpternlogd $0x96,0x180(%rsp),%xmm1,%xmm0
  12d3d8:	vpxor  0x1e0(%rsp),%xmm3,%xmm4
  12d3e1:	vpternlogd $0x96,0x1c0(%rsp),%xmm2,%xmm4
  12d3ea:	vmovdqa 0x210(%rsp),%xmm1
  12d3f3:	vmovdqa 0x230(%rsp),%xmm2
  12d3fc:	vpxor  0x220(%rsp),%xmm2,%xmm2
  12d405:	vpternlogd $0x96,0x200(%rsp),%xmm1,%xmm2
  12d40e:	vmovdqa 0x250(%rsp),%xmm3
  12d417:	vmovdqa 0x270(%rsp),%xmm1
  12d420:	vpxor  0x260(%rsp),%xmm1,%xmm1
  12d429:	vpternlogd $0x96,0x240(%rsp),%xmm3,%xmm1
  12d432:	vmovdqa 0x290(%rsp),%xmm3
  12d43b:	vmovdqa 0x2b0(%rsp),%xmm5
  12d444:	vpxor  0x2a0(%rsp),%xmm5,%xmm5
  12d44d:	vpternlogd $0x96,0x280(%rsp),%xmm3,%xmm5
  12d456:	vmovdqa 0x2d0(%rsp),%xmm6
  12d45f:	vmovdqa 0x2f0(%rsp),%xmm3
  12d468:	vpxor  0x2e0(%rsp),%xmm3,%xmm3
  12d471:	vpternlogd $0x96,0x2c0(%rsp),%xmm6,%xmm3
  12d47a:	vmovdqa 0x310(%rsp),%xmm6
  12d483:	vmovdqa 0x330(%rsp),%xmm7
  12d48c:	vpxor  0x320(%rsp),%xmm7,%xmm7
  12d495:	vpternlogd $0x96,0x300(%rsp),%xmm6,%xmm7
  12d49e:	vmovdqa 0x350(%rsp),%xmm6
  12d4a7:	vmovdqa 0x370(%rsp),%xmm8
  12d4b0:	vpxor  0x360(%rsp),%xmm8,%xmm8
  12d4b9:	vpternlogd $0x96,0x340(%rsp),%xmm6,%xmm8
  12d4c2:	vmovdqa 0x390(%rsp),%xmm6
  12d4cb:	vmovdqa 0x3b0(%rsp),%xmm9
  12d4d4:	vpxor  0x3a0(%rsp),%xmm9,%xmm9
  12d4dd:	vpternlogd $0x96,0x380(%rsp),%xmm6,%xmm9
  12d4e6:	vmovq  %xmm7,%rcx
  12d4eb:	vpextrq $0x1,%xmm7,%r14
  12d4f1:	vmovq  %xmm8,%rdx
  12d4f6:	vpextrq $0x1,%xmm8,%rsi
  12d4fc:	vmovq  %xmm9,%r9
  12d501:	vpextrq $0x1,%xmm9,%rdi
  12d507:	xor    0x400(%rsp),%rcx
  12d50f:	xor    0x410(%rsp),%rdx
  12d517:	xor    0x418(%rsp),%rsi
  12d51f:	xor    %rdi,%rdx
  12d522:	lea    (%rdx,%rdx,1),%rdi
  12d526:	mov    %rsi,%r11
  12d529:	shld   $0x1,%rdx,%r11
  12d52e:	lea    0x0(,%rdx,4),%r8
  12d536:	xor    %rcx,%r8
  12d539:	mov    %rsi,%r10
  12d53c:	shld   $0x2,%rdx,%r10
  12d541:	mov    %rsi,%rbx
  12d544:	shld   $0x7,%rdx,%rbx
  12d549:	xor    %r11,%rbx
  12d54c:	mov    %rsi,%r11
  12d54f:	mov    %rsi,%rcx
  12d552:	xor    0x408(%rsp),%r14
  12d55a:	xor    %r9,%r14
  12d55d:	xor    %r10,%r14
  12d560:	xor    %rbx,%r14
  12d563:	xor    %rsi,%r14
  12d566:	shr    $0x3f,%rsi
  12d56a:	shr    $0x3e,%r11
  12d56e:	xor    %rsi,%r11
  12d571:	shr    $0x39,%rcx
  12d575:	xor    %r11,%rcx
  12d578:	lea    (%rcx,%rcx,1),%esi
  12d57b:	lea    0x0(,%rcx,4),%r9d
  12d583:	mov    %ecx,%r10d
  12d586:	xor    %rdx,%rcx
  12d589:	shl    $0x7,%rdx
  12d58d:	xor    %rdi,%rdx
  12d590:	xor    %r8,%rdx
  12d593:	xor    %rdx,%r9
  12d596:	shl    $0x7,%r10d
  12d59a:	xor    %rsi,%r10
  12d59d:	xor    %r9,%r10
  12d5a0:	xor    %r10,%rcx
  12d5a3:	vmovdqa -0xe030b(%rip),%ymm6        # 4d2a0 <anon.25c1abbc0d37fd1a879b913e2262f7b9.7.llvm.8827852567942549522+0x20>
  12d5ab:	vpermq 0x3c0(%rsp),%zmm6,%zmm7
  12d5b3:	vmovdqa 0x3f0(%rsp),%xmm8
  12d5bc:	vpunpcklqdq %xmm1,%xmm0,%xmm9
  12d5c0:	vpxor  %xmm7,%xmm9,%xmm9
  12d5c4:	vpunpcklqdq %xmm5,%xmm4,%xmm10
  12d5c8:	vpalignr $0x8,0x3c8(%rsp),%xmm8,%xmm11
  12d5d3:	vpunpckhqdq %xmm3,%xmm2,%xmm7
  12d5d7:	vpternlogq $0x96,%xmm10,%xmm11,%xmm7
  12d5de:	vpunpckhqdq %xmm5,%xmm4,%xmm4
  12d5e2:	vpblendd $0x3,0x3d8(%rsp),%xmm8,%xmm5
  12d5ed:	vpxor  %xmm4,%xmm5,%xmm4
  12d5f1:	vpaddq %xmm7,%xmm7,%xmm5
  12d5f5:	vpshldq $0x1,%xmm7,%xmm4,%xmm8
  12d5fc:	vpsllq $0x2,%xmm7,%xmm10
  12d601:	vpternlogq $0x96,%xmm5,%xmm9,%xmm10
  12d608:	vpshldq $0x2,%xmm7,%xmm4,%xmm5
  12d60f:	vmovq  0x3e8(%rsp),%xmm9
  12d618:	vpalignr $0x8,%xmm7,%xmm9,%xmm9
  12d61e:	vmovq  0x3c8(%rsp),%xmm11
  12d627:	vpunpcklqdq %xmm11,%xmm7,%xmm11
  12d62c:	vbroadcasti128 -0xdfa65(%rip),%ymm12        # 4dbd0 <anon.63d0f987bce3d84fd72ab39c9562e4c9.33.llvm.7785120977846935677+0x10>
  12d635:	vinserti128 $0x1,%xmm9,%ymm11,%ymm9
  12d63b:	vpsllvq %ymm12,%ymm9,%ymm9
  12d640:	vpsrlvq -0xdf9c9(%rip),%xmm4,%xmm11        # 4dc80 <anon.63d0f987bce3d84fd72ab39c9562e4c9.26.llvm.7785120977846935677+0x10>
  12d649:	vpsrlvq -0xdfa32(%rip),%xmm4,%xmm13        # 4dc20 <anon.63d0f987bce3d84fd72ab39c9562e4c9.33.llvm.7785120977846935677+0x60>
  12d652:	vpshldq $0x7,%xmm7,%xmm4,%xmm14
  12d659:	vpsrlq $0x39,%xmm4,%xmm15
  12d65e:	vpternlogq $0x96,%xmm13,%xmm11,%xmm15
  12d665:	vinserti128 $0x1,%xmm3,%ymm2,%ymm2
  12d66b:	vmovdqa -0xe0193(%rip),%ymm3        # 4d4e0 <anon.25c1abbc0d37fd1a879b913e2262f7b9.7.llvm.8827852567942549522+0x260>
  12d673:	vpermi2q %ymm2,%ymm15,%ymm3
  12d679:	vmovdqa %ymm15,%ymm2
  12d67d:	vinserti128 $0x1,%xmm4,%ymm4,%ymm4
  12d683:	vmovdqa -0xdffcb(%rip),%ymm11        # 4d6c0 <anon.25c1abbc0d37fd1a879b913e2262f7b9.7.llvm.8827852567942549522+0x440>
  12d68b:	vpermi2q %ymm4,%ymm15,%ymm11
  12d691:	vpermt2q %ymm5,%ymm6,%ymm15
  12d697:	vpsllvq -0xe04e0(%rip),%ymm15,%ymm4        # 4d1c0 <anon.ab55e065755f1972554ab355d5d0b777.9.llvm.13326952744552151834+0x1a6>
  12d6a0:	vpsllvq -0xe0169(%rip),%ymm3,%ymm3        # 4d540 <anon.25c1abbc0d37fd1a879b913e2262f7b9.7.llvm.8827852567942549522+0x2c0>
  12d6a9:	vpermt2q %ymm8,%ymm6,%ymm2
  12d6af:	vpsllvq %ymm12,%ymm2,%ymm2
  12d6b4:	vinserti128 $0x1,%xmm1,%ymm0,%ymm0
  12d6ba:	vmovdqa -0xe0082(%rip),%ymm1        # 4d640 <anon.25c1abbc0d37fd1a879b913e2262f7b9.7.llvm.8827852567942549522+0x3c0>
  12d6c2:	vpermi2q %ymm0,%ymm10,%ymm1
  12d6c8:	vpternlogq $0x96,%ymm9,%ymm3,%ymm1
  12d6cf:	vpternlogq $0x96,%ymm4,%ymm2,%ymm1
  12d6d6:	vpermi2q %ymm14,%ymm7,%ymm6
  12d6dc:	vpternlogq $0x96,%ymm6,%ymm1,%ymm11
  12d6e3:	vmovdqu %ymm11,(%rax)
  12d6e7:	mov    %rcx,0x20(%rax)
  12d6eb:	mov    %r14,0x28(%rax)
  12d6ef:	lea    -0x28(%rbp),%rsp
  12d6f3:	pop    %rbx
  12d6f4:	pop    %r12
  12d6f6:	pop    %r13
  12d6f8:	pop    %r14
  12d6fa:	pop    %r15
  12d6fc:	pop    %rbp
  12d6fd:	vzeroupper 
  12d700:	ret    
  12d701:	vmovaps %xmm0,0x410(%rsp)
  12d70a:	lea    0xa8697(%rip),%rcx        # 1d5da8 <anon.63cb55c002f8575e8c0cec61335b015d.2.llvm.1934387363032200245+0x1a8>
  12d711:	mov    %r14,%rdi
  12d714:	mov    0xe0(%rsp),%rdx
  12d71c:	vzeroupper 
  12d71f:	call   *0xad60b(%rip)        # 1dad30 <_DYNAMIC+0x2e8>
  12d725:	vmovaps %xmm0,0x410(%rsp)
  12d72e:	lea    0xa868b(%rip),%rcx        # 1d5dc0 <anon.63cb55c002f8575e8c0cec61335b015d.2.llvm.1934387363032200245+0x1c0>
  12d735:	mov    %r14,%rdi
  12d738:	mov    0xd8(%rsp),%rdx
  12d740:	vzeroupper 
  12d743:	call   *0xad5e7(%rip)        # 1dad30 <_DYNAMIC+0x2e8>
  12d749:	lea    0xa88e0(%rip),%rcx        # 1d6030 <anon.63cb55c002f8575e8c0cec61335b015d.2.llvm.1934387363032200245+0x430>
  12d750:	call   *0xad5da(%rip)        # 1dad30 <_DYNAMIC+0x2e8>
  12d756:	lea    0xa88bb(%rip),%rcx        # 1d6018 <anon.63cb55c002f8575e8c0cec61335b015d.2.llvm.1934387363032200245+0x418>
  12d75d:	mov    %r14,%rdi
  12d760:	call   *0xad5ca(%rip)        # 1dad30 <_DYNAMIC+0x2e8>
  12d766:	lea    0xa88db(%rip),%rcx        # 1d6048 <anon.63cb55c002f8575e8c0cec61335b015d.2.llvm.1934387363032200245+0x448>
  12d76d:	call   *0xad5bd(%rip)        # 1dad30 <_DYNAMIC+0x2e8>
  12d773:	vmovaps 0xc0(%rsp),%xmm0
  12d77c:	vmovaps %xmm0,0x410(%rsp)
  12d785:	lea    0xa867c(%rip),%rdx        # 1d5e08 <anon.63cb55c002f8575e8c0cec61335b015d.2.llvm.1934387363032200245+0x208>
  12d78c:	call   *0xad6d6(%rip)        # 1dae68 <_DYNAMIC+0x420>
  12d792:	vmovaps 0xc0(%rsp),%xmm0
  12d79b:	vmovaps %xmm0,0x410(%rsp)
  12d7a4:	lea    0xa8645(%rip),%rdx        # 1d5df0 <anon.63cb55c002f8575e8c0cec61335b015d.2.llvm.1934387363032200245+0x1f0>
  12d7ab:	mov    %rax,%rdi
  12d7ae:	mov    0x18(%rsp),%rsi
  12d7b3:	call   *0xad6af(%rip)        # 1dae68 <_DYNAMIC+0x420>
  12d7b9:	vmovaps 0xc0(%rsp),%xmm0
  12d7c2:	vmovaps %xmm0,0x410(%rsp)
  12d7cb:	lea    0xa8606(%rip),%rdx        # 1d5dd8 <anon.63cb55c002f8575e8c0cec61335b015d.2.llvm.1934387363032200245+0x1d8>
  12d7d2:	call   *0xad690(%rip)        # 1dae68 <_DYNAMIC+0x420>
  12d7d8:	vmovaps 0xc0(%rsp),%xmm0
  12d7e1:	vmovaps %xmm0,0x410(%rsp)
  12d7ea:	lea    0xa862f(%rip),%rdx        # 1d5e20 <anon.63cb55c002f8575e8c0cec61335b015d.2.llvm.1934387363032200245+0x220>
  12d7f1:	mov    %rax,%rdi
  12d7f4:	call   *0xad66e(%rip)        # 1dae68 <_DYNAMIC+0x420>
