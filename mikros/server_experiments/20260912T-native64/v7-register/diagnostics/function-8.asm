000000000065c5d0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_>:
  65c5d0:	push   rbp
  65c5d1:	push   r15
  65c5d3:	push   r14
  65c5d5:	push   r13
  65c5d7:	push   r12
  65c5d9:	push   rbx
  65c5da:	sub    rsp,0x1e8
  65c5e1:	mov    r12,rdi
  65c5e4:	mov    r13,QWORD PTR [rsp+0x220]
  65c5ec:	mov    QWORD PTR [rsp+0x38],rsi
  65c5f1:	mov    QWORD PTR [rsp+0x50],r8
  65c5f6:	shr    rsi,1
  65c5f9:	cmp    rsi,r8
  65c5fc:	jae    65c652 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x82>
  65c5fe:	mov    r10,QWORD PTR [r9+0x10]
  65c602:	test   r10,r10
  65c605:	je     65c9da <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x40a>
  65c60b:	mov    r11,QWORD PTR [r9+0x28]
  65c60f:	test   r11,r11
  65c612:	je     65c9da <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x40a>
  65c618:	mov    QWORD PTR [rsp+0x8],r12
  65c61d:	mov    r12,QWORD PTR [r9]
  65c620:	mov    r8,QWORD PTR [r9+0x8]
  65c624:	mov    rbx,QWORD PTR [r9+0x18]
  65c628:	mov    r14,QWORD PTR [r9+0x20]
  65c62c:	mov    r9,QWORD PTR [r9+0x30]
  65c630:	mov    rdi,QWORD PTR [r13+0x10]
  65c634:	test   r8,r8
  65c637:	je     65c682 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0xb2>
  65c639:	mov    rax,r8
  65c63c:	or     rax,r10
  65c63f:	shr    rax,0x20
  65c643:	je     65c68d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0xbd>
  65c645:	mov    rax,r8
  65c648:	xor    edx,edx
  65c64a:	div    r10
  65c64d:	mov    rsi,rax
  65c650:	jmp    65c697 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0xc7>
  65c652:	test   dl,dl
  65c654:	je     65c675 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0xa5>
  65c656:	mov    rax,0xffffffffffffffa8
  65c65d:	mov    rax,QWORD PTR fs:[rax]
  65c661:	test   rax,rax
  65c664:	je     65c7ef <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x21f>
  65c66a:	add    rax,0x110
  65c670:	jmp    65c807 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x237>
  65c675:	test   rcx,rcx
  65c678:	je     65c5fe <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x2e>
  65c67a:	shr    rcx,1
  65c67d:	jmp    65c81b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x24b>
  65c682:	xor    esi,esi
  65c684:	test   r14,r14
  65c687:	jne    65c6a4 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0xd4>
  65c689:	xor    eax,eax
  65c68b:	jmp    65c6d1 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x101>
  65c68d:	mov    eax,r8d
  65c690:	xor    edx,edx
  65c692:	div    r10d
  65c695:	mov    esi,eax
  65c697:	cmp    rdx,0x1
  65c69b:	sbb    rsi,0xffffffffffffffff
  65c69f:	test   r14,r14
  65c6a2:	je     65c689 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0xb9>
  65c6a4:	mov    rax,r14
  65c6a7:	or     rax,r11
  65c6aa:	shr    rax,0x20
  65c6ae:	je     65c6ba <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0xea>
  65c6b0:	mov    rax,r14
  65c6b3:	xor    edx,edx
  65c6b5:	div    r11
  65c6b8:	jmp    65c6c2 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0xf2>
  65c6ba:	mov    eax,r14d
  65c6bd:	xor    edx,edx
  65c6bf:	div    r11d
  65c6c2:	cmp    rdx,0x1
  65c6c6:	sbb    rax,0xffffffffffffffff
  65c6ca:	cmp    rax,rsi
  65c6cd:	cmovae rax,rsi
  65c6d1:	lea    rdx,[rax+r9*1]
  65c6d5:	xor    esi,esi
  65c6d7:	sub    rdx,r9
  65c6da:	cmovae rsi,rdx
  65c6de:	cmp    rax,rsi
  65c6e1:	cmovb  rsi,rax
  65c6e5:	test   rsi,rsi
  65c6e8:	je     65c7d4 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x204>
  65c6ee:	mov    rax,QWORD PTR [rdi]
  65c6f1:	mov    QWORD PTR [rsp+0x20],rax
  65c6f6:	mov    rax,r11
  65c6f9:	shl    rax,0x4
  65c6fd:	mov    QWORD PTR [rsp+0x18],rax
  65c702:	mov    rax,r10
  65c705:	shl    rax,0x4
  65c709:	mov    QWORD PTR [rsp+0x10],rax
  65c70e:	shl    r9,0xa
  65c712:	vxorps xmm0,xmm0,xmm0
  65c716:	vxorps xmm1,xmm1,xmm1
  65c71a:	nop    WORD PTR [rax+rax*1+0x0]
  65c720:	vmovups YMMWORD PTR [rsp+0x1a0],ymm1
  65c729:	vmovups YMMWORD PTR [rsp+0x1c0],ymm0
  65c732:	mov    QWORD PTR [rsp+0x28],rsi
  65c737:	mov    QWORD PTR [rsp+0x30],r9
  65c73c:	mov    rbp,r10
  65c73f:	sub    rbp,r8
  65c742:	cmovb  r8,r10
  65c746:	neg    rbp
  65c749:	mov    r13,r11
  65c74c:	sub    r13,r14
  65c74f:	cmovb  r14,r11
  65c753:	neg    r13
  65c756:	mov    QWORD PTR [rsp],r14
  65c75a:	lea    rdi,[rsp+0x120]
  65c762:	mov    rsi,QWORD PTR [rsp+0x20]
  65c767:	mov    rdx,r9
  65c76a:	mov    rcx,r12
  65c76d:	mov    r9,rbx
  65c770:	mov    r14,r10
  65c773:	mov    r15,r11
  65c776:	vzeroupper 
  65c779:	call   664440 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_>
  65c77e:	vmovups ymm1,YMMWORD PTR [rsp+0x1a0]
  65c787:	vmovups ymm0,YMMWORD PTR [rsp+0x1c0]
  65c790:	mov    rsi,QWORD PTR [rsp+0x28]
  65c795:	mov    r9,QWORD PTR [rsp+0x30]
  65c79a:	mov    r11,r15
  65c79d:	mov    r10,r14
  65c7a0:	vxorps ymm0,ymm0,YMMWORD PTR [rsp+0x120]
  65c7a9:	vxorps ymm1,ymm1,YMMWORD PTR [rsp+0x140]
  65c7b2:	add    rbx,QWORD PTR [rsp+0x18]
  65c7b7:	add    r12,QWORD PTR [rsp+0x10]
  65c7bc:	add    r9,0x400
  65c7c3:	mov    r8,rbp
  65c7c6:	mov    r14,r13
  65c7c9:	dec    rsi
  65c7cc:	jne    65c720 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x150>
  65c7d2:	jmp    65c7dc <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x20c>
  65c7d4:	vxorps xmm0,xmm0,xmm0
  65c7d8:	vxorps xmm1,xmm1,xmm1
  65c7dc:	mov    rax,QWORD PTR [rsp+0x8]
  65c7e1:	vmovups YMMWORD PTR [rax],ymm0
  65c7e5:	vmovups YMMWORD PTR [rax+0x20],ymm1
  65c7ea:	jmp    65c9c5 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x3f5>
  65c7ef:	mov    rbx,r9
  65c7f2:	mov    r15,rsi
  65c7f5:	mov    r14,rcx
  65c7f8:	call   QWORD PTR [rip+0x9bde2]        # 6f85e0 <_DYNAMIC+0x5c8>
  65c7fe:	mov    rcx,r14
  65c801:	mov    rsi,r15
  65c804:	mov    r9,rbx
  65c807:	mov    rax,QWORD PTR [rax]
  65c80a:	mov    rax,QWORD PTR [rax+0x208]
  65c811:	shr    rcx,1
  65c814:	cmp    rcx,rax
  65c817:	cmovbe rcx,rax
  65c81b:	mov    QWORD PTR [rsp+0x48],rcx
  65c820:	mov    rax,QWORD PTR [r9]
  65c823:	mov    r11,QWORD PTR [r9+0x8]
  65c827:	mov    rcx,QWORD PTR [r9+0x10]
  65c82b:	mov    rdx,QWORD PTR [r9+0x18]
  65c82f:	mov    r10,QWORD PTR [r9+0x20]
  65c833:	mov    rdi,QWORD PTR [r9+0x28]
  65c837:	mov    r8,rcx
  65c83a:	imul   r8,rsi
  65c83e:	cmp    r11,r8
  65c841:	cmovb  r8,r11
  65c845:	mov    rbx,r8
  65c848:	shl    rbx,0x4
  65c84c:	add    rbx,rax
  65c84f:	sub    r11,r8
  65c852:	lea    r14,[rsp+0x38]
  65c857:	mov    QWORD PTR [rsp+0x58],r14
  65c85c:	lea    r14,[rsp+0x40]
  65c861:	mov    QWORD PTR [rsp+0x60],r14
  65c866:	lea    r15,[rsp+0x48]
  65c86b:	mov    QWORD PTR [rsp+0x68],r15
  65c870:	mov    QWORD PTR [rsp+0x70],rbx
  65c875:	mov    rbx,rdi
  65c878:	imul   rbx,rsi
  65c87c:	cmp    r10,rbx
  65c87f:	cmovb  rbx,r10
  65c883:	mov    QWORD PTR [rsp+0x78],r11
  65c888:	mov    r11,rbx
  65c88b:	shl    r11,0x4
  65c88f:	add    r11,rdx
  65c892:	mov    QWORD PTR [rsp+0x80],rcx
  65c89a:	mov    QWORD PTR [rsp+0x88],r11
  65c8a2:	mov    QWORD PTR [rsp+0x40],rsi
  65c8a7:	sub    r10,rbx
  65c8aa:	mov    r9,QWORD PTR [r9+0x30]
  65c8ae:	add    rsi,r9
  65c8b1:	mov    QWORD PTR [rsp+0x90],r10
  65c8b9:	mov    r10,QWORD PTR [r13+0x0]
  65c8bd:	mov    QWORD PTR [rsp+0x98],rdi
  65c8c5:	mov    QWORD PTR [rsp+0xa0],rsi
  65c8cd:	mov    rsi,QWORD PTR [r13+0x8]
  65c8d1:	mov    r11,QWORD PTR [r13+0x10]
  65c8d5:	mov    QWORD PTR [rsp+0xa8],r10
  65c8dd:	mov    QWORD PTR [rsp+0xb0],rsi
  65c8e5:	mov    QWORD PTR [rsp+0xb8],r11
  65c8ed:	mov    QWORD PTR [rsp+0xc0],r14
  65c8f5:	mov    QWORD PTR [rsp+0xc8],r15
  65c8fd:	mov    QWORD PTR [rsp+0xd0],rax
  65c905:	mov    QWORD PTR [rsp+0xd8],r8
  65c90d:	mov    QWORD PTR [rsp+0xe0],rcx
  65c915:	mov    QWORD PTR [rsp+0xe8],rdx
  65c91d:	mov    QWORD PTR [rsp+0xf0],rbx
  65c925:	mov    QWORD PTR [rsp+0xf8],rdi
  65c92d:	mov    QWORD PTR [rsp+0x100],r9
  65c935:	mov    QWORD PTR [rsp+0x108],r10
  65c93d:	mov    QWORD PTR [rsp+0x110],rsi
  65c945:	mov    QWORD PTR [rsp+0x118],r11
  65c94d:	mov    rbx,0xffffffffffffffa8
  65c954:	mov    rdx,QWORD PTR fs:[rbx]
  65c958:	test   rdx,rdx
  65c95b:	jne    65c980 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x3b0>
  65c95d:	call   QWORD PTR [rip+0x9bc7d]        # 6f85e0 <_DYNAMIC+0x5c8>
  65c963:	mov    rsi,QWORD PTR [rax]
  65c966:	mov    rdx,QWORD PTR fs:[rbx]
  65c96a:	test   rdx,rdx
  65c96d:	je     65c9f3 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x423>
  65c973:	cmp    QWORD PTR [rdx+0x110],rsi
  65c97a:	jne    65ca0b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x43b>
  65c980:	lea    rdi,[rsp+0x120]
  65c988:	lea    rsi,[rsp+0x58]
  65c98d:	xor    ecx,ecx
  65c98f:	call   663340 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_>
  65c994:	vmovups ymm0,YMMWORD PTR [rsp+0x160]
  65c99d:	vmovups ymm1,YMMWORD PTR [rsp+0x180]
  65c9a6:	vxorps ymm0,ymm0,YMMWORD PTR [rsp+0x120]
  65c9af:	vxorps ymm1,ymm1,YMMWORD PTR [rsp+0x140]
  65c9b8:	vmovups YMMWORD PTR [r12],ymm0
  65c9be:	vmovups YMMWORD PTR [r12+0x20],ymm1
  65c9c5:	add    rsp,0x1e8
  65c9cc:	pop    rbx
  65c9cd:	pop    r12
  65c9cf:	pop    r13
  65c9d1:	pop    r14
  65c9d3:	pop    r15
  65c9d5:	pop    rbp
  65c9d6:	vzeroupper 
  65c9d9:	ret    
  65c9da:	lea    rdi,[rip+0xffffffffff9b5d23]        # 12704 <anon.0e5291c933a25680de2bf032d375ada2.76.llvm.16073103944965275176+0x1>
  65c9e1:	lea    rdx,[rip+0x98c98]        # 6f5680 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x900>
  65c9e8:	mov    esi,0x37
  65c9ed:	call   QWORD PTR [rip+0x9b8dd]        # 6f82d0 <_DYNAMIC+0x2b8>
  65c9f3:	sub    rsi,0xffffffffffffff80
  65c9f7:	lea    rdi,[rsp+0x120]
  65c9ff:	lea    rdx,[rsp+0x58]
  65ca04:	call   655130 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B94_E0TB94_B94_EEB7q_>
  65ca09:	jmp    65c994 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x3c4>
  65ca0b:	sub    rsi,0xffffffffffffff80
  65ca0f:	lea    rdi,[rsp+0x120]
  65ca17:	lea    rcx,[rsp+0x58]
  65ca1c:	call   6560a0 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B95_E0TB95_B95_EEB7r_>
  65ca21:	jmp    65c994 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_+0x3c4>
  65ca26:	int3   
  65ca27:	int3   
  65ca28:	int3   
  65ca29:	int3   
  65ca2a:	int3   
  65ca2b:	int3   
  65ca2c:	int3   
  65ca2d:	int3   
  65ca2e:	int3   
  65ca2f:	int3   

