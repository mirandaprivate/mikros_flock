000000000065ca30 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_>:
  65ca30:	push   rbp
  65ca31:	push   r15
  65ca33:	push   r14
  65ca35:	push   r13
  65ca37:	push   r12
  65ca39:	push   rbx
  65ca3a:	sub    rsp,0x1e8
  65ca41:	mov    r12,rdi
  65ca44:	mov    r13,QWORD PTR [rsp+0x220]
  65ca4c:	mov    QWORD PTR [rsp+0x38],rsi
  65ca51:	mov    QWORD PTR [rsp+0x50],r8
  65ca56:	shr    rsi,1
  65ca59:	cmp    rsi,r8
  65ca5c:	jae    65cab2 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x82>
  65ca5e:	mov    r10,QWORD PTR [r9+0x10]
  65ca62:	test   r10,r10
  65ca65:	je     65ce3a <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x40a>
  65ca6b:	mov    r11,QWORD PTR [r9+0x28]
  65ca6f:	test   r11,r11
  65ca72:	je     65ce3a <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x40a>
  65ca78:	mov    QWORD PTR [rsp+0x8],r12
  65ca7d:	mov    r12,QWORD PTR [r9]
  65ca80:	mov    r8,QWORD PTR [r9+0x8]
  65ca84:	mov    rbx,QWORD PTR [r9+0x18]
  65ca88:	mov    r14,QWORD PTR [r9+0x20]
  65ca8c:	mov    r9,QWORD PTR [r9+0x30]
  65ca90:	mov    rdi,QWORD PTR [r13+0x10]
  65ca94:	test   r8,r8
  65ca97:	je     65cae2 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0xb2>
  65ca99:	mov    rax,r8
  65ca9c:	or     rax,r10
  65ca9f:	shr    rax,0x20
  65caa3:	je     65caed <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0xbd>
  65caa5:	mov    rax,r8
  65caa8:	xor    edx,edx
  65caaa:	div    r10
  65caad:	mov    rsi,rax
  65cab0:	jmp    65caf7 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0xc7>
  65cab2:	test   dl,dl
  65cab4:	je     65cad5 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0xa5>
  65cab6:	mov    rax,0xffffffffffffffa8
  65cabd:	mov    rax,QWORD PTR fs:[rax]
  65cac1:	test   rax,rax
  65cac4:	je     65cc4f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x21f>
  65caca:	add    rax,0x110
  65cad0:	jmp    65cc67 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x237>
  65cad5:	test   rcx,rcx
  65cad8:	je     65ca5e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x2e>
  65cada:	shr    rcx,1
  65cadd:	jmp    65cc7b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x24b>
  65cae2:	xor    esi,esi
  65cae4:	test   r14,r14
  65cae7:	jne    65cb04 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0xd4>
  65cae9:	xor    eax,eax
  65caeb:	jmp    65cb31 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x101>
  65caed:	mov    eax,r8d
  65caf0:	xor    edx,edx
  65caf2:	div    r10d
  65caf5:	mov    esi,eax
  65caf7:	cmp    rdx,0x1
  65cafb:	sbb    rsi,0xffffffffffffffff
  65caff:	test   r14,r14
  65cb02:	je     65cae9 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0xb9>
  65cb04:	mov    rax,r14
  65cb07:	or     rax,r11
  65cb0a:	shr    rax,0x20
  65cb0e:	je     65cb1a <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0xea>
  65cb10:	mov    rax,r14
  65cb13:	xor    edx,edx
  65cb15:	div    r11
  65cb18:	jmp    65cb22 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0xf2>
  65cb1a:	mov    eax,r14d
  65cb1d:	xor    edx,edx
  65cb1f:	div    r11d
  65cb22:	cmp    rdx,0x1
  65cb26:	sbb    rax,0xffffffffffffffff
  65cb2a:	cmp    rax,rsi
  65cb2d:	cmovae rax,rsi
  65cb31:	lea    rdx,[rax+r9*1]
  65cb35:	xor    esi,esi
  65cb37:	sub    rdx,r9
  65cb3a:	cmovae rsi,rdx
  65cb3e:	cmp    rax,rsi
  65cb41:	cmovb  rsi,rax
  65cb45:	test   rsi,rsi
  65cb48:	je     65cc34 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x204>
  65cb4e:	mov    rax,QWORD PTR [rdi]
  65cb51:	mov    QWORD PTR [rsp+0x20],rax
  65cb56:	mov    rax,r11
  65cb59:	shl    rax,0x4
  65cb5d:	mov    QWORD PTR [rsp+0x18],rax
  65cb62:	mov    rax,r10
  65cb65:	shl    rax,0x4
  65cb69:	mov    QWORD PTR [rsp+0x10],rax
  65cb6e:	shl    r9,0xa
  65cb72:	vxorps xmm0,xmm0,xmm0
  65cb76:	vxorps xmm1,xmm1,xmm1
  65cb7a:	nop    WORD PTR [rax+rax*1+0x0]
  65cb80:	vmovups YMMWORD PTR [rsp+0x1a0],ymm1
  65cb89:	vmovups YMMWORD PTR [rsp+0x1c0],ymm0
  65cb92:	mov    QWORD PTR [rsp+0x28],rsi
  65cb97:	mov    QWORD PTR [rsp+0x30],r9
  65cb9c:	mov    rbp,r10
  65cb9f:	sub    rbp,r8
  65cba2:	cmovb  r8,r10
  65cba6:	neg    rbp
  65cba9:	mov    r13,r11
  65cbac:	sub    r13,r14
  65cbaf:	cmovb  r14,r11
  65cbb3:	neg    r13
  65cbb6:	mov    QWORD PTR [rsp],r14
  65cbba:	lea    rdi,[rsp+0x120]
  65cbc2:	mov    rsi,QWORD PTR [rsp+0x20]
  65cbc7:	mov    rdx,r9
  65cbca:	mov    rcx,r12
  65cbcd:	mov    r9,rbx
  65cbd0:	mov    r14,r10
  65cbd3:	mov    r15,r11
  65cbd6:	vzeroupper 
  65cbd9:	call   6650d0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_into0B5_>
  65cbde:	vmovups ymm1,YMMWORD PTR [rsp+0x1a0]
  65cbe7:	vmovups ymm0,YMMWORD PTR [rsp+0x1c0]
  65cbf0:	mov    rsi,QWORD PTR [rsp+0x28]
  65cbf5:	mov    r9,QWORD PTR [rsp+0x30]
  65cbfa:	mov    r11,r15
  65cbfd:	mov    r10,r14
  65cc00:	vxorps ymm0,ymm0,YMMWORD PTR [rsp+0x120]
  65cc09:	vxorps ymm1,ymm1,YMMWORD PTR [rsp+0x140]
  65cc12:	add    rbx,QWORD PTR [rsp+0x18]
  65cc17:	add    r12,QWORD PTR [rsp+0x10]
  65cc1c:	add    r9,0x400
  65cc23:	mov    r8,rbp
  65cc26:	mov    r14,r13
  65cc29:	dec    rsi
  65cc2c:	jne    65cb80 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x150>
  65cc32:	jmp    65cc3c <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x20c>
  65cc34:	vxorps xmm0,xmm0,xmm0
  65cc38:	vxorps xmm1,xmm1,xmm1
  65cc3c:	mov    rax,QWORD PTR [rsp+0x8]
  65cc41:	vmovups YMMWORD PTR [rax],ymm0
  65cc45:	vmovups YMMWORD PTR [rax+0x20],ymm1
  65cc4a:	jmp    65ce25 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x3f5>
  65cc4f:	mov    rbx,r9
  65cc52:	mov    r15,rsi
  65cc55:	mov    r14,rcx
  65cc58:	call   QWORD PTR [rip+0x9b982]        # 6f85e0 <_DYNAMIC+0x5c8>
  65cc5e:	mov    rcx,r14
  65cc61:	mov    rsi,r15
  65cc64:	mov    r9,rbx
  65cc67:	mov    rax,QWORD PTR [rax]
  65cc6a:	mov    rax,QWORD PTR [rax+0x208]
  65cc71:	shr    rcx,1
  65cc74:	cmp    rcx,rax
  65cc77:	cmovbe rcx,rax
  65cc7b:	mov    QWORD PTR [rsp+0x48],rcx
  65cc80:	mov    rax,QWORD PTR [r9]
  65cc83:	mov    r11,QWORD PTR [r9+0x8]
  65cc87:	mov    rcx,QWORD PTR [r9+0x10]
  65cc8b:	mov    rdx,QWORD PTR [r9+0x18]
  65cc8f:	mov    r10,QWORD PTR [r9+0x20]
  65cc93:	mov    rdi,QWORD PTR [r9+0x28]
  65cc97:	mov    r8,rcx
  65cc9a:	imul   r8,rsi
  65cc9e:	cmp    r11,r8
  65cca1:	cmovb  r8,r11
  65cca5:	mov    rbx,r8
  65cca8:	shl    rbx,0x4
  65ccac:	add    rbx,rax
  65ccaf:	sub    r11,r8
  65ccb2:	lea    r14,[rsp+0x38]
  65ccb7:	mov    QWORD PTR [rsp+0x58],r14
  65ccbc:	lea    r14,[rsp+0x40]
  65ccc1:	mov    QWORD PTR [rsp+0x60],r14
  65ccc6:	lea    r15,[rsp+0x48]
  65cccb:	mov    QWORD PTR [rsp+0x68],r15
  65ccd0:	mov    QWORD PTR [rsp+0x70],rbx
  65ccd5:	mov    rbx,rdi
  65ccd8:	imul   rbx,rsi
  65ccdc:	cmp    r10,rbx
  65ccdf:	cmovb  rbx,r10
  65cce3:	mov    QWORD PTR [rsp+0x78],r11
  65cce8:	mov    r11,rbx
  65cceb:	shl    r11,0x4
  65ccef:	add    r11,rdx
  65ccf2:	mov    QWORD PTR [rsp+0x80],rcx
  65ccfa:	mov    QWORD PTR [rsp+0x88],r11
  65cd02:	mov    QWORD PTR [rsp+0x40],rsi
  65cd07:	sub    r10,rbx
  65cd0a:	mov    r9,QWORD PTR [r9+0x30]
  65cd0e:	add    rsi,r9
  65cd11:	mov    QWORD PTR [rsp+0x90],r10
  65cd19:	mov    r10,QWORD PTR [r13+0x0]
  65cd1d:	mov    QWORD PTR [rsp+0x98],rdi
  65cd25:	mov    QWORD PTR [rsp+0xa0],rsi
  65cd2d:	mov    rsi,QWORD PTR [r13+0x8]
  65cd31:	mov    r11,QWORD PTR [r13+0x10]
  65cd35:	mov    QWORD PTR [rsp+0xa8],r10
  65cd3d:	mov    QWORD PTR [rsp+0xb0],rsi
  65cd45:	mov    QWORD PTR [rsp+0xb8],r11
  65cd4d:	mov    QWORD PTR [rsp+0xc0],r14
  65cd55:	mov    QWORD PTR [rsp+0xc8],r15
  65cd5d:	mov    QWORD PTR [rsp+0xd0],rax
  65cd65:	mov    QWORD PTR [rsp+0xd8],r8
  65cd6d:	mov    QWORD PTR [rsp+0xe0],rcx
  65cd75:	mov    QWORD PTR [rsp+0xe8],rdx
  65cd7d:	mov    QWORD PTR [rsp+0xf0],rbx
  65cd85:	mov    QWORD PTR [rsp+0xf8],rdi
  65cd8d:	mov    QWORD PTR [rsp+0x100],r9
  65cd95:	mov    QWORD PTR [rsp+0x108],r10
  65cd9d:	mov    QWORD PTR [rsp+0x110],rsi
  65cda5:	mov    QWORD PTR [rsp+0x118],r11
  65cdad:	mov    rbx,0xffffffffffffffa8
  65cdb4:	mov    rdx,QWORD PTR fs:[rbx]
  65cdb8:	test   rdx,rdx
  65cdbb:	jne    65cde0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x3b0>
  65cdbd:	call   QWORD PTR [rip+0x9b81d]        # 6f85e0 <_DYNAMIC+0x5c8>
  65cdc3:	mov    rsi,QWORD PTR [rax]
  65cdc6:	mov    rdx,QWORD PTR fs:[rbx]
  65cdca:	test   rdx,rdx
  65cdcd:	je     65ce53 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x423>
  65cdd3:	cmp    QWORD PTR [rdx+0x110],rsi
  65cdda:	jne    65ce6b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x43b>
  65cde0:	lea    rdi,[rsp+0x120]
  65cde8:	lea    rsi,[rsp+0x58]
  65cded:	xor    ecx,ecx
  65cdef:	call   6638c0 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_>
  65cdf4:	vmovups ymm0,YMMWORD PTR [rsp+0x160]
  65cdfd:	vmovups ymm1,YMMWORD PTR [rsp+0x180]
  65ce06:	vxorps ymm0,ymm0,YMMWORD PTR [rsp+0x120]
  65ce0f:	vxorps ymm1,ymm1,YMMWORD PTR [rsp+0x140]
  65ce18:	vmovups YMMWORD PTR [r12],ymm0
  65ce1e:	vmovups YMMWORD PTR [r12+0x20],ymm1
  65ce25:	add    rsp,0x1e8
  65ce2c:	pop    rbx
  65ce2d:	pop    r12
  65ce2f:	pop    r13
  65ce31:	pop    r14
  65ce33:	pop    r15
  65ce35:	pop    rbp
  65ce36:	vzeroupper 
  65ce39:	ret    
  65ce3a:	lea    rdi,[rip+0xffffffffff9b58c3]        # 12704 <anon.0e5291c933a25680de2bf032d375ada2.76.llvm.16073103944965275176+0x1>
  65ce41:	lea    rdx,[rip+0x98838]        # 6f5680 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x900>
  65ce48:	mov    esi,0x37
  65ce4d:	call   QWORD PTR [rip+0x9b47d]        # 6f82d0 <_DYNAMIC+0x2b8>
  65ce53:	sub    rsi,0xffffffffffffff80
  65ce57:	lea    rdi,[rsp+0x120]
  65ce5f:	lea    rdx,[rsp+0x58]
  65ce64:	call   655320 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B99_E0TB99_B99_EEB7q_>
  65ce69:	jmp    65cdf4 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x3c4>
  65ce6b:	sub    rsi,0xffffffffffffff80
  65ce6f:	lea    rdi,[rsp+0x120]
  65ce77:	lea    rcx,[rsp+0x58]
  65ce7c:	call   6562e0 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B9a_E0TB9a_B9a_EEB7r_>
  65ce81:	jmp    65cdf4 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x3c4>
  65ce86:	int3   
  65ce87:	int3   
  65ce88:	int3   
  65ce89:	int3   
  65ce8a:	int3   
  65ce8b:	int3   
  65ce8c:	int3   
  65ce8d:	int3   
  65ce8e:	int3   
  65ce8f:	int3   

