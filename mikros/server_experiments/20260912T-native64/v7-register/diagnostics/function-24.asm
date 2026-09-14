00000000006752c0 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_>:
  6752c0:	push   rbp
  6752c1:	push   r15
  6752c3:	push   r14
  6752c5:	push   r13
  6752c7:	push   r12
  6752c9:	push   rbx
  6752ca:	sub    rsp,0xe8
  6752d1:	mov    rcx,QWORD PTR [rdi]
  6752d4:	mov    rax,QWORD PTR [rdi+0x8]
  6752d8:	mov    QWORD PTR [rdi],0x0
  6752df:	test   rcx,rcx
  6752e2:	je     67548f <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x1cf>
  6752e8:	mov    rbx,rdi
  6752eb:	mov    rdx,QWORD PTR [rdi+0x60]
  6752ef:	mov    QWORD PTR [rsp+0xa0],rdx
  6752f7:	vmovups ymm0,YMMWORD PTR [rdi+0x40]
  6752fc:	vmovups YMMWORD PTR [rsp+0x80],ymm0
  675305:	vmovups ymm0,YMMWORD PTR [rdi+0x10]
  67530a:	vmovups ymm1,YMMWORD PTR [rdi+0x20]
  67530f:	vmovups YMMWORD PTR [rsp+0x50],ymm0
  675315:	vmovups YMMWORD PTR [rsp+0x60],ymm1
  67531b:	mov    QWORD PTR [rsp+0x40],rcx
  675320:	mov    QWORD PTR [rsp+0x48],rax
  675325:	mov    rdx,QWORD PTR [rsp+0x50]
  67532a:	lea    r9,[rsp+0x58]
  67532f:	lea    rdi,[rsp+0x90]
  675337:	mov    rsi,QWORD PTR [rcx]
  67533a:	sub    rsi,QWORD PTR [rax]
  67533d:	mov    rcx,QWORD PTR [rdx]
  675340:	mov    r8,QWORD PTR [rdx+0x8]
  675344:	mov    QWORD PTR [rsp],rdi
  675348:	lea    rdi,[rsp+0xa8]
  675350:	mov    edx,0x1
  675355:	vzeroupper 
  675358:	call   65ca30 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_>
  67535d:	mov    r14,QWORD PTR [rsp+0xa8]
  675365:	vmovups ymm0,YMMWORD PTR [rsp+0xc8]
  67536e:	vmovups YMMWORD PTR [rsp+0x50],ymm0
  675374:	vmovups ymm0,YMMWORD PTR [rsp+0xb8]
  67537d:	vmovups YMMWORD PTR [rsp+0x40],ymm0
  675383:	vmovups ymm0,YMMWORD PTR [rsp+0x40]
  675389:	vmovups ymm1,YMMWORD PTR [rsp+0x50]
  67538f:	mov    r15,QWORD PTR [rsp+0xb0]
  675397:	vmovups YMMWORD PTR [rsp+0x10],ymm0
  67539d:	vmovups YMMWORD PTR [rsp+0x20],ymm1
  6753a3:	mov    r13d,0x1
  6753a9:	cmp    DWORD PTR [rbx+0x68],0x2
  6753ad:	jb     6753db <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x11b>
  6753af:	mov    r12,QWORD PTR [rbx+0x70]
  6753b3:	mov    rbp,QWORD PTR [rbx+0x78]
  6753b7:	mov    rax,QWORD PTR [rbp+0x0]
  6753bb:	test   rax,rax
  6753be:	je     6753c8 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x108>
  6753c0:	mov    rdi,r12
  6753c3:	vzeroupper 
  6753c6:	call   rax
  6753c8:	cmp    QWORD PTR [rbp+0x8],0x0
  6753cd:	je     6753db <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x11b>
  6753cf:	mov    rdi,r12
  6753d2:	vzeroupper 
  6753d5:	call   QWORD PTR [rip+0x82e45]        # 6f8220 <free@GLIBC_2.2.5>
  6753db:	mov    QWORD PTR [rbx+0x68],r13
  6753df:	mov    QWORD PTR [rbx+0x70],r14
  6753e3:	mov    QWORD PTR [rbx+0x78],r15
  6753e7:	vmovups ymm0,YMMWORD PTR [rsp+0x10]
  6753ed:	vmovups ymm1,YMMWORD PTR [rsp+0x20]
  6753f3:	vmovups YMMWORD PTR [rbx+0x80],ymm0
  6753fb:	vmovups YMMWORD PTR [rbx+0x90],ymm1
  675403:	movzx  ebp,BYTE PTR [rbx+0xc8]
  67540a:	test   bpl,bpl
  67540d:	je     67542a <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x16a>
  67540f:	mov    rax,QWORD PTR [rbx+0xb0]
  675416:	mov    rdi,QWORD PTR [rax]
  675419:	lock inc QWORD PTR [rdi]
  67541d:	jle    675530 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x270>
  675423:	mov    QWORD PTR [rsp+0x10],rdi
  675428:	jmp    675434 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x174>
  67542a:	mov    rax,QWORD PTR [rbx+0xb0]
  675431:	mov    rdi,QWORD PTR [rax]
  675434:	mov    rsi,QWORD PTR [rbx+0xc0]
  67543b:	mov    eax,0x3
  675440:	xchg   QWORD PTR [rbx+0xb8],rax
  675447:	cmp    rax,0x2
  67544b:	jne    67545c <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x19c>
  67544d:	add    rdi,0x1d8
  675454:	vzeroupper 
  675457:	call   67c660 <_RNvMNtCsiKyBw8IgE1b_10rayon_core5sleepNtB2_5Sleep20wake_specific_thread.llvm.10825995516073413387>
  67545c:	test   bpl,bpl
  67545f:	je     67547a <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x1ba>
  675461:	mov    rax,QWORD PTR [rsp+0x10]
  675466:	lock dec QWORD PTR [rax]
  67546a:	jne    67547a <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x1ba>
  67546c:	lea    rdi,[rsp+0x10]
  675471:	vzeroupper 
  675474:	call   QWORD PTR [rip+0x8331e]        # 6f8798 <_DYNAMIC+0x780>
  67547a:	add    rsp,0xe8
  675481:	pop    rbx
  675482:	pop    r12
  675484:	pop    r13
  675486:	pop    r14
  675488:	pop    r15
  67548a:	pop    rbp
  67548b:	vzeroupper 
  67548e:	ret    
  67548f:	lea    rdi,[rip+0x80172]        # 6f5608 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x888>
  675496:	call   QWORD PTR [rip+0x82f1c]        # 6f83b8 <_DYNAMIC+0x3a0>
  67549c:	jmp    675530 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x270>
  6754a1:	cmp    QWORD PTR [rbp+0x8],0x0
  6754a6:	je     6754b1 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x1f1>
  6754a8:	mov    rdi,r12
  6754ab:	call   QWORD PTR [rip+0x82d6f]        # 6f8220 <free@GLIBC_2.2.5>
  6754b1:	mov    QWORD PTR [rbx+0x68],r13
  6754b5:	mov    QWORD PTR [rbx+0x70],r14
  6754b9:	mov    QWORD PTR [rbx+0x78],r15
  6754bd:	vmovups ymm0,YMMWORD PTR [rsp+0x10]
  6754c3:	vmovups ymm1,YMMWORD PTR [rsp+0x20]
  6754c9:	vmovups YMMWORD PTR [rbx+0x80],ymm0
  6754d1:	vmovups YMMWORD PTR [rbx+0x90],ymm1
  6754d9:	jmp    675522 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x262>
  6754db:	test   bpl,bpl
  6754de:	je     675522 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x262>
  6754e0:	mov    rax,QWORD PTR [rsp+0x10]
  6754e5:	lock dec QWORD PTR [rax]
  6754e9:	jne    675522 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x262>
  6754eb:	lea    rdi,[rsp+0x10]
  6754f0:	call   QWORD PTR [rip+0x832a2]        # 6f8798 <_DYNAMIC+0x780>
  6754f6:	jmp    675522 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x262>
  6754f8:	call   QWORD PTR [rip+0x82fca]        # 6f84c8 <_DYNAMIC+0x4b0>
  6754fe:	mov    rdi,rax
  675501:	call   QWORD PTR [rip+0x83151]        # 6f8658 <_DYNAMIC+0x640>
  675507:	mov    r14,rax
  67550a:	mov    r15,rdx
  67550d:	mov    r13d,0x2
  675513:	cmp    DWORD PTR [rbx+0x68],0x2
  675517:	jae    6753af <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0xef>
  67551d:	jmp    6753db <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_+0x11b>
  675522:	lea    rdi,[rsp+0xf]
  675527:	vzeroupper 
  67552a:	call   QWORD PTR [rip+0x83260]        # 6f8790 <_DYNAMIC+0x778>
  675530:	ud2    
  675532:	call   QWORD PTR [rip+0x82f90]        # 6f84c8 <_DYNAMIC+0x4b0>
  675538:	int3   
  675539:	int3   
  67553a:	int3   
  67553b:	int3   
  67553c:	int3   
  67553d:	int3   
  67553e:	int3   
  67553f:	int3   

