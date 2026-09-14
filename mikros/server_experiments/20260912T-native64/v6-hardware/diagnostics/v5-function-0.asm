000000000062e960 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_>:
  62e960:	push   rbp
  62e961:	push   r15
  62e963:	push   r14
  62e965:	push   r13
  62e967:	push   r12
  62e969:	push   rbx
  62e96a:	mov    r11,rsp
  62e96d:	sub    r11,0xc000
  62e974:	sub    rsp,0x1000
  62e97b:	mov    QWORD PTR [rsp],0x0
  62e983:	cmp    rsp,r11
  62e986:	jne    62e974 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x14>
  62e988:	sub    rsp,0x988
  62e98f:	mov    r14,r9
  62e992:	mov    r13,rdi
  62e995:	mov    rbx,QWORD PTR [rsp+0xc9c8]
  62e99d:	mov    r12,QWORD PTR [rsp+0xc9c0]
  62e9a5:	mov    QWORD PTR [rsp+0xc0],rsi
  62e9ad:	mov    QWORD PTR [rsp+0xc8],rcx
  62e9b5:	mov    QWORD PTR [rsp+0xd0],r8
  62e9bd:	shr    rsi,1
  62e9c0:	cmp    rsi,r8
  62e9c3:	jae    62f9df <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x107f>
  62e9c9:	mov    QWORD PTR [rsp+0x58],r13
  62e9ce:	mov    rbx,QWORD PTR [rbx+0x18]
  62e9d2:	lea    rdi,[rsp+0x7980]
  62e9da:	mov    edx,0x3000
  62e9df:	xor    esi,esi
  62e9e1:	call   QWORD PTR [rip+0xb8811]        # 6e71f8 <memset@GLIBC_2.2.5>
  62e9e7:	cmp    r14,r12
  62e9ea:	jae    62f878 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0xf18>
  62e9f0:	mov    rdi,r14
  62e9f3:	movabs r12,0x7fffffffffffffc0
  62e9fd:	mov    rax,QWORD PTR [rbx]
  62ea00:	mov    QWORD PTR [rsp+0x20],rax
  62ea05:	mov    rax,QWORD PTR [rbx+0x8]
  62ea09:	mov    rcx,QWORD PTR [rbx+0x10]
  62ea0d:	mov    QWORD PTR [rsp+0x40],rcx
  62ea12:	mov    rcx,QWORD PTR [rbx+0x20]
  62ea16:	mov    QWORD PTR [rsp+0x38],rcx
  62ea1b:	mov    rcx,QWORD PTR [rbx+0x30]
  62ea1f:	mov    QWORD PTR [rsp+0x88],rcx
  62ea27:	mov    rcx,QWORD PTR [rbx+0x38]
  62ea2b:	mov    QWORD PTR [rsp+0x30],rcx
  62ea30:	mov    rcx,QWORD PTR [rbx+0x40]
  62ea34:	mov    rdx,QWORD PTR [rbx+0x48]
  62ea38:	mov    QWORD PTR [rsp+0x48],rdx
  62ea3d:	mov    rbp,QWORD PTR [rbx+0x50]
  62ea41:	mov    rdx,QWORD PTR [rax+0x8]
  62ea45:	mov    r8,QWORD PTR [rax+0x10]
  62ea49:	mov    r13,QWORD PTR [rcx+0x8]
  62ea4d:	add    r13,QWORD PTR [rcx+0x48]
  62ea51:	mov    eax,0x87
  62ea56:	vmovq  xmm0,rax
  62ea5b:	vmovdqa XMMWORD PTR [rsp+0x170],xmm0
  62ea64:	mov    rax,r8
  62ea67:	shl    rax,0x4
  62ea6b:	mov    QWORD PTR [rsp+0x50],rdx
  62ea70:	add    rax,rdx
  62ea73:	mov    QWORD PTR [rsp+0x68],rax
  62ea78:	lea    r14,[rsp+0x1180]
  62ea80:	lea    rsi,[rsp+0x7980]
  62ea88:	mov    rax,QWORD PTR [rip+0xb8759]        # 6e71e8 <memcpy@GLIBC_2.14>
  62ea8f:	lea    r15,[rip+0xffffffffff9e0aea]        # f580 <_RNvNvNtCstrZ1g5xv8S_15crossbeam_epoch5guard11unprotected11UNPROTECTED+0x540>
  62ea96:	mov    QWORD PTR [rsp+0x60],r8
  62ea9b:	mov    QWORD PTR [rsp+0x18],rbp
  62eaa0:	mov    edx,0x3000
  62eaa5:	mov    rbx,rdi
  62eaa8:	mov    rdi,r14
  62eaab:	call   rax
  62eaad:	mov    QWORD PTR [rsp+0x28],rbx
  62eab2:	mov    QWORD PTR [rsp+0x4180],rbx
  62eaba:	mov    edx,0x1000
  62eabf:	mov    rdi,r14
  62eac2:	xor    esi,esi
  62eac4:	call   QWORD PTR [rip+0xb872e]        # 6e71f8 <memset@GLIBC_2.2.5>
  62eaca:	cmp    QWORD PTR [rsp+0x60],0x0
  62ead0:	vmovdqa64 zmm8,ZMMWORD PTR [rip+0xffffffffff9e1026]        # fb00 <anon.01aa214378fce5a476fd64886db4a5c0.2569.llvm.3043956165524440405+0x1f9>
  62eada:	vmovdqa64 zmm9,ZMMWORD PTR [rip+0xffffffffff9e105c]        # fb40 <anon.01aa214378fce5a476fd64886db4a5c0.2569.llvm.3043956165524440405+0x239>
  62eae4:	vmovdqa xmm10,XMMWORD PTR [rsp+0x170]
  62eaed:	vpbroadcastq zmm11,QWORD PTR [rip+0xffffffffff9ddf91]        # ca88 <anon.36d21461a6996b7f2bd9a76e3225918f.29.llvm.7983859039285366149+0x108>
  62eaf7:	vmovdqa64 zmm12,ZMMWORD PTR [rip+0xffffffffff9e073f]        # f240 <_RNvNvNtCstrZ1g5xv8S_15crossbeam_epoch5guard11unprotected11UNPROTECTED+0x200>
  62eb01:	vmovdqa64 zmm13,ZMMWORD PTR [rip+0xffffffffff9e06f5]        # f200 <_RNvNvNtCstrZ1g5xv8S_15crossbeam_epoch5guard11unprotected11UNPROTECTED+0x1c0>
  62eb0b:	lea    r14,[rip+0xffffffffff9e0a7e]        # f590 <_RNvNvNtCstrZ1g5xv8S_15crossbeam_epoch5guard11unprotected11UNPROTECTED+0x550>
  62eb12:	je     62f4dc <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0xb7c>
  62eb18:	mov    rax,QWORD PTR [rsp+0x20]
  62eb1d:	movzx  eax,BYTE PTR [rax+0x30]
  62eb21:	shlx   rax,QWORD PTR [rsp+0x28],rax
  62eb28:	mov    QWORD PTR [rsp+0x70],rax
  62eb2d:	mov    rcx,QWORD PTR [rsp+0x50]
  62eb32:	xor    eax,eax
  62eb34:	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  62eb40:	mov    rdx,rax
  62eb43:	mov    QWORD PTR [rsp+0x90],rcx
  62eb4b:	lea    rax,[rcx+0x10]
  62eb4f:	mov    QWORD PTR [rsp+0x78],rax
  62eb54:	lea    rax,[rdx+0x1]
  62eb58:	mov    QWORD PTR [rsp+0x80],rax
  62eb60:	or     rdx,QWORD PTR [rsp+0x70]
  62eb65:	shl    rdx,0xb
  62eb69:	mov    rax,QWORD PTR [rsp+0x40]
  62eb6e:	lea    r10,[rax+rdx*1]
  62eb72:	mov    rax,QWORD PTR [rsp+0x38]
  62eb77:	mov    QWORD PTR [rsp+0x98],rdx
  62eb7f:	lea    r11,[rax+rdx*1]
  62eb83:	xor    eax,eax
  62eb85:	data16 cs nop WORD PTR [rax+rax*1+0x0]
  62eb90:	mov    rcx,rax
  62eb93:	inc    rax
  62eb96:	mov    QWORD PTR [rsp+0xa8],rax
  62eb9e:	mov    rdx,rcx
  62eba1:	shl    rdx,0x7
  62eba5:	add    rdx,QWORD PTR [rsp+0x98]
  62ebad:	shl    rcx,0x6
  62ebb1:	lea    rax,[rsp+0x2180]
  62ebb9:	add    rax,rcx
  62ebbc:	mov    QWORD PTR [rsp+0x10],rax
  62ebc1:	lea    rax,[rsp+0x2580]
  62ebc9:	mov    QWORD PTR [rsp+0xa0],rcx
  62ebd1:	add    rax,rcx
  62ebd4:	mov    QWORD PTR [rsp+0x8],rax
  62ebd9:	vpxor  xmm0,xmm0,xmm0
  62ebdd:	mov    QWORD PTR [rsp+0xb0],r11
  62ebe5:	mov    QWORD PTR [rsp+0xb8],r10
  62ebed:	vpxor  xmm1,xmm1,xmm1
  62ebf1:	xor    edi,edi
  62ebf3:	mov    QWORD PTR [rsp],rdx
  62ebf7:	jmp    62ec1e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x2be>
  62ebf9:	nop    DWORD PTR [rax+0x0]
  62ec00:	vpxord zmm1,zmm1,zmm3
  62ec06:	vpxord zmm0,zmm0,zmm2
  62ec0c:	add    r10,0x10
  62ec10:	add    r11,0x10
  62ec14:	cmp    rdi,0x8
  62ec18:	je     62ef50 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x5f0>
  62ec1e:	mov    r9,rdi
  62ec21:	inc    rdi
  62ec24:	mov    rcx,r9
  62ec27:	shl    rcx,0x4
  62ec2b:	add    rcx,rdx
  62ec2e:	mov    rax,QWORD PTR [rsp+0x40]
  62ec33:	movzx  eax,BYTE PTR [rax+rcx*1]
  62ec37:	shl    eax,0x7
  62ec3a:	vmovdqu64 zmm3,ZMMWORD PTR [r13+rax*1+0x0]
  62ec42:	vmovdqu64 zmm2,ZMMWORD PTR [r13+rax*1+0x40]
  62ec4a:	mov    eax,0x1
  62ec4f:	mov    esi,0x8
  62ec54:	jmp    62ec92 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x332>
  62ec56:	vpermq zmm4,zmm12,zmm4
  62ec5c:	xor    r8,0x40
  62ec60:	vmovdqu64 zmm5,ZMMWORD PTR [rdx+r8*1]
  62ec67:	vshufi64x2 zmm5,zmm5,zmm5,0x4e
  62ec6e:	vpxord zmm3,zmm4,zmm3
  62ec74:	vpshufd zmm4,zmm5,0x4e
  62ec7b:	vpxord zmm2,zmm4,zmm2
  62ec81:	inc    rax
  62ec84:	add    rsi,0x8
  62ec88:	cmp    rax,0x10
  62ec8c:	je     62ed90 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x430>
  62ec92:	movzx  edx,BYTE PTR [r10+rax*1]
  62ec97:	shl    edx,0x7
  62ec9a:	add    rdx,r13
  62ec9d:	mov    ebx,eax
  62ec9f:	shr    ebx,1
  62eca1:	and    ebx,0x3
  62eca4:	mov    r8,rsi
  62eca7:	and    r8,r12
  62ecaa:	vmovdqu64 zmm4,ZMMWORD PTR [r8+rdx*1]
  62ecb1:	test   al,0x1
  62ecb3:	jne    62ecd0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x370>
  62ecb5:	movsxd rbx,DWORD PTR [r14+rbx*4]
  62ecb9:	add    rbx,r14
  62ecbc:	jmp    rbx
  62ecbe:	xor    r8,0x40
  62ecc2:	vmovdqu64 zmm5,ZMMWORD PTR [rdx+r8*1]
  62ecc9:	jmp    62ed56 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x3f6>
  62ecce:	xchg   ax,ax
  62ecd0:	movsxd rbx,DWORD PTR [r15+rbx*4]
  62ecd4:	add    rbx,r15
  62ecd7:	jmp    rbx
  62ecd9:	vpshufd zmm4,zmm4,0x4e
  62ece0:	xor    r8,0x40
  62ece4:	vmovdqu64 zmm5,ZMMWORD PTR [rdx+r8*1]
  62eceb:	jmp    62ec6e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x30e>
  62ecf0:	vshufi64x2 zmm4,zmm4,zmm4,0x4e
  62ecf7:	xor    r8,0x40
  62ecfb:	vmovdqu64 zmm5,ZMMWORD PTR [rdx+r8*1]
  62ed02:	vshufi64x2 zmm5,zmm5,zmm5,0x4e
  62ed09:	jmp    62ed56 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x3f6>
  62ed0b:	vshufi64x2 zmm4,zmm4,zmm4,0x1b
  62ed12:	xor    r8,0x40
  62ed16:	vmovdqu64 zmm5,ZMMWORD PTR [rdx+r8*1]
  62ed1d:	vshufi64x2 zmm5,zmm5,zmm5,0x1b
  62ed24:	jmp    62ed56 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x3f6>
  62ed26:	vpermq zmm4,zmm13,zmm4
  62ed2c:	xor    r8,0x40
  62ed30:	vmovdqu64 zmm5,ZMMWORD PTR [rdx+r8*1]
  62ed37:	vshufi64x2 zmm5,zmm5,zmm5,0x1b
  62ed3e:	jmp    62ec6e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x30e>
  62ed43:	vpermq zmm4,zmm4,0x4e
  62ed4a:	xor    r8,0x40
  62ed4e:	vpermq zmm5,ZMMWORD PTR [rdx+r8*1],0x4e
  62ed56:	vpxord zmm3,zmm3,zmm4
  62ed5c:	vpxord zmm2,zmm2,zmm5
  62ed62:	inc    rax
  62ed65:	add    rsi,0x8
  62ed69:	cmp    rax,0x10
  62ed6d:	jne    62ec92 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x332>
  62ed73:	jmp    62ed90 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x430>
  62ed75:	vpermq zmm4,zmm4,0x1b
  62ed7c:	xor    r8,0x40
  62ed80:	vpermq zmm5,ZMMWORD PTR [rdx+r8*1],0x4e
  62ed88:	jmp    62ec6e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x30e>
  62ed8d:	nop    DWORD PTR [rax]
  62ed90:	mov    rax,QWORD PTR [rsp+0x38]
  62ed95:	movzx  eax,BYTE PTR [rax+rcx*1]
  62ed99:	shl    eax,0x7
  62ed9c:	vmovdqu64 zmm5,ZMMWORD PTR [r13+rax*1+0x0]
  62eda4:	vmovdqu64 zmm4,ZMMWORD PTR [r13+rax*1+0x40]
  62edac:	mov    eax,0x1
  62edb1:	mov    ecx,0x8
  62edb6:	jmp    62edf4 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x494>
  62edb8:	vpermq zmm6,zmm12,zmm6
  62edbe:	xor    rsi,0x40
  62edc2:	vmovdqu64 zmm7,ZMMWORD PTR [rdx+rsi*1]
  62edc9:	vshufi64x2 zmm7,zmm7,zmm7,0x4e
  62edd0:	vpxord zmm5,zmm6,zmm5
  62edd6:	vpshufd zmm6,zmm7,0x4e
  62eddd:	vpxord zmm4,zmm6,zmm4
  62ede3:	inc    rax
  62ede6:	add    rcx,0x8
  62edea:	cmp    rax,0x10
  62edee:	je     62ef10 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x5b0>
  62edf4:	movzx  edx,BYTE PTR [r11+rax*1]
  62edf9:	shl    edx,0x7
  62edfc:	add    rdx,r13
  62edff:	mov    r8d,eax
  62ee02:	shr    r8d,1
  62ee05:	and    r8d,0x3
  62ee09:	mov    rsi,rcx
  62ee0c:	and    rsi,r12
  62ee0f:	vmovdqu64 zmm6,ZMMWORD PTR [rsi+rdx*1]
  62ee16:	test   al,0x1
  62ee18:	jne    62ee40 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x4e0>
  62ee1a:	lea    rbx,[rip+0xffffffffff9e078f]        # f5b0 <_RNvNvNtCstrZ1g5xv8S_15crossbeam_epoch5guard11unprotected11UNPROTECTED+0x570>
  62ee21:	movsxd r8,DWORD PTR [rbx+r8*4]
  62ee25:	add    r8,rbx
  62ee28:	jmp    r8
  62ee2b:	xor    rsi,0x40
  62ee2f:	vmovdqu64 zmm7,ZMMWORD PTR [rdx+rsi*1]
  62ee36:	jmp    62eece <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x56e>
  62ee3b:	nop    DWORD PTR [rax+rax*1+0x0]
  62ee40:	lea    rbx,[rip+0xffffffffff9e0759]        # f5a0 <_RNvNvNtCstrZ1g5xv8S_15crossbeam_epoch5guard11unprotected11UNPROTECTED+0x560>
  62ee47:	movsxd r8,DWORD PTR [rbx+r8*4]
  62ee4b:	add    r8,rbx
  62ee4e:	jmp    r8
  62ee51:	vpshufd zmm6,zmm6,0x4e
  62ee58:	xor    rsi,0x40
  62ee5c:	vmovdqu64 zmm7,ZMMWORD PTR [rdx+rsi*1]
  62ee63:	jmp    62edd0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x470>
  62ee68:	vshufi64x2 zmm6,zmm6,zmm6,0x4e
  62ee6f:	xor    rsi,0x40
  62ee73:	vmovdqu64 zmm7,ZMMWORD PTR [rdx+rsi*1]
  62ee7a:	vshufi64x2 zmm7,zmm7,zmm7,0x4e
  62ee81:	jmp    62eece <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x56e>
  62ee83:	vshufi64x2 zmm6,zmm6,zmm6,0x1b
  62ee8a:	xor    rsi,0x40
  62ee8e:	vmovdqu64 zmm7,ZMMWORD PTR [rdx+rsi*1]
  62ee95:	vshufi64x2 zmm7,zmm7,zmm7,0x1b
  62ee9c:	jmp    62eece <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x56e>
  62ee9e:	vpermq zmm6,zmm13,zmm6
  62eea4:	xor    rsi,0x40
  62eea8:	vmovdqu64 zmm7,ZMMWORD PTR [rdx+rsi*1]
  62eeaf:	vshufi64x2 zmm7,zmm7,zmm7,0x1b
  62eeb6:	jmp    62edd0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x470>
  62eebb:	vpermq zmm6,zmm6,0x4e
  62eec2:	xor    rsi,0x40
  62eec6:	vpermq zmm7,ZMMWORD PTR [rdx+rsi*1],0x4e
  62eece:	vpxord zmm5,zmm5,zmm6
  62eed4:	vpxord zmm4,zmm4,zmm7
  62eeda:	inc    rax
  62eedd:	add    rcx,0x8
  62eee1:	cmp    rax,0x10
  62eee5:	jne    62edf4 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x494>
  62eeeb:	jmp    62ef10 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x5b0>
  62eeed:	vpermq zmm6,zmm6,0x1b
  62eef4:	xor    rsi,0x40
  62eef8:	vpermq zmm7,ZMMWORD PTR [rdx+rsi*1],0x4e
  62ef00:	jmp    62edd0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x470>
  62ef05:	data16 cs nop WORD PTR [rax+rax*1+0x0]
  62ef10:	vgf2p8mulb zmm3,zmm3,zmm5
  62ef16:	vgf2p8mulb zmm2,zmm2,zmm4
  62ef1c:	test   r9,r9
  62ef1f:	mov    rdx,QWORD PTR [rsp]
  62ef23:	je     62ec00 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x2a0>
  62ef29:	mov    al,0x1
  62ef2b:	mov    ecx,r9d
  62ef2e:	shl    al,cl
  62ef30:	vpbroadcastb zmm4,eax
  62ef36:	vgf2p8mulb zmm3,zmm3,zmm4
  62ef3c:	vgf2p8mulb zmm2,zmm2,zmm4
  62ef42:	jmp    62ec00 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x2a0>
  62ef47:	nop    WORD PTR [rax+rax*1+0x0]
  62ef50:	mov    rax,QWORD PTR [rsp+0x10]
  62ef55:	vmovdqu64 ZMMWORD PTR [rax],zmm1
  62ef5b:	mov    rax,QWORD PTR [rsp+0x8]
  62ef60:	vmovdqu64 ZMMWORD PTR [rax],zmm0
  62ef66:	mov    rax,rdx
  62ef69:	or     rax,0x7f
  62ef6d:	cmp    rax,QWORD PTR [rsp+0x30]
  62ef72:	jae    62fa09 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x10a9>
  62ef78:	mov    rax,QWORD PTR [rsp+0x88]
  62ef80:	vmovdqu64 zmm0,ZMMWORD PTR [rax+rdx*1]
  62ef87:	vmovdqu64 zmm1,ZMMWORD PTR [rax+rdx*1+0x40]
  62ef8f:	vmovdqa64 zmm2,zmm0
  62ef95:	vpermt2b zmm2,zmm8,zmm1
  62ef9b:	vgf2p8affineqb zmm2,zmm11,zmm2,0x0
  62efa2:	vpermt2b zmm0,zmm9,zmm1
  62efa8:	lea    rax,[rsp+0x1580]
  62efb0:	mov    rcx,QWORD PTR [rsp+0xa0]
  62efb8:	vmovdqu64 ZMMWORD PTR [rax+rcx*1+0x1400],zmm2
  62efc0:	vgf2p8affineqb zmm0,zmm11,zmm0,0x0
  62efc7:	vmovdqu64 ZMMWORD PTR [rax+rcx*1+0x1800],zmm0
  62efcf:	mov    r10,QWORD PTR [rsp+0xb8]
  62efd7:	sub    r10,0xffffffffffffff80
  62efdb:	mov    r11,QWORD PTR [rsp+0xb0]
  62efe3:	sub    r11,0xffffffffffffff80
  62efe7:	mov    rax,QWORD PTR [rsp+0xa8]
  62efef:	cmp    rax,0x10
  62eff3:	jne    62eb90 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x230>
  62eff9:	lea    rcx,[rsp+0x29c0]
  62f001:	xor    eax,eax
  62f003:	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  62f010:	mov    QWORD PTR [rsp+0x10],rax
  62f015:	inc    rax
  62f018:	mov    QWORD PTR [rsp+0x8],rax
  62f01d:	mov    QWORD PTR [rsp],rcx
  62f021:	xor    esi,esi
  62f023:	xor    r8d,r8d
  62f026:	xor    r11d,r11d
  62f029:	xor    r10d,r10d
  62f02c:	xor    r9d,r9d
  62f02f:	nop
  62f030:	movzx  edi,BYTE PTR [rcx-0x840]
  62f037:	add    rdi,rsi
  62f03a:	cmp    rdi,rbp
  62f03d:	jae    62fcf7 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x1397>
  62f043:	movzx  eax,BYTE PTR [rcx-0x40]
  62f047:	add    rax,rsi
  62f04a:	cmp    rax,rbp
  62f04d:	jae    62fd17 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x13b7>
  62f053:	movzx  ebx,BYTE PTR [rcx-0x800]
  62f05a:	lea    rdx,[rsi+rbx*1+0x100]
  62f062:	cmp    rdx,rbp
  62f065:	jae    62fcef <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x138f>
  62f06b:	mov    r14,rbp
  62f06e:	movzx  ebp,BYTE PTR [rcx]
  62f071:	lea    rbx,[rsi+rbp*1+0x100]
  62f079:	cmp    rbx,r14
  62f07c:	jae    62fd0a <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x13aa>
  62f082:	shl    rdi,0x4
  62f086:	mov    r14,QWORD PTR [rsp+0x48]
  62f08b:	xor    r11,QWORD PTR [r14+rdi*1+0x8]
  62f090:	xor    r10,QWORD PTR [r14+rdi*1]
  62f094:	shl    rax,0x4
  62f098:	xor    r9,QWORD PTR [r14+rax*1]
  62f09c:	xor    r8,QWORD PTR [r14+rax*1+0x8]
  62f0a1:	shl    rdx,0x4
  62f0a5:	xor    r11,QWORD PTR [r14+rdx*1+0x8]
  62f0aa:	xor    r10,QWORD PTR [r14+rdx*1]
  62f0ae:	shl    rbx,0x4
  62f0b2:	xor    r9,QWORD PTR [r14+rbx*1]
  62f0b6:	xor    r8,QWORD PTR [r14+rbx*1+0x8]
  62f0bb:	add    rsi,0x200
  62f0c2:	sub    rcx,0xffffffffffffff80
  62f0c6:	cmp    rsi,0x1000
  62f0cd:	mov    rbp,QWORD PTR [rsp+0x18]
  62f0d2:	jne    62f030 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x6d0>
  62f0d8:	mov    rdx,QWORD PTR [rsp+0x10]
  62f0dd:	shl    rdx,0x4
  62f0e1:	vmovq  xmm1,r10
  62f0e6:	mov    rcx,QWORD PTR [rsp+0x90]
  62f0ee:	mov    rax,QWORD PTR [rcx]
  62f0f1:	mov    rcx,QWORD PTR [rcx+0x8]
  62f0f5:	vmovq  xmm0,rax
  62f0fa:	vpclmullqlqdq xmm3,xmm1,xmm0
  62f100:	vmovq  xmm2,r11
  62f105:	vmovq  xmm1,rcx
  62f10a:	vpclmullqlqdq xmm4,xmm2,xmm1
  62f110:	xor    r10,r11
  62f113:	xor    rcx,rax
  62f116:	vmovq  xmm5,r10
  62f11b:	vmovq  xmm2,rcx
  62f120:	vpclmullqlqdq xmm5,xmm5,xmm2
  62f126:	vpternlogq xmm5,xmm3,xmm4,0x96
  62f12d:	vpshufd xmm6,xmm4,0xee
  62f132:	vpclmullqlqdq xmm6,xmm6,xmm10
  62f138:	vpshufd xmm7,xmm5,0xee
  62f13d:	vpxor  xmm4,xmm7,xmm4
  62f141:	vpextrq rax,xmm6,0x1
  62f147:	vpclmullqlqdq xmm4,xmm4,xmm10
  62f14d:	lea    rcx,[rax+rax*1]
  62f151:	shl    rax,0x2
  62f155:	xor    rax,rcx
  62f158:	vpxor  xmm7,xmm3,xmm4
  62f15c:	vmovq  rcx,xmm7
  62f161:	xor    rcx,rax
  62f164:	vpshufd xmm3,xmm3,0xee
  62f169:	vpsllq xmm7,xmm6,0x7
  62f16e:	vpxor  xmm3,xmm6,xmm3
  62f172:	vpalignr xmm3,xmm3,xmm7,0x8
  62f178:	vpbroadcastq xmm5,xmm5
  62f17d:	vpinsrq xmm5,xmm5,rcx,0x0
  62f183:	vpxor  xmm3,xmm3,xmm5
  62f187:	vmovdqa xmm5,XMMWORD PTR [rsp+rdx*1+0x1180]
  62f190:	vpblendd xmm4,xmm5,xmm4,0xc
  62f196:	vpunpckhqdq xmm5,xmm6,xmm5
  62f19a:	vpternlogq xmm5,xmm3,xmm4,0x96
  62f1a1:	vmovdqa XMMWORD PTR [rsp+rdx*1+0x1180],xmm5
  62f1aa:	vmovq  xmm3,r9
  62f1af:	vpclmullqlqdq xmm3,xmm3,xmm0
  62f1b5:	vmovq  xmm4,r8
  62f1ba:	vpclmullqlqdq xmm4,xmm4,xmm1
  62f1c0:	xor    r8,r9
  62f1c3:	vmovq  xmm5,r8
  62f1c8:	vpclmullqlqdq xmm5,xmm5,xmm2
  62f1ce:	vpternlogq xmm5,xmm3,xmm4,0x96
  62f1d5:	vpshufd xmm6,xmm4,0xee
  62f1da:	vpclmullqlqdq xmm6,xmm6,xmm10
  62f1e0:	vpshufd xmm7,xmm5,0xee
  62f1e5:	vpxor  xmm4,xmm7,xmm4
  62f1e9:	vpextrq rax,xmm6,0x1
  62f1ef:	vpclmullqlqdq xmm4,xmm4,xmm10
  62f1f5:	lea    rcx,[rax+rax*1]
  62f1f9:	shl    rax,0x2
  62f1fd:	xor    rax,rcx
  62f200:	vpxor  xmm7,xmm3,xmm4
  62f204:	vmovq  rcx,xmm7
  62f209:	xor    rcx,rax
  62f20c:	vpshufd xmm3,xmm3,0xee
  62f211:	vpsllq xmm7,xmm6,0x7
  62f216:	vpxor  xmm3,xmm6,xmm3
  62f21a:	vpalignr xmm3,xmm3,xmm7,0x8
  62f220:	vpbroadcastq xmm5,xmm5
  62f225:	vpinsrq xmm5,xmm5,rcx,0x0
  62f22b:	vpxor  xmm3,xmm3,xmm5
  62f22f:	lea    rax,[rsp+0x1580]
  62f237:	vmovdqa xmm5,XMMWORD PTR [rax+rdx*1+0x400]
  62f240:	vpblendd xmm4,xmm5,xmm4,0xc
  62f246:	vpunpckhqdq xmm5,xmm6,xmm5
  62f24a:	vpternlogq xmm5,xmm3,xmm4,0x96
  62f251:	vmovdqa XMMWORD PTR [rax+rdx*1+0x400],xmm5
  62f25a:	mov    rcx,QWORD PTR [rsp]
  62f25e:	inc    rcx
  62f261:	mov    rdx,QWORD PTR [rsp+0x8]
  62f266:	mov    rax,rdx
  62f269:	cmp    rdx,0x40
  62f26d:	jne    62f010 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x6b0>
  62f273:	lea    rcx,[rsp+0x2dc0]
  62f27b:	xor    eax,eax
  62f27d:	nop    DWORD PTR [rax]
  62f280:	mov    QWORD PTR [rsp+0x10],rax
  62f285:	inc    rax
  62f288:	mov    QWORD PTR [rsp+0x8],rax
  62f28d:	mov    QWORD PTR [rsp],rcx
  62f291:	xor    esi,esi
  62f293:	xor    r8d,r8d
  62f296:	xor    r10d,r10d
  62f299:	xor    r11d,r11d
  62f29c:	xor    r9d,r9d
  62f29f:	nop
  62f2a0:	movzx  edi,BYTE PTR [rcx-0x840]
  62f2a7:	add    rdi,rsi
  62f2aa:	cmp    rdi,rbp
  62f2ad:	jae    62fcf7 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x1397>
  62f2b3:	movzx  eax,BYTE PTR [rcx-0x40]
  62f2b7:	add    rax,rsi
  62f2ba:	cmp    rax,rbp
  62f2bd:	jae    62fd17 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x13b7>
  62f2c3:	movzx  ebx,BYTE PTR [rcx-0x800]
  62f2ca:	lea    rdx,[rsi+rbx*1+0x100]
  62f2d2:	cmp    rdx,rbp
  62f2d5:	jae    62fcef <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x138f>
  62f2db:	mov    r14,rbp
  62f2de:	movzx  ebp,BYTE PTR [rcx]
  62f2e1:	lea    rbx,[rsi+rbp*1+0x100]
  62f2e9:	cmp    rbx,r14
  62f2ec:	jae    62fd0a <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x13aa>
  62f2f2:	shl    rdi,0x4
  62f2f6:	mov    r14,QWORD PTR [rsp+0x48]
  62f2fb:	xor    r10,QWORD PTR [r14+rdi*1+0x8]
  62f300:	xor    r11,QWORD PTR [r14+rdi*1]
  62f304:	shl    rax,0x4
  62f308:	xor    r9,QWORD PTR [r14+rax*1]
  62f30c:	xor    r8,QWORD PTR [r14+rax*1+0x8]
  62f311:	shl    rdx,0x4
  62f315:	xor    r10,QWORD PTR [r14+rdx*1+0x8]
  62f31a:	xor    r11,QWORD PTR [r14+rdx*1]
  62f31e:	shl    rbx,0x4
  62f322:	xor    r9,QWORD PTR [r14+rbx*1]
  62f326:	xor    r8,QWORD PTR [r14+rbx*1+0x8]
  62f32b:	add    rsi,0x200
  62f332:	sub    rcx,0xffffffffffffff80
  62f336:	cmp    rsi,0x1000
  62f33d:	mov    rbp,QWORD PTR [rsp+0x18]
  62f342:	lea    r14,[rip+0xffffffffff9e0247]        # f590 <_RNvNvNtCstrZ1g5xv8S_15crossbeam_epoch5guard11unprotected11UNPROTECTED+0x550>
  62f349:	jne    62f2a0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x940>
  62f34f:	mov    rsi,QWORD PTR [rsp+0x10]
  62f354:	shl    rsi,0x4
  62f358:	vmovq  xmm3,r11
  62f35d:	vpclmullqlqdq xmm3,xmm3,xmm0
  62f363:	vmovq  xmm4,r10
  62f368:	vpclmullqlqdq xmm4,xmm4,xmm1
  62f36e:	xor    r11,r10
  62f371:	vmovq  xmm5,r11
  62f376:	vpclmullqlqdq xmm5,xmm5,xmm2
  62f37c:	vpternlogq xmm5,xmm3,xmm4,0x96
  62f383:	vpshufd xmm6,xmm4,0xee
  62f388:	vpclmullqlqdq xmm6,xmm6,xmm10
  62f38e:	vpshufd xmm7,xmm5,0xee
  62f393:	vpxor  xmm4,xmm7,xmm4
  62f397:	vpextrq rax,xmm6,0x1
  62f39d:	vpclmullqlqdq xmm4,xmm4,xmm10
  62f3a3:	lea    rcx,[rax+rax*1]
  62f3a7:	shl    rax,0x2
  62f3ab:	xor    rax,rcx
  62f3ae:	vpxor  xmm7,xmm3,xmm4
  62f3b2:	vmovq  rcx,xmm7
  62f3b7:	xor    rcx,rax
  62f3ba:	vpshufd xmm3,xmm3,0xee
  62f3bf:	vpsllq xmm7,xmm6,0x7
  62f3c4:	vpxor  xmm3,xmm6,xmm3
  62f3c8:	vpalignr xmm3,xmm3,xmm7,0x8
  62f3ce:	vpbroadcastq xmm5,xmm5
  62f3d3:	vpinsrq xmm5,xmm5,rcx,0x0
  62f3d9:	vpxor  xmm3,xmm3,xmm5
  62f3dd:	lea    rdx,[rsp+0x1580]
  62f3e5:	vmovdqa xmm5,XMMWORD PTR [rdx+rsi*1]
  62f3ea:	vpblendd xmm4,xmm5,xmm4,0xc
  62f3f0:	vpunpckhqdq xmm5,xmm6,xmm5
  62f3f4:	vpternlogq xmm5,xmm3,xmm4,0x96
  62f3fb:	vmovdqa XMMWORD PTR [rdx+rsi*1],xmm5
  62f400:	vmovq  xmm3,r9
  62f405:	vpclmullqlqdq xmm3,xmm3,xmm0
  62f40b:	vmovq  xmm4,r8
  62f410:	vpclmullqlqdq xmm4,xmm4,xmm1
  62f416:	xor    r8,r9
  62f419:	vmovq  xmm5,r8
  62f41e:	vpclmullqlqdq xmm5,xmm5,xmm2
  62f424:	vpternlogq xmm5,xmm3,xmm4,0x96
  62f42b:	vpshufd xmm6,xmm4,0xee
  62f430:	vpclmullqlqdq xmm6,xmm6,xmm10
  62f436:	vpshufd xmm7,xmm5,0xee
  62f43b:	vpxor  xmm4,xmm7,xmm4
  62f43f:	vpextrq rax,xmm6,0x1
  62f445:	vpclmullqlqdq xmm4,xmm4,xmm10
  62f44b:	lea    rcx,[rax+rax*1]
  62f44f:	shl    rax,0x2
  62f453:	xor    rax,rcx
  62f456:	vpxor  xmm7,xmm3,xmm4
  62f45a:	vmovq  rcx,xmm7
  62f45f:	xor    rcx,rax
  62f462:	vpshufd xmm3,xmm3,0xee
  62f467:	vpsllq xmm7,xmm6,0x7
  62f46c:	vpxor  xmm3,xmm6,xmm3
  62f470:	vpalignr xmm3,xmm3,xmm7,0x8
  62f476:	vpbroadcastq xmm5,xmm5
  62f47b:	vpinsrq xmm5,xmm5,rcx,0x0
  62f481:	vpxor  xmm3,xmm3,xmm5
  62f485:	vmovdqa xmm5,XMMWORD PTR [rdx+rsi*1+0x800]
  62f48e:	vpblendd xmm4,xmm5,xmm4,0xc
  62f494:	vpunpckhqdq xmm5,xmm6,xmm5
  62f498:	vpternlogq xmm5,xmm3,xmm4,0x96
  62f49f:	vmovdqa XMMWORD PTR [rdx+rsi*1+0x800],xmm5
  62f4a8:	mov    rcx,QWORD PTR [rsp]
  62f4ac:	inc    rcx
  62f4af:	mov    rdx,QWORD PTR [rsp+0x8]
  62f4b4:	mov    rax,rdx
  62f4b7:	cmp    rdx,0x40
  62f4bb:	jne    62f280 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x920>
  62f4c1:	mov    rax,QWORD PTR [rsp+0x78]
  62f4c6:	mov    rcx,rax
  62f4c9:	cmp    rax,QWORD PTR [rsp+0x68]
  62f4ce:	mov    rax,QWORD PTR [rsp+0x80]
  62f4d6:	jne    62eb40 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x1e0>
  62f4dc:	mov    rax,QWORD PTR [rsp+0x20]
  62f4e1:	mov    rsi,QWORD PTR [rax+0x28]
  62f4e5:	mov    rdi,QWORD PTR [rsp+0x28]
  62f4ea:	cmp    rdi,rsi
  62f4ed:	jae    62fd82 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x1422>
  62f4f3:	lea    rcx,[rdi+0x1]
  62f4f7:	mov    QWORD PTR [rsp],rcx
  62f4fb:	mov    rax,QWORD PTR [rax+0x20]
  62f4ff:	shl    rdi,0x4
  62f503:	mov    rcx,QWORD PTR [rax+rdi*1]
  62f507:	mov    rax,QWORD PTR [rax+rdi*1+0x8]
  62f50c:	vmovq  xmm0,rcx
  62f511:	vmovq  xmm1,rax
  62f516:	xor    rax,rcx
  62f519:	vmovq  xmm2,rax
  62f51e:	mov    eax,0x2800
  62f523:	lea    rbx,[rsp+0x1180]
  62f52b:	nop    DWORD PTR [rax+rax*1+0x0]
  62f530:	mov    rcx,QWORD PTR [rsp+rax*1-0x1680]
  62f538:	mov    rdx,QWORD PTR [rsp+rax*1-0x1678]
  62f540:	vmovq  xmm3,rcx
  62f545:	vpclmullqlqdq xmm3,xmm3,xmm0
  62f54b:	vmovq  xmm4,rdx
  62f550:	vpclmullqlqdq xmm4,xmm4,xmm1
  62f556:	xor    rdx,rcx
  62f559:	vmovq  xmm5,rdx
  62f55e:	vpclmullqlqdq xmm5,xmm5,xmm2
  62f564:	vpternlogq xmm5,xmm3,xmm4,0x96
  62f56b:	vpshufd xmm6,xmm4,0xee
  62f570:	vpclmullqlqdq xmm6,xmm6,xmm10
  62f576:	vpshufd xmm7,xmm5,0xee
  62f57b:	vpxor  xmm4,xmm7,xmm4
  62f57f:	vpclmullqlqdq xmm4,xmm4,xmm10
  62f585:	vpextrq rcx,xmm6,0x1
  62f58b:	lea    rdx,[rcx+rcx*1]
  62f58f:	shl    rcx,0x2
  62f593:	xor    rcx,rdx
  62f596:	vpxor  xmm7,xmm3,xmm4
  62f59a:	vmovq  rdx,xmm7
  62f59f:	xor    rdx,rcx
  62f5a2:	vpshufd xmm3,xmm3,0xee
  62f5a7:	vpsllq xmm7,xmm6,0x7
  62f5ac:	vpxor  xmm3,xmm6,xmm3
  62f5b0:	vpalignr xmm3,xmm3,xmm7,0x8
  62f5b6:	vpbroadcastq xmm5,xmm5
  62f5bb:	vpinsrq xmm5,xmm5,rdx,0x0
  62f5c1:	vpxor  xmm3,xmm3,xmm5
  62f5c5:	vmovdqa xmm5,XMMWORD PTR [rsp+rax*1+0x980]
  62f5ce:	vpblendd xmm4,xmm5,xmm4,0xc
  62f5d4:	vpunpckhqdq xmm5,xmm6,xmm5
  62f5d8:	vpternlogq xmm5,xmm3,xmm4,0x96
  62f5df:	vmovdqa XMMWORD PTR [rsp+rax*1+0x980],xmm5
  62f5e8:	mov    rcx,QWORD PTR [rsp+rax*1-0xe80]
  62f5f0:	vmovq  xmm3,rcx
  62f5f5:	vpclmullqlqdq xmm3,xmm3,xmm0
  62f5fb:	mov    rdx,QWORD PTR [rsp+rax*1-0xe78]
  62f603:	vmovq  xmm4,rdx
  62f608:	vpclmullqlqdq xmm4,xmm4,xmm1
  62f60e:	xor    rdx,rcx
  62f611:	vmovq  xmm5,rdx
  62f616:	vpclmullqlqdq xmm5,xmm5,xmm2
  62f61c:	vpternlogq xmm5,xmm3,xmm4,0x96
  62f623:	vpshufd xmm6,xmm4,0xee
  62f628:	vpclmullqlqdq xmm6,xmm6,xmm10
  62f62e:	vpshufd xmm7,xmm5,0xee
  62f633:	vpxor  xmm4,xmm7,xmm4
  62f637:	vpextrq rcx,xmm6,0x1
  62f63d:	vpclmullqlqdq xmm4,xmm4,xmm10
  62f643:	lea    rdx,[rcx+rcx*1]
  62f647:	shl    rcx,0x2
  62f64b:	xor    rcx,rdx
  62f64e:	vpxor  xmm7,xmm3,xmm4
  62f652:	vmovq  rdx,xmm7
  62f657:	xor    rdx,rcx
  62f65a:	vpshufd xmm3,xmm3,0xee
  62f65f:	vpsllq xmm7,xmm6,0x7
  62f664:	vpxor  xmm3,xmm6,xmm3
  62f668:	vpalignr xmm3,xmm3,xmm7,0x8
  62f66e:	vpbroadcastq xmm5,xmm5
  62f673:	vpinsrq xmm5,xmm5,rdx,0x0
  62f679:	vpxor  xmm3,xmm3,xmm5
  62f67d:	vmovdqa xmm5,XMMWORD PTR [rsp+rax*1+0x1180]
  62f686:	vpblendd xmm4,xmm5,xmm4,0xc
  62f68c:	vpunpckhqdq xmm5,xmm6,xmm5
  62f690:	vpternlogq xmm5,xmm3,xmm4,0x96
  62f697:	vmovdqa XMMWORD PTR [rsp+rax*1+0x1180],xmm5
  62f6a0:	add    rax,0x10
  62f6a4:	cmp    rax,0x2c00
  62f6aa:	jne    62f530 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0xbd0>
  62f6b0:	mov    eax,0x2c00
  62f6b5:	lea    rdi,[rsp+0x7980]
  62f6bd:	nop    DWORD PTR [rax]
  62f6c0:	mov    rcx,QWORD PTR [rsp+rax*1-0x1680]
  62f6c8:	mov    rdx,QWORD PTR [rsp+rax*1-0x1678]
  62f6d0:	vmovq  xmm3,rcx
  62f6d5:	vpclmullqlqdq xmm3,xmm3,xmm0
  62f6db:	vmovq  xmm4,rdx
  62f6e0:	vpclmullqlqdq xmm4,xmm4,xmm1
  62f6e6:	xor    rdx,rcx
  62f6e9:	vmovq  xmm5,rdx
  62f6ee:	vpclmullqlqdq xmm5,xmm5,xmm2
  62f6f4:	vpternlogq xmm5,xmm3,xmm4,0x96
  62f6fb:	vpshufd xmm6,xmm4,0xee
  62f700:	vpclmullqlqdq xmm6,xmm6,xmm10
  62f706:	vpshufd xmm7,xmm5,0xee
  62f70b:	vpxor  xmm4,xmm7,xmm4
  62f70f:	vpclmullqlqdq xmm4,xmm4,xmm10
  62f715:	vpextrq rcx,xmm6,0x1
  62f71b:	lea    rdx,[rcx+rcx*1]
  62f71f:	shl    rcx,0x2
  62f723:	xor    rcx,rdx
  62f726:	vpxor  xmm7,xmm3,xmm4
  62f72a:	vmovq  rdx,xmm7
  62f72f:	xor    rdx,rcx
  62f732:	vpshufd xmm3,xmm3,0xee
  62f737:	vpsllq xmm7,xmm6,0x7
  62f73c:	vpxor  xmm3,xmm6,xmm3
  62f740:	vpalignr xmm3,xmm3,xmm7,0x8
  62f746:	vpbroadcastq xmm5,xmm5
  62f74b:	vpinsrq xmm5,xmm5,rdx,0x0
  62f751:	vpxor  xmm3,xmm3,xmm5
  62f755:	vmovdqa xmm5,XMMWORD PTR [rsp+rax*1+0x980]
  62f75e:	vpblendd xmm4,xmm5,xmm4,0xc
  62f764:	vpunpckhqdq xmm5,xmm6,xmm5
  62f768:	vpternlogq xmm5,xmm3,xmm4,0x96
  62f76f:	vmovdqa XMMWORD PTR [rsp+rax*1+0x980],xmm5
  62f778:	mov    rcx,QWORD PTR [rsp+rax*1-0xe80]
  62f780:	vmovq  xmm3,rcx
  62f785:	vpclmullqlqdq xmm3,xmm3,xmm0
  62f78b:	mov    rdx,QWORD PTR [rsp+rax*1-0xe78]
  62f793:	vmovq  xmm4,rdx
  62f798:	vpclmullqlqdq xmm4,xmm4,xmm1
  62f79e:	xor    rdx,rcx
  62f7a1:	vmovq  xmm5,rdx
  62f7a6:	vpclmullqlqdq xmm5,xmm5,xmm2
  62f7ac:	vpternlogq xmm5,xmm3,xmm4,0x96
  62f7b3:	vpshufd xmm6,xmm4,0xee
  62f7b8:	vpclmullqlqdq xmm6,xmm6,xmm10
  62f7be:	vpshufd xmm7,xmm5,0xee
  62f7c3:	vpxor  xmm4,xmm7,xmm4
  62f7c7:	vpextrq rcx,xmm6,0x1
  62f7cd:	vpclmullqlqdq xmm4,xmm4,xmm10
  62f7d3:	lea    rdx,[rcx+rcx*1]
  62f7d7:	shl    rcx,0x2
  62f7db:	xor    rcx,rdx
  62f7de:	vpxor  xmm7,xmm3,xmm4
  62f7e2:	vmovq  rdx,xmm7
  62f7e7:	xor    rdx,rcx
  62f7ea:	vpshufd xmm3,xmm3,0xee
  62f7ef:	vpsllq xmm7,xmm6,0x7
  62f7f4:	vpxor  xmm3,xmm6,xmm3
  62f7f8:	vpalignr xmm3,xmm3,xmm7,0x8
  62f7fe:	vpbroadcastq xmm5,xmm5
  62f803:	vpinsrq xmm5,xmm5,rdx,0x0
  62f809:	vpxor  xmm3,xmm3,xmm5
  62f80d:	vmovdqa xmm5,XMMWORD PTR [rsp+rax*1+0x1180]
  62f816:	vpblendd xmm4,xmm5,xmm4,0xc
  62f81c:	vpunpckhqdq xmm5,xmm6,xmm5
  62f820:	vpternlogq xmm5,xmm3,xmm4,0x96
  62f827:	vmovdqa XMMWORD PTR [rsp+rax*1+0x1180],xmm5
  62f830:	add    rax,0x10
  62f834:	cmp    rax,0x3000
  62f83a:	jne    62f6c0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0xd60>
  62f840:	mov    edx,0x3000
  62f845:	mov    rsi,rbx
  62f848:	mov    r14,QWORD PTR [rip+0xb7999]        # 6e71e8 <memcpy@GLIBC_2.14>
  62f84f:	vzeroupper 
  62f852:	call   r14
  62f855:	mov    rax,r14
  62f858:	lea    rsi,[rsp+0x7980]
  62f860:	mov    rcx,QWORD PTR [rsp]
  62f864:	mov    rdi,rcx
  62f867:	cmp    rcx,QWORD PTR [rsp+0xc9c0]
  62f86f:	mov    r14,rbx
  62f872:	jne    62eaa0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x140>
  62f878:	lea    rdi,[rsp+0x1180]
  62f880:	lea    rsi,[rsp+0x7980]
  62f888:	mov    rbx,QWORD PTR [rip+0xb7959]        # 6e71e8 <memcpy@GLIBC_2.14>
  62f88f:	mov    edx,0x3000
  62f894:	call   rbx
  62f896:	lea    rsi,[rsp+0x3180]
  62f89e:	lea    r15,[rsp+0x5980]
  62f8a6:	mov    edx,0x800
  62f8ab:	mov    rdi,r15
  62f8ae:	call   rbx
  62f8b0:	lea    rdi,[rsp+0x6180]
  62f8b8:	lea    r14,[rsp+0x3980]
  62f8c0:	mov    edx,0x800
  62f8c5:	mov    rsi,r14
  62f8c8:	call   rbx
  62f8ca:	lea    rdi,[rsp+0x8980]
  62f8d2:	mov    r12d,0x1000
  62f8d8:	mov    edx,0x1000
  62f8dd:	mov    rsi,r15
  62f8e0:	call   rbx
  62f8e2:	lea    rdi,[rsp+0x180]
  62f8ea:	mov    r15,QWORD PTR [rip+0xb7907]        # 6e71f8 <memset@GLIBC_2.2.5>
  62f8f1:	mov    edx,0x800
  62f8f6:	xor    esi,esi
  62f8f8:	call   r15
  62f8fb:	lea    rdi,[rsp+0x980]
  62f903:	mov    edx,0x800
  62f908:	xor    esi,esi
  62f90a:	call   r15
  62f90d:	lea    rdi,[rsp+0x5180]
  62f915:	mov    edx,0x800
  62f91a:	mov    rsi,r14
  62f91d:	call   rbx
  62f91f:	nop
  62f920:	vmovups zmm0,ZMMWORD PTR [rsp+r12*1-0xe80]
  62f928:	vmovdqu64 zmm1,ZMMWORD PTR [rsp+r12*1-0xe40]
  62f930:	vxorps zmm0,zmm0,ZMMWORD PTR [rsp+r12*1+0x7980]
  62f93b:	vmovups ZMMWORD PTR [rsp+r12*1-0xe80],zmm0
  62f943:	vmovups zmm0,ZMMWORD PTR [rsp+r12*1-0x680]
  62f94b:	vmovdqu64 zmm2,ZMMWORD PTR [rsp+r12*1-0x640]
  62f953:	vxorps zmm0,zmm0,ZMMWORD PTR [rsp+r12*1+0x4180]
  62f95e:	vmovups ZMMWORD PTR [rsp+r12*1-0x680],zmm0
  62f966:	vpxorq zmm0,zmm1,ZMMWORD PTR [rsp+r12*1+0x79c0]
  62f971:	vmovdqu64 ZMMWORD PTR [rsp+r12*1-0xe40],zmm0
  62f979:	vpxorq zmm0,zmm2,ZMMWORD PTR [rsp+r12*1+0x41c0]
  62f984:	vmovdqu64 ZMMWORD PTR [rsp+r12*1-0x640],zmm0
  62f98c:	sub    r12,0xffffffffffffff80
  62f990:	cmp    r12,0x1800
  62f997:	jne    62f920 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0xfc0>
  62f999:	lea    r14,[rsp+0x6980]
  62f9a1:	lea    rsi,[rsp+0x180]
  62f9a9:	mov    edx,0x800
  62f9ae:	mov    rdi,r14
  62f9b1:	vzeroupper 
  62f9b4:	call   rbx
  62f9b6:	lea    rdi,[rsp+0x7180]
  62f9be:	lea    rsi,[rsp+0x980]
  62f9c6:	mov    edx,0x800
  62f9cb:	call   rbx
  62f9cd:	mov    edx,0x1000
  62f9d2:	mov    rdi,QWORD PTR [rsp+0x58]
  62f9d7:	mov    rsi,r14
  62f9da:	jmp    62fcdb <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x137b>
  62f9df:	test   dl,dl
  62f9e1:	je     62f9fb <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x109b>
  62f9e3:	mov    rax,0xffffffffffffffa8
  62f9ea:	mov    rax,QWORD PTR fs:[rax]
  62f9ee:	test   rax,rax
  62f9f1:	je     62fa28 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x10c8>
  62f9f3:	add    rax,0x110
  62f9f9:	jmp    62fa40 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x10e0>
  62f9fb:	test   rcx,rcx
  62f9fe:	je     62e9c9 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x69>
  62fa04:	shr    rcx,1
  62fa07:	jmp    62fa54 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x10f4>
  62fa09:	mov    rdi,rdx
  62fa0c:	lea    rsi,[rdx+0x80]
  62fa13:	lea    rcx,[rip+0xb35ae]        # 6e2fc8 <anon.01aa214378fce5a476fd64886db4a5c0.1334.llvm.3043956165524440405+0x210>
  62fa1a:	mov    rdx,QWORD PTR [rsp+0x30]
  62fa1f:	vzeroupper 
  62fa22:	call   QWORD PTR [rip+0xb78e8]        # 6e7310 <_DYNAMIC+0x340>
  62fa28:	mov    rbp,r14
  62fa2b:	mov    r14,rsi
  62fa2e:	mov    r15,rcx
  62fa31:	call   QWORD PTR [rip+0xb7b61]        # 6e7598 <_DYNAMIC+0x5c8>
  62fa37:	mov    rcx,r15
  62fa3a:	mov    rsi,r14
  62fa3d:	mov    r14,rbp
  62fa40:	mov    rax,QWORD PTR [rax]
  62fa43:	mov    rax,QWORD PTR [rax+0x208]
  62fa4a:	shr    rcx,1
  62fa4d:	cmp    rcx,rax
  62fa50:	cmovbe rcx,rax
  62fa54:	mov    QWORD PTR [rsp+0xc8],rcx
  62fa5c:	mov    QWORD PTR [rsp+0x5180],rsi
  62fa64:	xor    eax,eax
  62fa66:	mov    rcx,r12
  62fa69:	sub    rcx,r14
  62fa6c:	cmovae rax,rcx
  62fa70:	cmp    rsi,rax
  62fa73:	ja     62fd2d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x13cd>
  62fa79:	add    rsi,r14
  62fa7c:	mov    rax,QWORD PTR [rbx]
  62fa7f:	mov    rcx,QWORD PTR [rbx+0x8]
  62fa83:	mov    rdx,QWORD PTR [rbx+0x10]
  62fa87:	mov    rdi,QWORD PTR [rbx+0x18]
  62fa8b:	mov    r8,QWORD PTR [rbx+0x20]
  62fa8f:	lea    r9,[rsp+0xc0]
  62fa97:	mov    QWORD PTR [rsp+0xd8],r9
  62fa9f:	lea    r9,[rsp+0x5180]
  62faa7:	mov    QWORD PTR [rsp+0xe0],r9
  62faaf:	lea    r10,[rsp+0xc8]
  62fab7:	mov    QWORD PTR [rsp+0xe8],r10
  62fabf:	mov    QWORD PTR [rsp+0xf0],rax
  62fac7:	mov    QWORD PTR [rsp+0xf8],rcx
  62facf:	mov    QWORD PTR [rsp+0x100],rdx
  62fad7:	mov    QWORD PTR [rsp+0x108],rdi
  62fadf:	mov    QWORD PTR [rsp+0x110],r8
  62fae7:	mov    QWORD PTR [rsp+0x118],rsi
  62faef:	mov    QWORD PTR [rsp+0x120],r12
  62faf7:	mov    QWORD PTR [rsp+0x128],r9
  62faff:	mov    QWORD PTR [rsp+0x130],r10
  62fb07:	mov    QWORD PTR [rsp+0x138],rax
  62fb0f:	mov    QWORD PTR [rsp+0x140],rcx
  62fb17:	mov    QWORD PTR [rsp+0x148],rdx
  62fb1f:	mov    QWORD PTR [rsp+0x150],rdi
  62fb27:	mov    QWORD PTR [rsp+0x158],r8
  62fb2f:	mov    QWORD PTR [rsp+0x160],r14
  62fb37:	mov    QWORD PTR [rsp+0x168],rsi
  62fb3f:	mov    rbx,0xffffffffffffffa8
  62fb46:	mov    rdx,QWORD PTR fs:[rbx]
  62fb4a:	test   rdx,rdx
  62fb4d:	jne    62fb72 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x1212>
  62fb4f:	call   QWORD PTR [rip+0xb7a43]        # 6e7598 <_DYNAMIC+0x5c8>
  62fb55:	mov    rsi,QWORD PTR [rax]
  62fb58:	mov    rdx,QWORD PTR fs:[rbx]
  62fb5c:	test   rdx,rdx
  62fb5f:	je     62fd46 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x13e6>
  62fb65:	cmp    QWORD PTR [rdx+0x110],rsi
  62fb6c:	jne    62fd64 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x1404>
  62fb72:	lea    rdi,[rsp+0x1180]
  62fb7a:	lea    rsi,[rsp+0xd8]
  62fb82:	xor    ecx,ecx
  62fb84:	call   630ea0 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_4fold12FoldConsumerINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB3X_s1_0ENCB3X_s0_0ENvMB3Z_NtB3Z_6Worker3newNCB3X_s_0EE0NCBR_s_0TANtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128j80_B75_EB74_E0B45_>
  62fb89:	lea    r14,[rsp+0xa980]
  62fb91:	lea    rsi,[rsp+0x1180]
  62fb99:	mov    r12d,0x1000
  62fb9f:	mov    rbx,QWORD PTR [rip+0xb7642]        # 6e71e8 <memcpy@GLIBC_2.14>
  62fba6:	mov    edx,0x1000
  62fbab:	mov    rdi,r14
  62fbae:	call   rbx
  62fbb0:	lea    rsi,[rsp+0x2180]
  62fbb8:	lea    r15,[rsp+0xb980]
  62fbc0:	mov    edx,0x1000
  62fbc5:	mov    rdi,r15
  62fbc8:	call   rbx
  62fbca:	lea    rdi,[rsp+0x2180]
  62fbd2:	mov    edx,0x1000
  62fbd7:	mov    rsi,r15
  62fbda:	call   rbx
  62fbdc:	lea    rdi,[rsp+0x5980]
  62fbe4:	mov    edx,0x800
  62fbe9:	mov    rsi,r14
  62fbec:	call   rbx
  62fbee:	lea    rsi,[rsp+0xb180]
  62fbf6:	lea    rdi,[rsp+0x6980]
  62fbfe:	mov    edx,0x800
  62fc03:	call   rbx
  62fc05:	lea    rsi,[rsp+0xc180]
  62fc0d:	lea    rdi,[rsp+0x7980]
  62fc15:	mov    edx,0x800
  62fc1a:	call   rbx
  62fc1c:	nop    DWORD PTR [rax+0x0]
  62fc20:	vmovups zmm0,ZMMWORD PTR [rsp+r12*1+0x4980]
  62fc2b:	vmovdqu64 zmm1,ZMMWORD PTR [rsp+r12*1+0x49c0]
  62fc36:	vxorps zmm0,zmm0,ZMMWORD PTR [rsp+r12*1+0x1180]
  62fc3e:	vmovups ZMMWORD PTR [rsp+r12*1+0x4980],zmm0
  62fc49:	vmovups zmm0,ZMMWORD PTR [rsp+r12*1+0x5980]
  62fc54:	vmovdqu64 zmm2,ZMMWORD PTR [rsp+r12*1+0x59c0]
  62fc5f:	vxorps zmm0,zmm0,ZMMWORD PTR [rsp+r12*1+0x6980]
  62fc6a:	vmovups ZMMWORD PTR [rsp+r12*1+0x5980],zmm0
  62fc75:	vpxorq zmm0,zmm1,ZMMWORD PTR [rsp+r12*1+0x11c0]
  62fc7d:	vmovdqu64 ZMMWORD PTR [rsp+r12*1+0x49c0],zmm0
  62fc88:	vpxorq zmm0,zmm2,ZMMWORD PTR [rsp+r12*1+0x69c0]
  62fc93:	vmovdqu64 ZMMWORD PTR [rsp+r12*1+0x59c0],zmm0
  62fc9e:	sub    r12,0xffffffffffffff80
  62fca2:	cmp    r12,0x1800
  62fca9:	jne    62fc20 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x12c0>
  62fcaf:	lea    rsi,[rsp+0x5980]
  62fcb7:	mov    edx,0x800
  62fcbc:	mov    rdi,r13
  62fcbf:	vzeroupper 
  62fcc2:	call   rbx
  62fcc4:	add    r13,0x800
  62fccb:	lea    rsi,[rsp+0x6980]
  62fcd3:	mov    edx,0x800
  62fcd8:	mov    rdi,r13
  62fcdb:	call   rbx
  62fcdd:	add    rsp,0xc988
  62fce4:	pop    rbx
  62fce5:	pop    r12
  62fce7:	pop    r13
  62fce9:	pop    r14
  62fceb:	pop    r15
  62fced:	pop    rbp
  62fcee:	ret    
  62fcef:	lea    rdi,[rbx+rsi*1+0x100]
  62fcf7:	lea    rdx,[rip+0xb3ca2]        # 6e39a0 <anon.01aa214378fce5a476fd64886db4a5c0.2593.llvm.3043956165524440405+0x5e8>
  62fcfe:	mov    rsi,rbp
  62fd01:	vzeroupper 
  62fd04:	call   QWORD PTR [rip+0xb7516]        # 6e7220 <_DYNAMIC+0x250>
  62fd0a:	lea    rax,[rbp+rsi*1+0x100]
  62fd12:	mov    rbp,QWORD PTR [rsp+0x18]
  62fd17:	lea    rdx,[rip+0xb3c9a]        # 6e39b8 <anon.01aa214378fce5a476fd64886db4a5c0.2593.llvm.3043956165524440405+0x600>
  62fd1e:	mov    rdi,rax
  62fd21:	mov    rsi,rbp
  62fd24:	vzeroupper 
  62fd27:	call   QWORD PTR [rip+0xb74f3]        # 6e7220 <_DYNAMIC+0x250>
  62fd2d:	lea    rdi,[rip+0xffffffffff9e2b40]        # 12874 <anon.b29a6a3dffb4aad81a99129e80bda2af.11.llvm.12663359458273545214>
  62fd34:	lea    rdx,[rip+0xb4d1d]        # 6e4a58 <anon.b29a6a3dffb4aad81a99129e80bda2af.16.llvm.12663359458273545214>
  62fd3b:	mov    esi,0x2b
  62fd40:	call   QWORD PTR [rip+0xb75d2]        # 6e7318 <_DYNAMIC+0x348>
  62fd46:	sub    rsi,0xffffffffffffff80
  62fd4a:	lea    rdi,[rsp+0x1180]
  62fd52:	lea    rdx,[rsp+0xd8]
  62fd5a:	call   622dd0 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1P_5range12IterProducerjEINtNtB1N_4fold12FoldConsumerINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB4P_s1_0ENCB4P_s0_0ENvMB4R_NtB4R_6Worker3newNCB4P_s_0EE0NCB1G_s_0TANtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128j80_B7Y_EB7X_E0TB7X_B7X_EEB4X_>
  62fd5f:	jmp    62fb89 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x1229>
  62fd64:	sub    rsi,0xffffffffffffff80
  62fd68:	lea    rdi,[rsp+0x1180]
  62fd70:	lea    rcx,[rsp+0xd8]
  62fd78:	call   623ca0 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1Q_5range12IterProducerjEINtNtB1O_4fold12FoldConsumerINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB4Q_s1_0ENCB4Q_s0_0ENvMB4S_NtB4S_6Worker3newNCB4Q_s_0EE0NCB1H_s_0TANtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128j80_B7Z_EB7Y_E0TB7Y_B7Y_EEB4Y_>
  62fd7d:	jmp    62fb89 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_4fold12FoldConsumerINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtNtNtCs3dS8gfMgR2U_10flock_core9zerocheck25univariate_skip_optimized7skip12842round1_shift_reduce_extract_c_packed_skip7s2_0NCB34_s1_0ENCB34_s0_0ENvMB36_NtB36_6Worker3newNCB34_s_0EEB3c_+0x1229>
  62fd82:	lea    rdx,[rip+0xb3227]        # 6e2fb0 <anon.01aa214378fce5a476fd64886db4a5c0.1334.llvm.3043956165524440405+0x1f8>
  62fd89:	vzeroupper 
  62fd8c:	call   QWORD PTR [rip+0xb748e]        # 6e7220 <_DYNAMIC+0x250>
  62fd92:	int3   
  62fd93:	int3   
  62fd94:	int3   
  62fd95:	int3   
  62fd96:	int3   
  62fd97:	int3   
  62fd98:	int3   
  62fd99:	int3   
  62fd9a:	int3   
  62fd9b:	int3   
  62fd9c:	int3   
  62fd9d:	int3   
  62fd9e:	int3   
  62fd9f:	int3   

