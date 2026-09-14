00000000006333e0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_>:
  6333e0:	push   rbp
  6333e1:	push   r15
  6333e3:	push   r14
  6333e5:	push   r13
  6333e7:	push   r12
  6333e9:	push   rbx
  6333ea:	mov    r11,rsp
  6333ed:	sub    r11,0xc000
  6333f4:	sub    rsp,0x1000
  6333fb:	mov    QWORD PTR [rsp],0x0
  633403:	cmp    rsp,r11
  633406:	jne    6333f4 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x14>
  633408:	sub    rsp,0x988
  63340f:	mov    r14,r9
  633412:	mov    r13,rdi
  633415:	mov    rbx,QWORD PTR [rsp+0xc9c8]
  63341d:	mov    r12,QWORD PTR [rsp+0xc9c0]
  633425:	mov    QWORD PTR [rsp+0xc0],rsi
  63342d:	mov    QWORD PTR [rsp+0xc8],rcx
  633435:	mov    QWORD PTR [rsp+0xd0],r8
  63343d:	shr    rsi,1
  633440:	cmp    rsi,r8
  633443:	jae    63445f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x107f>
  633449:	mov    QWORD PTR [rsp+0x58],r13
  63344e:	mov    rbx,QWORD PTR [rbx+0x18]
  633452:	lea    rdi,[rsp+0x7980]
  63345a:	mov    edx,0x3000
  63345f:	xor    esi,esi
  633461:	call   QWORD PTR [rip+0xbb1c1]        # 6ee628 <memset@GLIBC_2.2.5>
  633467:	cmp    r14,r12
  63346a:	jae    6342f8 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0xf18>
  633470:	mov    rdi,r14
  633473:	movabs r12,0x7fffffffffffffc0
  63347d:	mov    rax,QWORD PTR [rbx]
  633480:	mov    QWORD PTR [rsp+0x20],rax
  633485:	mov    rax,QWORD PTR [rbx+0x8]
  633489:	mov    rcx,QWORD PTR [rbx+0x10]
  63348d:	mov    QWORD PTR [rsp+0x40],rcx
  633492:	mov    rcx,QWORD PTR [rbx+0x20]
  633496:	mov    QWORD PTR [rsp+0x38],rcx
  63349b:	mov    rcx,QWORD PTR [rbx+0x30]
  63349f:	mov    QWORD PTR [rsp+0x88],rcx
  6334a7:	mov    rcx,QWORD PTR [rbx+0x38]
  6334ab:	mov    QWORD PTR [rsp+0x30],rcx
  6334b0:	mov    rcx,QWORD PTR [rbx+0x40]
  6334b4:	mov    rdx,QWORD PTR [rbx+0x48]
  6334b8:	mov    QWORD PTR [rsp+0x48],rdx
  6334bd:	mov    rbp,QWORD PTR [rbx+0x50]
  6334c1:	mov    rdx,QWORD PTR [rax+0x8]
  6334c5:	mov    r8,QWORD PTR [rax+0x10]
  6334c9:	mov    r13,QWORD PTR [rcx+0x8]
  6334cd:	add    r13,QWORD PTR [rcx+0x48]
  6334d1:	mov    eax,0x87
  6334d6:	vmovq  xmm0,rax
  6334db:	vmovdqa XMMWORD PTR [rsp+0x170],xmm0
  6334e4:	mov    rax,r8
  6334e7:	shl    rax,0x4
  6334eb:	mov    QWORD PTR [rsp+0x50],rdx
  6334f0:	add    rax,rdx
  6334f3:	mov    QWORD PTR [rsp+0x68],rax
  6334f8:	lea    r14,[rsp+0x1180]
  633500:	lea    rsi,[rsp+0x7980]
  633508:	mov    rax,QWORD PTR [rip+0xbb109]        # 6ee618 <memcpy@GLIBC_2.14>
  63350f:	lea    r15,[rip+0xffffffffff9dc16a]        # f680 <_RNvNvNtCstrZ1g5xv8S_15crossbeam_epoch5guard11unprotected11UNPROTECTED+0x540>
  633516:	mov    QWORD PTR [rsp+0x60],r8
  63351b:	mov    QWORD PTR [rsp+0x18],rbp
  633520:	mov    edx,0x3000
  633525:	mov    rbx,rdi
  633528:	mov    rdi,r14
  63352b:	call   rax
  63352d:	mov    QWORD PTR [rsp+0x28],rbx
  633532:	mov    QWORD PTR [rsp+0x4180],rbx
  63353a:	mov    edx,0x1000
  63353f:	mov    rdi,r14
  633542:	xor    esi,esi
  633544:	call   QWORD PTR [rip+0xbb0de]        # 6ee628 <memset@GLIBC_2.2.5>
  63354a:	cmp    QWORD PTR [rsp+0x60],0x0
  633550:	vmovdqa64 zmm8,ZMMWORD PTR [rip+0xffffffffff9dc6a6]        # fc00 <anon.01aa214378fce5a476fd64886db4a5c0.2569.llvm.5168012589948291949+0x1f9>
  63355a:	vmovdqa64 zmm9,ZMMWORD PTR [rip+0xffffffffff9dc6dc]        # fc40 <anon.01aa214378fce5a476fd64886db4a5c0.2569.llvm.5168012589948291949+0x239>
  633564:	vmovdqa xmm10,XMMWORD PTR [rsp+0x170]
  63356d:	vpbroadcastq zmm11,QWORD PTR [rip+0xffffffffff9d95c1]        # cb38 <anon.36d21461a6996b7f2bd9a76e3225918f.29.llvm.7983859039285366149+0x118>
  633577:	vmovdqa64 zmm12,ZMMWORD PTR [rip+0xffffffffff9dbdbf]        # f340 <_RNvNvNtCstrZ1g5xv8S_15crossbeam_epoch5guard11unprotected11UNPROTECTED+0x200>
  633581:	vmovdqa64 zmm13,ZMMWORD PTR [rip+0xffffffffff9dbd75]        # f300 <_RNvNvNtCstrZ1g5xv8S_15crossbeam_epoch5guard11unprotected11UNPROTECTED+0x1c0>
  63358b:	lea    r14,[rip+0xffffffffff9dc0fe]        # f690 <_RNvNvNtCstrZ1g5xv8S_15crossbeam_epoch5guard11unprotected11UNPROTECTED+0x550>
  633592:	je     633f5c <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0xb7c>
  633598:	mov    rax,QWORD PTR [rsp+0x20]
  63359d:	movzx  eax,BYTE PTR [rax+0x30]
  6335a1:	shlx   rax,QWORD PTR [rsp+0x28],rax
  6335a8:	mov    QWORD PTR [rsp+0x70],rax
  6335ad:	mov    rcx,QWORD PTR [rsp+0x50]
  6335b2:	xor    eax,eax
  6335b4:	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  6335c0:	mov    rdx,rax
  6335c3:	mov    QWORD PTR [rsp+0x90],rcx
  6335cb:	lea    rax,[rcx+0x10]
  6335cf:	mov    QWORD PTR [rsp+0x78],rax
  6335d4:	lea    rax,[rdx+0x1]
  6335d8:	mov    QWORD PTR [rsp+0x80],rax
  6335e0:	or     rdx,QWORD PTR [rsp+0x70]
  6335e5:	shl    rdx,0xb
  6335e9:	mov    rax,QWORD PTR [rsp+0x40]
  6335ee:	lea    r10,[rax+rdx*1]
  6335f2:	mov    rax,QWORD PTR [rsp+0x38]
  6335f7:	mov    QWORD PTR [rsp+0x98],rdx
  6335ff:	lea    r11,[rax+rdx*1]
  633603:	xor    eax,eax
  633605:	data16 cs nop WORD PTR [rax+rax*1+0x0]
  633610:	mov    rcx,rax
  633613:	inc    rax
  633616:	mov    QWORD PTR [rsp+0xa8],rax
  63361e:	mov    rdx,rcx
  633621:	shl    rdx,0x7
  633625:	add    rdx,QWORD PTR [rsp+0x98]
  63362d:	shl    rcx,0x6
  633631:	lea    rax,[rsp+0x2180]
  633639:	add    rax,rcx
  63363c:	mov    QWORD PTR [rsp+0x10],rax
  633641:	lea    rax,[rsp+0x2580]
  633649:	mov    QWORD PTR [rsp+0xa0],rcx
  633651:	add    rax,rcx
  633654:	mov    QWORD PTR [rsp+0x8],rax
  633659:	vpxor  xmm0,xmm0,xmm0
  63365d:	mov    QWORD PTR [rsp+0xb0],r11
  633665:	mov    QWORD PTR [rsp+0xb8],r10
  63366d:	vpxor  xmm1,xmm1,xmm1
  633671:	xor    edi,edi
  633673:	mov    QWORD PTR [rsp],rdx
  633677:	jmp    63369e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x2be>
  633679:	nop    DWORD PTR [rax+0x0]
  633680:	vpxord zmm1,zmm1,zmm3
  633686:	vpxord zmm0,zmm0,zmm2
  63368c:	add    r10,0x10
  633690:	add    r11,0x10
  633694:	cmp    rdi,0x8
  633698:	je     6339d0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x5f0>
  63369e:	mov    r9,rdi
  6336a1:	inc    rdi
  6336a4:	mov    rcx,r9
  6336a7:	shl    rcx,0x4
  6336ab:	add    rcx,rdx
  6336ae:	mov    rax,QWORD PTR [rsp+0x40]
  6336b3:	movzx  eax,BYTE PTR [rax+rcx*1]
  6336b7:	shl    eax,0x7
  6336ba:	vmovdqu64 zmm3,ZMMWORD PTR [r13+rax*1+0x0]
  6336c2:	vmovdqu64 zmm2,ZMMWORD PTR [r13+rax*1+0x40]
  6336ca:	mov    eax,0x1
  6336cf:	mov    esi,0x8
  6336d4:	jmp    633712 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x332>
  6336d6:	vpermq zmm4,zmm12,zmm4
  6336dc:	xor    r8,0x40
  6336e0:	vmovdqu64 zmm5,ZMMWORD PTR [rdx+r8*1]
  6336e7:	vshufi64x2 zmm5,zmm5,zmm5,0x4e
  6336ee:	vpxord zmm3,zmm4,zmm3
  6336f4:	vpshufd zmm4,zmm5,0x4e
  6336fb:	vpxord zmm2,zmm4,zmm2
  633701:	inc    rax
  633704:	add    rsi,0x8
  633708:	cmp    rax,0x10
  63370c:	je     633810 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x430>
  633712:	movzx  edx,BYTE PTR [r10+rax*1]
  633717:	shl    edx,0x7
  63371a:	add    rdx,r13
  63371d:	mov    ebx,eax
  63371f:	shr    ebx,1
  633721:	and    ebx,0x3
  633724:	mov    r8,rsi
  633727:	and    r8,r12
  63372a:	vmovdqu64 zmm4,ZMMWORD PTR [r8+rdx*1]
  633731:	test   al,0x1
  633733:	jne    633750 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x370>
  633735:	movsxd rbx,DWORD PTR [r14+rbx*4]
  633739:	add    rbx,r14
  63373c:	jmp    rbx
  63373e:	xor    r8,0x40
  633742:	vmovdqu64 zmm5,ZMMWORD PTR [rdx+r8*1]
  633749:	jmp    6337d6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x3f6>
  63374e:	xchg   ax,ax
  633750:	movsxd rbx,DWORD PTR [r15+rbx*4]
  633754:	add    rbx,r15
  633757:	jmp    rbx
  633759:	vpshufd zmm4,zmm4,0x4e
  633760:	xor    r8,0x40
  633764:	vmovdqu64 zmm5,ZMMWORD PTR [rdx+r8*1]
  63376b:	jmp    6336ee <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x30e>
  633770:	vshufi64x2 zmm4,zmm4,zmm4,0x4e
  633777:	xor    r8,0x40
  63377b:	vmovdqu64 zmm5,ZMMWORD PTR [rdx+r8*1]
  633782:	vshufi64x2 zmm5,zmm5,zmm5,0x4e
  633789:	jmp    6337d6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x3f6>
  63378b:	vshufi64x2 zmm4,zmm4,zmm4,0x1b
  633792:	xor    r8,0x40
  633796:	vmovdqu64 zmm5,ZMMWORD PTR [rdx+r8*1]
  63379d:	vshufi64x2 zmm5,zmm5,zmm5,0x1b
  6337a4:	jmp    6337d6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x3f6>
  6337a6:	vpermq zmm4,zmm13,zmm4
  6337ac:	xor    r8,0x40
  6337b0:	vmovdqu64 zmm5,ZMMWORD PTR [rdx+r8*1]
  6337b7:	vshufi64x2 zmm5,zmm5,zmm5,0x1b
  6337be:	jmp    6336ee <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x30e>
  6337c3:	vpermq zmm4,zmm4,0x4e
  6337ca:	xor    r8,0x40
  6337ce:	vpermq zmm5,ZMMWORD PTR [rdx+r8*1],0x4e
  6337d6:	vpxord zmm3,zmm3,zmm4
  6337dc:	vpxord zmm2,zmm2,zmm5
  6337e2:	inc    rax
  6337e5:	add    rsi,0x8
  6337e9:	cmp    rax,0x10
  6337ed:	jne    633712 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x332>
  6337f3:	jmp    633810 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x430>
  6337f5:	vpermq zmm4,zmm4,0x1b
  6337fc:	xor    r8,0x40
  633800:	vpermq zmm5,ZMMWORD PTR [rdx+r8*1],0x4e
  633808:	jmp    6336ee <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x30e>
  63380d:	nop    DWORD PTR [rax]
  633810:	mov    rax,QWORD PTR [rsp+0x38]
  633815:	movzx  eax,BYTE PTR [rax+rcx*1]
  633819:	shl    eax,0x7
  63381c:	vmovdqu64 zmm5,ZMMWORD PTR [r13+rax*1+0x0]
  633824:	vmovdqu64 zmm4,ZMMWORD PTR [r13+rax*1+0x40]
  63382c:	mov    eax,0x1
  633831:	mov    ecx,0x8
  633836:	jmp    633874 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x494>
  633838:	vpermq zmm6,zmm12,zmm6
  63383e:	xor    rsi,0x40
  633842:	vmovdqu64 zmm7,ZMMWORD PTR [rdx+rsi*1]
  633849:	vshufi64x2 zmm7,zmm7,zmm7,0x4e
  633850:	vpxord zmm5,zmm6,zmm5
  633856:	vpshufd zmm6,zmm7,0x4e
  63385d:	vpxord zmm4,zmm6,zmm4
  633863:	inc    rax
  633866:	add    rcx,0x8
  63386a:	cmp    rax,0x10
  63386e:	je     633990 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x5b0>
  633874:	movzx  edx,BYTE PTR [r11+rax*1]
  633879:	shl    edx,0x7
  63387c:	add    rdx,r13
  63387f:	mov    r8d,eax
  633882:	shr    r8d,1
  633885:	and    r8d,0x3
  633889:	mov    rsi,rcx
  63388c:	and    rsi,r12
  63388f:	vmovdqu64 zmm6,ZMMWORD PTR [rsi+rdx*1]
  633896:	test   al,0x1
  633898:	jne    6338c0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x4e0>
  63389a:	lea    rbx,[rip+0xffffffffff9dbe0f]        # f6b0 <_RNvNvNtCstrZ1g5xv8S_15crossbeam_epoch5guard11unprotected11UNPROTECTED+0x570>
  6338a1:	movsxd r8,DWORD PTR [rbx+r8*4]
  6338a5:	add    r8,rbx
  6338a8:	jmp    r8
  6338ab:	xor    rsi,0x40
  6338af:	vmovdqu64 zmm7,ZMMWORD PTR [rdx+rsi*1]
  6338b6:	jmp    63394e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x56e>
  6338bb:	nop    DWORD PTR [rax+rax*1+0x0]
  6338c0:	lea    rbx,[rip+0xffffffffff9dbdd9]        # f6a0 <_RNvNvNtCstrZ1g5xv8S_15crossbeam_epoch5guard11unprotected11UNPROTECTED+0x560>
  6338c7:	movsxd r8,DWORD PTR [rbx+r8*4]
  6338cb:	add    r8,rbx
  6338ce:	jmp    r8
  6338d1:	vpshufd zmm6,zmm6,0x4e
  6338d8:	xor    rsi,0x40
  6338dc:	vmovdqu64 zmm7,ZMMWORD PTR [rdx+rsi*1]
  6338e3:	jmp    633850 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x470>
  6338e8:	vshufi64x2 zmm6,zmm6,zmm6,0x4e
  6338ef:	xor    rsi,0x40
  6338f3:	vmovdqu64 zmm7,ZMMWORD PTR [rdx+rsi*1]
  6338fa:	vshufi64x2 zmm7,zmm7,zmm7,0x4e
  633901:	jmp    63394e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x56e>
  633903:	vshufi64x2 zmm6,zmm6,zmm6,0x1b
  63390a:	xor    rsi,0x40
  63390e:	vmovdqu64 zmm7,ZMMWORD PTR [rdx+rsi*1]
  633915:	vshufi64x2 zmm7,zmm7,zmm7,0x1b
  63391c:	jmp    63394e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x56e>
  63391e:	vpermq zmm6,zmm13,zmm6
  633924:	xor    rsi,0x40
  633928:	vmovdqu64 zmm7,ZMMWORD PTR [rdx+rsi*1]
  63392f:	vshufi64x2 zmm7,zmm7,zmm7,0x1b
  633936:	jmp    633850 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x470>
  63393b:	vpermq zmm6,zmm6,0x4e
  633942:	xor    rsi,0x40
  633946:	vpermq zmm7,ZMMWORD PTR [rdx+rsi*1],0x4e
  63394e:	vpxord zmm5,zmm5,zmm6
  633954:	vpxord zmm4,zmm4,zmm7
  63395a:	inc    rax
  63395d:	add    rcx,0x8
  633961:	cmp    rax,0x10
  633965:	jne    633874 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x494>
  63396b:	jmp    633990 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x5b0>
  63396d:	vpermq zmm6,zmm6,0x1b
  633974:	xor    rsi,0x40
  633978:	vpermq zmm7,ZMMWORD PTR [rdx+rsi*1],0x4e
  633980:	jmp    633850 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x470>
  633985:	data16 cs nop WORD PTR [rax+rax*1+0x0]
  633990:	vgf2p8mulb zmm3,zmm3,zmm5
  633996:	vgf2p8mulb zmm2,zmm2,zmm4
  63399c:	test   r9,r9
  63399f:	mov    rdx,QWORD PTR [rsp]
  6339a3:	je     633680 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x2a0>
  6339a9:	mov    al,0x1
  6339ab:	mov    ecx,r9d
  6339ae:	shl    al,cl
  6339b0:	vpbroadcastb zmm4,eax
  6339b6:	vgf2p8mulb zmm3,zmm3,zmm4
  6339bc:	vgf2p8mulb zmm2,zmm2,zmm4
  6339c2:	jmp    633680 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x2a0>
  6339c7:	nop    WORD PTR [rax+rax*1+0x0]
  6339d0:	mov    rax,QWORD PTR [rsp+0x10]
  6339d5:	vmovdqu64 ZMMWORD PTR [rax],zmm1
  6339db:	mov    rax,QWORD PTR [rsp+0x8]
  6339e0:	vmovdqu64 ZMMWORD PTR [rax],zmm0
  6339e6:	mov    rax,rdx
  6339e9:	or     rax,0x7f
  6339ed:	cmp    rax,QWORD PTR [rsp+0x30]
  6339f2:	jae    634489 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x10a9>
  6339f8:	mov    rax,QWORD PTR [rsp+0x88]
  633a00:	vmovdqu64 zmm0,ZMMWORD PTR [rax+rdx*1]
  633a07:	vmovdqu64 zmm1,ZMMWORD PTR [rax+rdx*1+0x40]
  633a0f:	vmovdqa64 zmm2,zmm0
  633a15:	vpermt2b zmm2,zmm8,zmm1
  633a1b:	vgf2p8affineqb zmm2,zmm11,zmm2,0x0
  633a22:	vpermt2b zmm0,zmm9,zmm1
  633a28:	lea    rax,[rsp+0x1580]
  633a30:	mov    rcx,QWORD PTR [rsp+0xa0]
  633a38:	vmovdqu64 ZMMWORD PTR [rax+rcx*1+0x1400],zmm2
  633a40:	vgf2p8affineqb zmm0,zmm11,zmm0,0x0
  633a47:	vmovdqu64 ZMMWORD PTR [rax+rcx*1+0x1800],zmm0
  633a4f:	mov    r10,QWORD PTR [rsp+0xb8]
  633a57:	sub    r10,0xffffffffffffff80
  633a5b:	mov    r11,QWORD PTR [rsp+0xb0]
  633a63:	sub    r11,0xffffffffffffff80
  633a67:	mov    rax,QWORD PTR [rsp+0xa8]
  633a6f:	cmp    rax,0x10
  633a73:	jne    633610 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x230>
  633a79:	lea    rcx,[rsp+0x29c0]
  633a81:	xor    eax,eax
  633a83:	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  633a90:	mov    QWORD PTR [rsp+0x10],rax
  633a95:	inc    rax
  633a98:	mov    QWORD PTR [rsp+0x8],rax
  633a9d:	mov    QWORD PTR [rsp],rcx
  633aa1:	xor    esi,esi
  633aa3:	xor    r8d,r8d
  633aa6:	xor    r11d,r11d
  633aa9:	xor    r10d,r10d
  633aac:	xor    r9d,r9d
  633aaf:	nop
  633ab0:	movzx  edi,BYTE PTR [rcx-0x840]
  633ab7:	add    rdi,rsi
  633aba:	cmp    rdi,rbp
  633abd:	jae    634777 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x1397>
  633ac3:	movzx  eax,BYTE PTR [rcx-0x40]
  633ac7:	add    rax,rsi
  633aca:	cmp    rax,rbp
  633acd:	jae    634797 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x13b7>
  633ad3:	movzx  ebx,BYTE PTR [rcx-0x800]
  633ada:	lea    rdx,[rsi+rbx*1+0x100]
  633ae2:	cmp    rdx,rbp
  633ae5:	jae    63476f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x138f>
  633aeb:	mov    r14,rbp
  633aee:	movzx  ebp,BYTE PTR [rcx]
  633af1:	lea    rbx,[rsi+rbp*1+0x100]
  633af9:	cmp    rbx,r14
  633afc:	jae    63478a <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x13aa>
  633b02:	shl    rdi,0x4
  633b06:	mov    r14,QWORD PTR [rsp+0x48]
  633b0b:	xor    r11,QWORD PTR [r14+rdi*1+0x8]
  633b10:	xor    r10,QWORD PTR [r14+rdi*1]
  633b14:	shl    rax,0x4
  633b18:	xor    r9,QWORD PTR [r14+rax*1]
  633b1c:	xor    r8,QWORD PTR [r14+rax*1+0x8]
  633b21:	shl    rdx,0x4
  633b25:	xor    r11,QWORD PTR [r14+rdx*1+0x8]
  633b2a:	xor    r10,QWORD PTR [r14+rdx*1]
  633b2e:	shl    rbx,0x4
  633b32:	xor    r9,QWORD PTR [r14+rbx*1]
  633b36:	xor    r8,QWORD PTR [r14+rbx*1+0x8]
  633b3b:	add    rsi,0x200
  633b42:	sub    rcx,0xffffffffffffff80
  633b46:	cmp    rsi,0x1000
  633b4d:	mov    rbp,QWORD PTR [rsp+0x18]
  633b52:	jne    633ab0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x6d0>
  633b58:	mov    rdx,QWORD PTR [rsp+0x10]
  633b5d:	shl    rdx,0x4
  633b61:	vmovq  xmm1,r10
  633b66:	mov    rcx,QWORD PTR [rsp+0x90]
  633b6e:	mov    rax,QWORD PTR [rcx]
  633b71:	mov    rcx,QWORD PTR [rcx+0x8]
  633b75:	vmovq  xmm0,rax
  633b7a:	vpclmullqlqdq xmm3,xmm1,xmm0
  633b80:	vmovq  xmm2,r11
  633b85:	vmovq  xmm1,rcx
  633b8a:	vpclmullqlqdq xmm4,xmm2,xmm1
  633b90:	xor    r10,r11
  633b93:	xor    rcx,rax
  633b96:	vmovq  xmm5,r10
  633b9b:	vmovq  xmm2,rcx
  633ba0:	vpclmullqlqdq xmm5,xmm5,xmm2
  633ba6:	vpternlogq xmm5,xmm3,xmm4,0x96
  633bad:	vpshufd xmm6,xmm4,0xee
  633bb2:	vpclmullqlqdq xmm6,xmm6,xmm10
  633bb8:	vpshufd xmm7,xmm5,0xee
  633bbd:	vpxor  xmm4,xmm7,xmm4
  633bc1:	vpextrq rax,xmm6,0x1
  633bc7:	vpclmullqlqdq xmm4,xmm4,xmm10
  633bcd:	lea    rcx,[rax+rax*1]
  633bd1:	shl    rax,0x2
  633bd5:	xor    rax,rcx
  633bd8:	vpxor  xmm7,xmm3,xmm4
  633bdc:	vmovq  rcx,xmm7
  633be1:	xor    rcx,rax
  633be4:	vpshufd xmm3,xmm3,0xee
  633be9:	vpsllq xmm7,xmm6,0x7
  633bee:	vpxor  xmm3,xmm6,xmm3
  633bf2:	vpalignr xmm3,xmm3,xmm7,0x8
  633bf8:	vpbroadcastq xmm5,xmm5
  633bfd:	vpinsrq xmm5,xmm5,rcx,0x0
  633c03:	vpxor  xmm3,xmm3,xmm5
  633c07:	vmovdqa xmm5,XMMWORD PTR [rsp+rdx*1+0x1180]
  633c10:	vpblendd xmm4,xmm5,xmm4,0xc
  633c16:	vpunpckhqdq xmm5,xmm6,xmm5
  633c1a:	vpternlogq xmm5,xmm3,xmm4,0x96
  633c21:	vmovdqa XMMWORD PTR [rsp+rdx*1+0x1180],xmm5
  633c2a:	vmovq  xmm3,r9
  633c2f:	vpclmullqlqdq xmm3,xmm3,xmm0
  633c35:	vmovq  xmm4,r8
  633c3a:	vpclmullqlqdq xmm4,xmm4,xmm1
  633c40:	xor    r8,r9
  633c43:	vmovq  xmm5,r8
  633c48:	vpclmullqlqdq xmm5,xmm5,xmm2
  633c4e:	vpternlogq xmm5,xmm3,xmm4,0x96
  633c55:	vpshufd xmm6,xmm4,0xee
  633c5a:	vpclmullqlqdq xmm6,xmm6,xmm10
  633c60:	vpshufd xmm7,xmm5,0xee
  633c65:	vpxor  xmm4,xmm7,xmm4
  633c69:	vpextrq rax,xmm6,0x1
  633c6f:	vpclmullqlqdq xmm4,xmm4,xmm10
  633c75:	lea    rcx,[rax+rax*1]
  633c79:	shl    rax,0x2
  633c7d:	xor    rax,rcx
  633c80:	vpxor  xmm7,xmm3,xmm4
  633c84:	vmovq  rcx,xmm7
  633c89:	xor    rcx,rax
  633c8c:	vpshufd xmm3,xmm3,0xee
  633c91:	vpsllq xmm7,xmm6,0x7
  633c96:	vpxor  xmm3,xmm6,xmm3
  633c9a:	vpalignr xmm3,xmm3,xmm7,0x8
  633ca0:	vpbroadcastq xmm5,xmm5
  633ca5:	vpinsrq xmm5,xmm5,rcx,0x0
  633cab:	vpxor  xmm3,xmm3,xmm5
  633caf:	lea    rax,[rsp+0x1580]
  633cb7:	vmovdqa xmm5,XMMWORD PTR [rax+rdx*1+0x400]
  633cc0:	vpblendd xmm4,xmm5,xmm4,0xc
  633cc6:	vpunpckhqdq xmm5,xmm6,xmm5
  633cca:	vpternlogq xmm5,xmm3,xmm4,0x96
  633cd1:	vmovdqa XMMWORD PTR [rax+rdx*1+0x400],xmm5
  633cda:	mov    rcx,QWORD PTR [rsp]
  633cde:	inc    rcx
  633ce1:	mov    rdx,QWORD PTR [rsp+0x8]
  633ce6:	mov    rax,rdx
  633ce9:	cmp    rdx,0x40
  633ced:	jne    633a90 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x6b0>
  633cf3:	lea    rcx,[rsp+0x2dc0]
  633cfb:	xor    eax,eax
  633cfd:	nop    DWORD PTR [rax]
  633d00:	mov    QWORD PTR [rsp+0x10],rax
  633d05:	inc    rax
  633d08:	mov    QWORD PTR [rsp+0x8],rax
  633d0d:	mov    QWORD PTR [rsp],rcx
  633d11:	xor    esi,esi
  633d13:	xor    r8d,r8d
  633d16:	xor    r10d,r10d
  633d19:	xor    r11d,r11d
  633d1c:	xor    r9d,r9d
  633d1f:	nop
  633d20:	movzx  edi,BYTE PTR [rcx-0x840]
  633d27:	add    rdi,rsi
  633d2a:	cmp    rdi,rbp
  633d2d:	jae    634777 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x1397>
  633d33:	movzx  eax,BYTE PTR [rcx-0x40]
  633d37:	add    rax,rsi
  633d3a:	cmp    rax,rbp
  633d3d:	jae    634797 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x13b7>
  633d43:	movzx  ebx,BYTE PTR [rcx-0x800]
  633d4a:	lea    rdx,[rsi+rbx*1+0x100]
  633d52:	cmp    rdx,rbp
  633d55:	jae    63476f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x138f>
  633d5b:	mov    r14,rbp
  633d5e:	movzx  ebp,BYTE PTR [rcx]
  633d61:	lea    rbx,[rsi+rbp*1+0x100]
  633d69:	cmp    rbx,r14
  633d6c:	jae    63478a <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x13aa>
  633d72:	shl    rdi,0x4
  633d76:	mov    r14,QWORD PTR [rsp+0x48]
  633d7b:	xor    r10,QWORD PTR [r14+rdi*1+0x8]
  633d80:	xor    r11,QWORD PTR [r14+rdi*1]
  633d84:	shl    rax,0x4
  633d88:	xor    r9,QWORD PTR [r14+rax*1]
  633d8c:	xor    r8,QWORD PTR [r14+rax*1+0x8]
  633d91:	shl    rdx,0x4
  633d95:	xor    r10,QWORD PTR [r14+rdx*1+0x8]
  633d9a:	xor    r11,QWORD PTR [r14+rdx*1]
  633d9e:	shl    rbx,0x4
  633da2:	xor    r9,QWORD PTR [r14+rbx*1]
  633da6:	xor    r8,QWORD PTR [r14+rbx*1+0x8]
  633dab:	add    rsi,0x200
  633db2:	sub    rcx,0xffffffffffffff80
  633db6:	cmp    rsi,0x1000
  633dbd:	mov    rbp,QWORD PTR [rsp+0x18]
  633dc2:	lea    r14,[rip+0xffffffffff9db8c7]        # f690 <_RNvNvNtCstrZ1g5xv8S_15crossbeam_epoch5guard11unprotected11UNPROTECTED+0x550>
  633dc9:	jne    633d20 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x940>
  633dcf:	mov    rsi,QWORD PTR [rsp+0x10]
  633dd4:	shl    rsi,0x4
  633dd8:	vmovq  xmm3,r11
  633ddd:	vpclmullqlqdq xmm3,xmm3,xmm0
  633de3:	vmovq  xmm4,r10
  633de8:	vpclmullqlqdq xmm4,xmm4,xmm1
  633dee:	xor    r11,r10
  633df1:	vmovq  xmm5,r11
  633df6:	vpclmullqlqdq xmm5,xmm5,xmm2
  633dfc:	vpternlogq xmm5,xmm3,xmm4,0x96
  633e03:	vpshufd xmm6,xmm4,0xee
  633e08:	vpclmullqlqdq xmm6,xmm6,xmm10
  633e0e:	vpshufd xmm7,xmm5,0xee
  633e13:	vpxor  xmm4,xmm7,xmm4
  633e17:	vpextrq rax,xmm6,0x1
  633e1d:	vpclmullqlqdq xmm4,xmm4,xmm10
  633e23:	lea    rcx,[rax+rax*1]
  633e27:	shl    rax,0x2
  633e2b:	xor    rax,rcx
  633e2e:	vpxor  xmm7,xmm3,xmm4
  633e32:	vmovq  rcx,xmm7
  633e37:	xor    rcx,rax
  633e3a:	vpshufd xmm3,xmm3,0xee
  633e3f:	vpsllq xmm7,xmm6,0x7
  633e44:	vpxor  xmm3,xmm6,xmm3
  633e48:	vpalignr xmm3,xmm3,xmm7,0x8
  633e4e:	vpbroadcastq xmm5,xmm5
  633e53:	vpinsrq xmm5,xmm5,rcx,0x0
  633e59:	vpxor  xmm3,xmm3,xmm5
  633e5d:	lea    rdx,[rsp+0x1580]
  633e65:	vmovdqa xmm5,XMMWORD PTR [rdx+rsi*1]
  633e6a:	vpblendd xmm4,xmm5,xmm4,0xc
  633e70:	vpunpckhqdq xmm5,xmm6,xmm5
  633e74:	vpternlogq xmm5,xmm3,xmm4,0x96
  633e7b:	vmovdqa XMMWORD PTR [rdx+rsi*1],xmm5
  633e80:	vmovq  xmm3,r9
  633e85:	vpclmullqlqdq xmm3,xmm3,xmm0
  633e8b:	vmovq  xmm4,r8
  633e90:	vpclmullqlqdq xmm4,xmm4,xmm1
  633e96:	xor    r8,r9
  633e99:	vmovq  xmm5,r8
  633e9e:	vpclmullqlqdq xmm5,xmm5,xmm2
  633ea4:	vpternlogq xmm5,xmm3,xmm4,0x96
  633eab:	vpshufd xmm6,xmm4,0xee
  633eb0:	vpclmullqlqdq xmm6,xmm6,xmm10
  633eb6:	vpshufd xmm7,xmm5,0xee
  633ebb:	vpxor  xmm4,xmm7,xmm4
  633ebf:	vpextrq rax,xmm6,0x1
  633ec5:	vpclmullqlqdq xmm4,xmm4,xmm10
  633ecb:	lea    rcx,[rax+rax*1]
  633ecf:	shl    rax,0x2
  633ed3:	xor    rax,rcx
  633ed6:	vpxor  xmm7,xmm3,xmm4
  633eda:	vmovq  rcx,xmm7
  633edf:	xor    rcx,rax
  633ee2:	vpshufd xmm3,xmm3,0xee
  633ee7:	vpsllq xmm7,xmm6,0x7
  633eec:	vpxor  xmm3,xmm6,xmm3
  633ef0:	vpalignr xmm3,xmm3,xmm7,0x8
  633ef6:	vpbroadcastq xmm5,xmm5
  633efb:	vpinsrq xmm5,xmm5,rcx,0x0
  633f01:	vpxor  xmm3,xmm3,xmm5
  633f05:	vmovdqa xmm5,XMMWORD PTR [rdx+rsi*1+0x800]
  633f0e:	vpblendd xmm4,xmm5,xmm4,0xc
  633f14:	vpunpckhqdq xmm5,xmm6,xmm5
  633f18:	vpternlogq xmm5,xmm3,xmm4,0x96
  633f1f:	vmovdqa XMMWORD PTR [rdx+rsi*1+0x800],xmm5
  633f28:	mov    rcx,QWORD PTR [rsp]
  633f2c:	inc    rcx
  633f2f:	mov    rdx,QWORD PTR [rsp+0x8]
  633f34:	mov    rax,rdx
  633f37:	cmp    rdx,0x40
  633f3b:	jne    633d00 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x920>
  633f41:	mov    rax,QWORD PTR [rsp+0x78]
  633f46:	mov    rcx,rax
  633f49:	cmp    rax,QWORD PTR [rsp+0x68]
  633f4e:	mov    rax,QWORD PTR [rsp+0x80]
  633f56:	jne    6335c0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x1e0>
  633f5c:	mov    rax,QWORD PTR [rsp+0x20]
  633f61:	mov    rsi,QWORD PTR [rax+0x28]
  633f65:	mov    rdi,QWORD PTR [rsp+0x28]
  633f6a:	cmp    rdi,rsi
  633f6d:	jae    634802 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x1422>
  633f73:	lea    rcx,[rdi+0x1]
  633f77:	mov    QWORD PTR [rsp],rcx
  633f7b:	mov    rax,QWORD PTR [rax+0x20]
  633f7f:	shl    rdi,0x4
  633f83:	mov    rcx,QWORD PTR [rax+rdi*1]
  633f87:	mov    rax,QWORD PTR [rax+rdi*1+0x8]
  633f8c:	vmovq  xmm0,rcx
  633f91:	vmovq  xmm1,rax
  633f96:	xor    rax,rcx
  633f99:	vmovq  xmm2,rax
  633f9e:	mov    eax,0x2800
  633fa3:	lea    rbx,[rsp+0x1180]
  633fab:	nop    DWORD PTR [rax+rax*1+0x0]
  633fb0:	mov    rcx,QWORD PTR [rsp+rax*1-0x1680]
  633fb8:	mov    rdx,QWORD PTR [rsp+rax*1-0x1678]
  633fc0:	vmovq  xmm3,rcx
  633fc5:	vpclmullqlqdq xmm3,xmm3,xmm0
  633fcb:	vmovq  xmm4,rdx
  633fd0:	vpclmullqlqdq xmm4,xmm4,xmm1
  633fd6:	xor    rdx,rcx
  633fd9:	vmovq  xmm5,rdx
  633fde:	vpclmullqlqdq xmm5,xmm5,xmm2
  633fe4:	vpternlogq xmm5,xmm3,xmm4,0x96
  633feb:	vpshufd xmm6,xmm4,0xee
  633ff0:	vpclmullqlqdq xmm6,xmm6,xmm10
  633ff6:	vpshufd xmm7,xmm5,0xee
  633ffb:	vpxor  xmm4,xmm7,xmm4
  633fff:	vpclmullqlqdq xmm4,xmm4,xmm10
  634005:	vpextrq rcx,xmm6,0x1
  63400b:	lea    rdx,[rcx+rcx*1]
  63400f:	shl    rcx,0x2
  634013:	xor    rcx,rdx
  634016:	vpxor  xmm7,xmm3,xmm4
  63401a:	vmovq  rdx,xmm7
  63401f:	xor    rdx,rcx
  634022:	vpshufd xmm3,xmm3,0xee
  634027:	vpsllq xmm7,xmm6,0x7
  63402c:	vpxor  xmm3,xmm6,xmm3
  634030:	vpalignr xmm3,xmm3,xmm7,0x8
  634036:	vpbroadcastq xmm5,xmm5
  63403b:	vpinsrq xmm5,xmm5,rdx,0x0
  634041:	vpxor  xmm3,xmm3,xmm5
  634045:	vmovdqa xmm5,XMMWORD PTR [rsp+rax*1+0x980]
  63404e:	vpblendd xmm4,xmm5,xmm4,0xc
  634054:	vpunpckhqdq xmm5,xmm6,xmm5
  634058:	vpternlogq xmm5,xmm3,xmm4,0x96
  63405f:	vmovdqa XMMWORD PTR [rsp+rax*1+0x980],xmm5
  634068:	mov    rcx,QWORD PTR [rsp+rax*1-0xe80]
  634070:	vmovq  xmm3,rcx
  634075:	vpclmullqlqdq xmm3,xmm3,xmm0
  63407b:	mov    rdx,QWORD PTR [rsp+rax*1-0xe78]
  634083:	vmovq  xmm4,rdx
  634088:	vpclmullqlqdq xmm4,xmm4,xmm1
  63408e:	xor    rdx,rcx
  634091:	vmovq  xmm5,rdx
  634096:	vpclmullqlqdq xmm5,xmm5,xmm2
  63409c:	vpternlogq xmm5,xmm3,xmm4,0x96
  6340a3:	vpshufd xmm6,xmm4,0xee
  6340a8:	vpclmullqlqdq xmm6,xmm6,xmm10
  6340ae:	vpshufd xmm7,xmm5,0xee
  6340b3:	vpxor  xmm4,xmm7,xmm4
  6340b7:	vpextrq rcx,xmm6,0x1
  6340bd:	vpclmullqlqdq xmm4,xmm4,xmm10
  6340c3:	lea    rdx,[rcx+rcx*1]
  6340c7:	shl    rcx,0x2
  6340cb:	xor    rcx,rdx
  6340ce:	vpxor  xmm7,xmm3,xmm4
  6340d2:	vmovq  rdx,xmm7
  6340d7:	xor    rdx,rcx
  6340da:	vpshufd xmm3,xmm3,0xee
  6340df:	vpsllq xmm7,xmm6,0x7
  6340e4:	vpxor  xmm3,xmm6,xmm3
  6340e8:	vpalignr xmm3,xmm3,xmm7,0x8
  6340ee:	vpbroadcastq xmm5,xmm5
  6340f3:	vpinsrq xmm5,xmm5,rdx,0x0
  6340f9:	vpxor  xmm3,xmm3,xmm5
  6340fd:	vmovdqa xmm5,XMMWORD PTR [rsp+rax*1+0x1180]
  634106:	vpblendd xmm4,xmm5,xmm4,0xc
  63410c:	vpunpckhqdq xmm5,xmm6,xmm5
  634110:	vpternlogq xmm5,xmm3,xmm4,0x96
  634117:	vmovdqa XMMWORD PTR [rsp+rax*1+0x1180],xmm5
  634120:	add    rax,0x10
  634124:	cmp    rax,0x2c00
  63412a:	jne    633fb0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0xbd0>
  634130:	mov    eax,0x2c00
  634135:	lea    rdi,[rsp+0x7980]
  63413d:	nop    DWORD PTR [rax]
  634140:	mov    rcx,QWORD PTR [rsp+rax*1-0x1680]
  634148:	mov    rdx,QWORD PTR [rsp+rax*1-0x1678]
  634150:	vmovq  xmm3,rcx
  634155:	vpclmullqlqdq xmm3,xmm3,xmm0
  63415b:	vmovq  xmm4,rdx
  634160:	vpclmullqlqdq xmm4,xmm4,xmm1
  634166:	xor    rdx,rcx
  634169:	vmovq  xmm5,rdx
  63416e:	vpclmullqlqdq xmm5,xmm5,xmm2
  634174:	vpternlogq xmm5,xmm3,xmm4,0x96
  63417b:	vpshufd xmm6,xmm4,0xee
  634180:	vpclmullqlqdq xmm6,xmm6,xmm10
  634186:	vpshufd xmm7,xmm5,0xee
  63418b:	vpxor  xmm4,xmm7,xmm4
  63418f:	vpclmullqlqdq xmm4,xmm4,xmm10
  634195:	vpextrq rcx,xmm6,0x1
  63419b:	lea    rdx,[rcx+rcx*1]
  63419f:	shl    rcx,0x2
  6341a3:	xor    rcx,rdx
  6341a6:	vpxor  xmm7,xmm3,xmm4
  6341aa:	vmovq  rdx,xmm7
  6341af:	xor    rdx,rcx
  6341b2:	vpshufd xmm3,xmm3,0xee
  6341b7:	vpsllq xmm7,xmm6,0x7
  6341bc:	vpxor  xmm3,xmm6,xmm3
  6341c0:	vpalignr xmm3,xmm3,xmm7,0x8
  6341c6:	vpbroadcastq xmm5,xmm5
  6341cb:	vpinsrq xmm5,xmm5,rdx,0x0
  6341d1:	vpxor  xmm3,xmm3,xmm5
  6341d5:	vmovdqa xmm5,XMMWORD PTR [rsp+rax*1+0x980]
  6341de:	vpblendd xmm4,xmm5,xmm4,0xc
  6341e4:	vpunpckhqdq xmm5,xmm6,xmm5
  6341e8:	vpternlogq xmm5,xmm3,xmm4,0x96
  6341ef:	vmovdqa XMMWORD PTR [rsp+rax*1+0x980],xmm5
  6341f8:	mov    rcx,QWORD PTR [rsp+rax*1-0xe80]
  634200:	vmovq  xmm3,rcx
  634205:	vpclmullqlqdq xmm3,xmm3,xmm0
  63420b:	mov    rdx,QWORD PTR [rsp+rax*1-0xe78]
  634213:	vmovq  xmm4,rdx
  634218:	vpclmullqlqdq xmm4,xmm4,xmm1
  63421e:	xor    rdx,rcx
  634221:	vmovq  xmm5,rdx
  634226:	vpclmullqlqdq xmm5,xmm5,xmm2
  63422c:	vpternlogq xmm5,xmm3,xmm4,0x96
  634233:	vpshufd xmm6,xmm4,0xee
  634238:	vpclmullqlqdq xmm6,xmm6,xmm10
  63423e:	vpshufd xmm7,xmm5,0xee
  634243:	vpxor  xmm4,xmm7,xmm4
  634247:	vpextrq rcx,xmm6,0x1
  63424d:	vpclmullqlqdq xmm4,xmm4,xmm10
  634253:	lea    rdx,[rcx+rcx*1]
  634257:	shl    rcx,0x2
  63425b:	xor    rcx,rdx
  63425e:	vpxor  xmm7,xmm3,xmm4
  634262:	vmovq  rdx,xmm7
  634267:	xor    rdx,rcx
  63426a:	vpshufd xmm3,xmm3,0xee
  63426f:	vpsllq xmm7,xmm6,0x7
  634274:	vpxor  xmm3,xmm6,xmm3
  634278:	vpalignr xmm3,xmm3,xmm7,0x8
  63427e:	vpbroadcastq xmm5,xmm5
  634283:	vpinsrq xmm5,xmm5,rdx,0x0
  634289:	vpxor  xmm3,xmm3,xmm5
  63428d:	vmovdqa xmm5,XMMWORD PTR [rsp+rax*1+0x1180]
  634296:	vpblendd xmm4,xmm5,xmm4,0xc
  63429c:	vpunpckhqdq xmm5,xmm6,xmm5
  6342a0:	vpternlogq xmm5,xmm3,xmm4,0x96
  6342a7:	vmovdqa XMMWORD PTR [rsp+rax*1+0x1180],xmm5
  6342b0:	add    rax,0x10
  6342b4:	cmp    rax,0x3000
  6342ba:	jne    634140 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0xd60>
  6342c0:	mov    edx,0x3000
  6342c5:	mov    rsi,rbx
  6342c8:	mov    r14,QWORD PTR [rip+0xba349]        # 6ee618 <memcpy@GLIBC_2.14>
  6342cf:	vzeroupper 
  6342d2:	call   r14
  6342d5:	mov    rax,r14
  6342d8:	lea    rsi,[rsp+0x7980]
  6342e0:	mov    rcx,QWORD PTR [rsp]
  6342e4:	mov    rdi,rcx
  6342e7:	cmp    rcx,QWORD PTR [rsp+0xc9c0]
  6342ef:	mov    r14,rbx
  6342f2:	jne    633520 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x140>
  6342f8:	lea    rdi,[rsp+0x1180]
  634300:	lea    rsi,[rsp+0x7980]
  634308:	mov    rbx,QWORD PTR [rip+0xba309]        # 6ee618 <memcpy@GLIBC_2.14>
  63430f:	mov    edx,0x3000
  634314:	call   rbx
  634316:	lea    rsi,[rsp+0x3180]
  63431e:	lea    r15,[rsp+0x5980]
  634326:	mov    edx,0x800
  63432b:	mov    rdi,r15
  63432e:	call   rbx
  634330:	lea    rdi,[rsp+0x6180]
  634338:	lea    r14,[rsp+0x3980]
  634340:	mov    edx,0x800
  634345:	mov    rsi,r14
  634348:	call   rbx
  63434a:	lea    rdi,[rsp+0x8980]
  634352:	mov    r12d,0x1000
  634358:	mov    edx,0x1000
  63435d:	mov    rsi,r15
  634360:	call   rbx
  634362:	lea    rdi,[rsp+0x180]
  63436a:	mov    r15,QWORD PTR [rip+0xba2b7]        # 6ee628 <memset@GLIBC_2.2.5>
  634371:	mov    edx,0x800
  634376:	xor    esi,esi
  634378:	call   r15
  63437b:	lea    rdi,[rsp+0x980]
  634383:	mov    edx,0x800
  634388:	xor    esi,esi
  63438a:	call   r15
  63438d:	lea    rdi,[rsp+0x5180]
  634395:	mov    edx,0x800
  63439a:	mov    rsi,r14
  63439d:	call   rbx
  63439f:	nop
  6343a0:	vmovups zmm0,ZMMWORD PTR [rsp+r12*1-0xe80]
  6343a8:	vmovdqu64 zmm1,ZMMWORD PTR [rsp+r12*1-0xe40]
  6343b0:	vxorps zmm0,zmm0,ZMMWORD PTR [rsp+r12*1+0x7980]
  6343bb:	vmovups ZMMWORD PTR [rsp+r12*1-0xe80],zmm0
  6343c3:	vmovups zmm0,ZMMWORD PTR [rsp+r12*1-0x680]
  6343cb:	vmovdqu64 zmm2,ZMMWORD PTR [rsp+r12*1-0x640]
  6343d3:	vxorps zmm0,zmm0,ZMMWORD PTR [rsp+r12*1+0x4180]
  6343de:	vmovups ZMMWORD PTR [rsp+r12*1-0x680],zmm0
  6343e6:	vpxorq zmm0,zmm1,ZMMWORD PTR [rsp+r12*1+0x79c0]
  6343f1:	vmovdqu64 ZMMWORD PTR [rsp+r12*1-0xe40],zmm0
  6343f9:	vpxorq zmm0,zmm2,ZMMWORD PTR [rsp+r12*1+0x41c0]
  634404:	vmovdqu64 ZMMWORD PTR [rsp+r12*1-0x640],zmm0
  63440c:	sub    r12,0xffffffffffffff80
  634410:	cmp    r12,0x1800
  634417:	jne    6343a0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0xfc0>
  634419:	lea    r14,[rsp+0x6980]
  634421:	lea    rsi,[rsp+0x180]
  634429:	mov    edx,0x800
  63442e:	mov    rdi,r14
  634431:	vzeroupper 
  634434:	call   rbx
  634436:	lea    rdi,[rsp+0x7180]
  63443e:	lea    rsi,[rsp+0x980]
  634446:	mov    edx,0x800
  63444b:	call   rbx
  63444d:	mov    edx,0x1000
  634452:	mov    rdi,QWORD PTR [rsp+0x58]
  634457:	mov    rsi,r14
  63445a:	jmp    63475b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x137b>
  63445f:	test   dl,dl
  634461:	je     63447b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x109b>
  634463:	mov    rax,0xffffffffffffffa8
  63446a:	mov    rax,QWORD PTR fs:[rax]
  63446e:	test   rax,rax
  634471:	je     6344a8 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x10c8>
  634473:	add    rax,0x110
  634479:	jmp    6344c0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x10e0>
  63447b:	test   rcx,rcx
  63447e:	je     633449 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x69>
  634484:	shr    rcx,1
  634487:	jmp    6344d4 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x10f4>
  634489:	mov    rdi,rdx
  63448c:	lea    rsi,[rdx+0x80]
  634493:	lea    rcx,[rip+0xb5f2e]        # 6ea3c8 <anon.01aa214378fce5a476fd64886db4a5c0.1334.llvm.5168012589948291949+0x210>
  63449a:	mov    rdx,QWORD PTR [rsp+0x30]
  63449f:	vzeroupper 
  6344a2:	call   QWORD PTR [rip+0xba298]        # 6ee740 <_DYNAMIC+0x340>
  6344a8:	mov    rbp,r14
  6344ab:	mov    r14,rsi
  6344ae:	mov    r15,rcx
  6344b1:	call   QWORD PTR [rip+0xba511]        # 6ee9c8 <_DYNAMIC+0x5c8>
  6344b7:	mov    rcx,r15
  6344ba:	mov    rsi,r14
  6344bd:	mov    r14,rbp
  6344c0:	mov    rax,QWORD PTR [rax]
  6344c3:	mov    rax,QWORD PTR [rax+0x208]
  6344ca:	shr    rcx,1
  6344cd:	cmp    rcx,rax
  6344d0:	cmovbe rcx,rax
  6344d4:	mov    QWORD PTR [rsp+0xc8],rcx
  6344dc:	mov    QWORD PTR [rsp+0x5180],rsi
  6344e4:	xor    eax,eax
  6344e6:	mov    rcx,r12
  6344e9:	sub    rcx,r14
  6344ec:	cmovae rax,rcx
  6344f0:	cmp    rsi,rax
  6344f3:	ja     6347ad <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x13cd>
  6344f9:	add    rsi,r14
  6344fc:	mov    rax,QWORD PTR [rbx]
  6344ff:	mov    rcx,QWORD PTR [rbx+0x8]
  634503:	mov    rdx,QWORD PTR [rbx+0x10]
  634507:	mov    rdi,QWORD PTR [rbx+0x18]
  63450b:	mov    r8,QWORD PTR [rbx+0x20]
  63450f:	lea    r9,[rsp+0xc0]
  634517:	mov    QWORD PTR [rsp+0xd8],r9
  63451f:	lea    r9,[rsp+0x5180]
  634527:	mov    QWORD PTR [rsp+0xe0],r9
  63452f:	lea    r10,[rsp+0xc8]
  634537:	mov    QWORD PTR [rsp+0xe8],r10
  63453f:	mov    QWORD PTR [rsp+0xf0],rax
  634547:	mov    QWORD PTR [rsp+0xf8],rcx
  63454f:	mov    QWORD PTR [rsp+0x100],rdx
  634557:	mov    QWORD PTR [rsp+0x108],rdi
  63455f:	mov    QWORD PTR [rsp+0x110],r8
  634567:	mov    QWORD PTR [rsp+0x118],rsi
  63456f:	mov    QWORD PTR [rsp+0x120],r12
  634577:	mov    QWORD PTR [rsp+0x128],r9
  63457f:	mov    QWORD PTR [rsp+0x130],r10
  634587:	mov    QWORD PTR [rsp+0x138],rax
  63458f:	mov    QWORD PTR [rsp+0x140],rcx
  634597:	mov    QWORD PTR [rsp+0x148],rdx
  63459f:	mov    QWORD PTR [rsp+0x150],rdi
  6345a7:	mov    QWORD PTR [rsp+0x158],r8
  6345af:	mov    QWORD PTR [rsp+0x160],r14
  6345b7:	mov    QWORD PTR [rsp+0x168],rsi
  6345bf:	mov    rbx,0xffffffffffffffa8
  6345c6:	mov    rdx,QWORD PTR fs:[rbx]
  6345ca:	test   rdx,rdx
  6345cd:	jne    6345f2 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x1212>
  6345cf:	call   QWORD PTR [rip+0xba3f3]        # 6ee9c8 <_DYNAMIC+0x5c8>
  6345d5:	mov    rsi,QWORD PTR [rax]
  6345d8:	mov    rdx,QWORD PTR fs:[rbx]
  6345dc:	test   rdx,rdx
  6345df:	je     6347c6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x13e6>
  6345e5:	cmp    QWORD PTR [rdx+0x110],rsi
  6345ec:	jne    6347e4 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x1404>
  6345f2:	lea    rdi,[rsp+0x1180]
  6345fa:	lea    rsi,[rsp+0xd8]
  634602:	xor    ecx,ecx
  634604:	call   635920 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_4fold12FoldConsumerINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB3X_s1_0ENCB3X_s0_0ENvMB3Z_NtB3Z_6Worker3newNCB3X_s_0EE0NCBR_s_0TANtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128j80_B75_EB74_E0B45_>
  634609:	lea    r14,[rsp+0xa980]
  634611:	lea    rsi,[rsp+0x1180]
  634619:	mov    r12d,0x1000
  63461f:	mov    rbx,QWORD PTR [rip+0xb9ff2]        # 6ee618 <memcpy@GLIBC_2.14>
  634626:	mov    edx,0x1000
  63462b:	mov    rdi,r14
  63462e:	call   rbx
  634630:	lea    rsi,[rsp+0x2180]
  634638:	lea    r15,[rsp+0xb980]
  634640:	mov    edx,0x1000
  634645:	mov    rdi,r15
  634648:	call   rbx
  63464a:	lea    rdi,[rsp+0x2180]
  634652:	mov    edx,0x1000
  634657:	mov    rsi,r15
  63465a:	call   rbx
  63465c:	lea    rdi,[rsp+0x5980]
  634664:	mov    edx,0x800
  634669:	mov    rsi,r14
  63466c:	call   rbx
  63466e:	lea    rsi,[rsp+0xb180]
  634676:	lea    rdi,[rsp+0x6980]
  63467e:	mov    edx,0x800
  634683:	call   rbx
  634685:	lea    rsi,[rsp+0xc180]
  63468d:	lea    rdi,[rsp+0x7980]
  634695:	mov    edx,0x800
  63469a:	call   rbx
  63469c:	nop    DWORD PTR [rax+0x0]
  6346a0:	vmovups zmm0,ZMMWORD PTR [rsp+r12*1+0x4980]
  6346ab:	vmovdqu64 zmm1,ZMMWORD PTR [rsp+r12*1+0x49c0]
  6346b6:	vxorps zmm0,zmm0,ZMMWORD PTR [rsp+r12*1+0x1180]
  6346be:	vmovups ZMMWORD PTR [rsp+r12*1+0x4980],zmm0
  6346c9:	vmovups zmm0,ZMMWORD PTR [rsp+r12*1+0x5980]
  6346d4:	vmovdqu64 zmm2,ZMMWORD PTR [rsp+r12*1+0x59c0]
  6346df:	vxorps zmm0,zmm0,ZMMWORD PTR [rsp+r12*1+0x6980]
  6346ea:	vmovups ZMMWORD PTR [rsp+r12*1+0x5980],zmm0
  6346f5:	vpxorq zmm0,zmm1,ZMMWORD PTR [rsp+r12*1+0x11c0]
  6346fd:	vmovdqu64 ZMMWORD PTR [rsp+r12*1+0x49c0],zmm0
  634708:	vpxorq zmm0,zmm2,ZMMWORD PTR [rsp+r12*1+0x69c0]
  634713:	vmovdqu64 ZMMWORD PTR [rsp+r12*1+0x59c0],zmm0
  63471e:	sub    r12,0xffffffffffffff80
  634722:	cmp    r12,0x1800
  634729:	jne    6346a0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x12c0>
  63472f:	lea    rsi,[rsp+0x5980]
  634737:	mov    edx,0x800
  63473c:	mov    rdi,r13
  63473f:	vzeroupper 
  634742:	call   rbx
  634744:	add    r13,0x800
  63474b:	lea    rsi,[rsp+0x6980]
  634753:	mov    edx,0x800
  634758:	mov    rdi,r13
  63475b:	call   rbx
  63475d:	add    rsp,0xc988
  634764:	pop    rbx
  634765:	pop    r12
  634767:	pop    r13
  634769:	pop    r14
  63476b:	pop    r15
  63476d:	pop    rbp
  63476e:	ret    
  63476f:	lea    rdi,[rbx+rsi*1+0x100]
  634777:	lea    rdx,[rip+0xb6622]        # 6eada0 <anon.01aa214378fce5a476fd64886db4a5c0.2593.llvm.5168012589948291949+0x5e8>
  63477e:	mov    rsi,rbp
  634781:	vzeroupper 
  634784:	call   QWORD PTR [rip+0xb9ec6]        # 6ee650 <_DYNAMIC+0x250>
  63478a:	lea    rax,[rbp+rsi*1+0x100]
  634792:	mov    rbp,QWORD PTR [rsp+0x18]
  634797:	lea    rdx,[rip+0xb661a]        # 6eadb8 <anon.01aa214378fce5a476fd64886db4a5c0.2593.llvm.5168012589948291949+0x600>
  63479e:	mov    rdi,rax
  6347a1:	mov    rsi,rbp
  6347a4:	vzeroupper 
  6347a7:	call   QWORD PTR [rip+0xb9ea3]        # 6ee650 <_DYNAMIC+0x250>
  6347ad:	lea    rdi,[rip+0xffffffffff9de294]        # 12a48 <anon.b29a6a3dffb4aad81a99129e80bda2af.11.llvm.12663359458273545214>
  6347b4:	lea    rdx,[rip+0xb769d]        # 6ebe58 <anon.b29a6a3dffb4aad81a99129e80bda2af.16.llvm.12663359458273545214>
  6347bb:	mov    esi,0x2b
  6347c0:	call   QWORD PTR [rip+0xb9f82]        # 6ee748 <_DYNAMIC+0x348>
  6347c6:	sub    rsi,0xffffffffffffff80
  6347ca:	lea    rdi,[rsp+0x1180]
  6347d2:	lea    rdx,[rsp+0xd8]
  6347da:	call   627850 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1P_5range12IterProducerjEINtNtB1N_4fold12FoldConsumerINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB4P_s1_0ENCB4P_s0_0ENvMB4R_NtB4R_6Worker3newNCB4P_s_0EE0NCB1G_s_0TANtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128j80_B7Y_EB7X_E0TB7X_B7X_EEB4X_>
  6347df:	jmp    634609 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x1229>
  6347e4:	sub    rsi,0xffffffffffffff80
  6347e8:	lea    rdi,[rsp+0x1180]
  6347f0:	lea    rcx,[rsp+0xd8]
  6347f8:	call   628720 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1Q_5range12IterProducerjEINtNtB1O_4fold12FoldConsumerINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB4Q_s1_0ENCB4Q_s0_0ENvMB4S_NtB4S_6Worker3newNCB4Q_s_0EE0NCB1H_s_0TANtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128j80_B7Z_EB7Y_E0TB7Y_B7Y_EEB4Y_>
  6347fd:	jmp    634609 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x1229>
  634802:	lea    rdx,[rip+0xb5ba7]        # 6ea3b0 <anon.01aa214378fce5a476fd64886db4a5c0.1334.llvm.5168012589948291949+0x1f8>
  634809:	vzeroupper 
  63480c:	call   QWORD PTR [rip+0xb9e3e]        # 6ee650 <_DYNAMIC+0x250>
  634812:	int3   
  634813:	int3   
  634814:	int3   
  634815:	int3   
  634816:	int3   
  634817:	int3   
  634818:	int3   
  634819:	int3   
  63481a:	int3   
  63481b:	int3   
  63481c:	int3   
  63481d:	int3   
  63481e:	int3   
  63481f:	int3   

