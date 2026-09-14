0000000000663340 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_>:
  663340:	push   rbp
  663341:	push   r15
  663343:	push   r14
  663345:	push   r13
  663347:	push   r12
  663349:	push   rbx
  66334a:	sub    rsp,0x2a8
  663351:	mov    DWORD PTR [rsp+0x14],ecx
  663355:	mov    r14,rdx
  663358:	mov    r12,rsi
  66335b:	mov    QWORD PTR [rsp+0x18],rdi
  663360:	lea    rax,[rdx+0x110]
  663367:	mov    rcx,QWORD PTR [rdx+0x100]
  66336e:	mov    QWORD PTR [rsp+0x100],rax
  663376:	mov    QWORD PTR [rsp+0x108],0x0
  663382:	mov    QWORD PTR [rsp+0x110],rcx
  66338a:	mov    BYTE PTR [rsp+0x118],0x0
  663392:	vmovups ymm0,YMMWORD PTR [rsi]
  663396:	vmovups ymm1,YMMWORD PTR [rsi+0x20]
  66339b:	vmovups ymm2,YMMWORD PTR [rsi+0x40]
  6633a0:	vmovups YMMWORD PTR [rsp+0x50],ymm0
  6633a6:	vmovups YMMWORD PTR [rsp+0x70],ymm1
  6633ac:	vmovups YMMWORD PTR [rsp+0x90],ymm2
  6633b5:	mov    rax,QWORD PTR [rsi+0x60]
  6633b9:	mov    QWORD PTR [rsp+0xb0],rax
  6633c1:	mov    QWORD PTR [rsp+0xb8],0x0
  6633cd:	mov    rax,QWORD PTR [rdx+0x118]
  6633d4:	mov    rbp,QWORD PTR [rax+0x108]
  6633db:	mov    r15,QWORD PTR [rax+0x100]
  6633e2:	mov    rax,QWORD PTR [rdx+0x118]
  6633e9:	mov    rbx,QWORD PTR [rax+0x108]
  6633f0:	mov    rax,QWORD PTR [rax+0x100]
  6633f7:	mov    rsi,QWORD PTR [rdx+0x128]
  6633fe:	mov    rcx,rbx
  663401:	sub    rcx,rax
  663404:	cmp    rcx,rsi
  663407:	jge    6637e8 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x4a8>
  66340d:	sub    rbp,r15
  663410:	movabs rcx,0x100000000
  66341a:	mov    rax,QWORD PTR [r14+0x120]
  663421:	dec    rsi
  663424:	and    rsi,rbx
  663427:	shl    rsi,0x4
  66342b:	lea    r15,[rip+0x11c0e]        # 675040 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2U_9enumerate17EnumerateProducerINtNtB2U_3zip11ZipProducerINtNtNtB2W_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtB1S_4F128EEB53_EEINtNtB2U_3map11MapConsumerINtNtB2U_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7W_s0_0ENCB7W_s_0EEs_0E0B1P_ENtB5_3Job7executeB80_>
  663432:	mov    QWORD PTR [rax+rsi*1],r15
  663436:	lea    r13,[rsp+0x50]
  66343b:	mov    QWORD PTR [rax+rsi*1+0x8],r13
  663440:	mov    rax,QWORD PTR [r14+0x118]
  663447:	inc    rbx
  66344a:	mov    QWORD PTR [rax+0x108],rbx
  663451:	mov    rsi,QWORD PTR [r14+0x110]
  663458:	lea    rdi,[rsi+0x1d8]
  66345f:	nop
  663460:	mov    rax,QWORD PTR [rsi+0x1f0]
  663467:	test   rax,rcx
  66346a:	jne    66348a <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x14a>
  66346c:	mov    rdx,rax
  66346f:	or     rdx,rcx
  663472:	lock cmpxchg QWORD PTR [rsi+0x1f0],rdx
  66347b:	jne    663460 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x120>
  66347d:	mov    rax,rdx
  663480:	and    rax,0xffff
  663486:	jne    663498 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x158>
  663488:	jmp    6634ac <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x16c>
  66348a:	mov    rdx,rax
  66348d:	mov    rax,rdx
  663490:	and    rax,0xffff
  663496:	je     6634ac <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x16c>
  663498:	test   rbp,rbp
  66349b:	jg     6637d5 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x495>
  6634a1:	shr    edx,0x10
  6634a4:	cmp    edx,eax
  6634a6:	je     6637d5 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x495>
  6634ac:	mov    rax,QWORD PTR [r12+0x68]
  6634b1:	mov    rdx,QWORD PTR [r12+0x70]
  6634b6:	vmovups ymm0,YMMWORD PTR [r12+0xa8]
  6634c0:	vmovups YMMWORD PTR [rsp+0x1c0],ymm0
  6634c9:	vmovups ymm0,YMMWORD PTR [r12+0x78]
  6634d0:	vmovups ymm1,YMMWORD PTR [r12+0x88]
  6634da:	vmovups YMMWORD PTR [rsp+0x190],ymm0
  6634e3:	vmovups YMMWORD PTR [rsp+0x1a0],ymm1
  6634ec:	mov    rsi,QWORD PTR [rax]
  6634ef:	mov    rcx,QWORD PTR [rdx]
  6634f2:	mov    r8,QWORD PTR [rdx+0x8]
  6634f6:	add    r12,0xb0
  6634fd:	mov    QWORD PTR [rsp],r12
  663501:	movzx  ebp,BYTE PTR [rsp+0x14]
  663506:	lea    rdi,[rsp+0x150]
  66350e:	lea    r9,[rsp+0x190]
  663516:	mov    edx,ebp
  663518:	vzeroupper 
  66351b:	call   65c5d0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_>
  663520:	mov    r12,QWORD PTR [rsp+0x150]
  663528:	mov    rbx,QWORD PTR [rsp+0x158]
  663530:	vmovups ymm0,YMMWORD PTR [rsp+0x170]
  663539:	vmovups YMMWORD PTR [rsp+0x30],ymm0
  66353f:	vmovups ymm0,YMMWORD PTR [rsp+0x160]
  663548:	vmovups YMMWORD PTR [rsp+0x20],ymm0
  66354e:	vmovups ymm0,YMMWORD PTR [rsp+0x20]
  663554:	vmovups ymm1,YMMWORD PTR [rsp+0x30]
  66355a:	vmovups YMMWORD PTR [rsp+0x120],ymm0
  663563:	vmovups YMMWORD PTR [rsp+0x130],ymm1
  66356c:	nop    DWORD PTR [rax+0x0]
  663570:	mov    rax,QWORD PTR [rsp+0x108]
  663578:	cmp    rax,0x3
  66357c:	je     6635bc <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x27c>
  66357e:	mov    rdi,r14
  663581:	vzeroupper 
  663584:	call   66b360 <_RNvMs8_NtCsiKyBw8IgE1b_10rayon_core8registryNtB5_12WorkerThread14take_local_job>
  663589:	test   rax,rax
  66358c:	je     6635aa <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x26a>
  66358e:	mov    rcx,r13
  663591:	xor    rcx,rdx
  663594:	mov    rsi,rax
  663597:	xor    rsi,r15
  66359a:	or     rsi,rcx
  66359d:	je     66363f <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x2ff>
  6635a3:	mov    rdi,rdx
  6635a6:	call   rax
  6635a8:	jmp    663570 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x230>
  6635aa:	mov    rax,QWORD PTR [rsp+0x108]
  6635b2:	cmp    rax,0x3
  6635b6:	jne    663822 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x4e2>
  6635bc:	mov    rax,QWORD PTR [rsp+0xb8]
  6635c4:	vmovdqa xmm0,XMMWORD PTR [rsp+0xc0]
  6635cd:	cmp    rax,0x1
  6635d1:	je     6635f1 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x2b1>
  6635d3:	cmp    rax,0x2
  6635d7:	jne    663806 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x4c6>
  6635dd:	vmovq  rdi,xmm0
  6635e2:	vpextrq rsi,xmm0,0x1
  6635e8:	vzeroupper 
  6635eb:	call   QWORD PTR [rip+0x95007]        # 6f85f8 <_DYNAMIC+0x5e0>
  6635f1:	vmovups ymm1,YMMWORD PTR [rsp+0xd0]
  6635fa:	vmovups ymm2,YMMWORD PTR [rsp+0xe0]
  663603:	mov    rax,QWORD PTR [rsp+0x18]
  663608:	vmovups YMMWORD PTR [rax+0x60],ymm2
  66360d:	vmovups YMMWORD PTR [rax+0x50],ymm1
  663612:	mov    QWORD PTR [rax],r12
  663615:	mov    QWORD PTR [rax+0x8],rbx
  663619:	vmovups ymm1,YMMWORD PTR [rsp+0x120]
  663622:	vmovups ymm2,YMMWORD PTR [rsp+0x130]
  66362b:	vmovups YMMWORD PTR [rax+0x10],ymm1
  663630:	vmovups YMMWORD PTR [rax+0x20],ymm2
  663635:	vmovdqu XMMWORD PTR [rax+0x40],xmm0
  66363a:	jmp    6637c0 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x480>
  66363f:	vmovups ymm0,YMMWORD PTR [rsp+0x50]
  663645:	vmovups ymm1,YMMWORD PTR [rsp+0x70]
  66364b:	vmovups ymm2,YMMWORD PTR [rsp+0x90]
  663654:	vmovups ymm3,YMMWORD PTR [rsp+0xb0]
  66365d:	vmovups YMMWORD PTR [rsp+0x190],ymm0
  663666:	vmovups ymm0,YMMWORD PTR [rsp+0x100]
  66366f:	vmovups YMMWORD PTR [rsp+0x240],ymm0
  663678:	vmovups ymm0,YMMWORD PTR [rsp+0xf0]
  663681:	vmovups YMMWORD PTR [rsp+0x230],ymm0
  66368a:	vmovups ymm0,YMMWORD PTR [rsp+0xd0]
  663693:	vmovups YMMWORD PTR [rsp+0x210],ymm0
  66369c:	vmovups YMMWORD PTR [rsp+0x1f0],ymm3
  6636a5:	vmovups YMMWORD PTR [rsp+0x1d0],ymm2
  6636ae:	vmovups YMMWORD PTR [rsp+0x1b0],ymm1
  6636b7:	mov    rax,QWORD PTR [rsp+0x190]
  6636bf:	test   rax,rax
  6636c2:	je     663838 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x4f8>
  6636c8:	mov    rcx,QWORD PTR [rsp+0x198]
  6636d0:	vmovups ymm0,YMMWORD PTR [rsp+0x68]
  6636d6:	vmovups ymm1,YMMWORD PTR [rsp+0x80]
  6636df:	vmovups YMMWORD PTR [rsp+0x168],ymm1
  6636e8:	vmovups YMMWORD PTR [rsp+0x150],ymm0
  6636f1:	mov    rdx,QWORD PTR [rsp+0x1a0]
  6636f9:	mov    rsi,QWORD PTR [rsp+0xb0]
  663701:	vmovaps xmm0,XMMWORD PTR [rsp+0xa0]
  66370a:	vmovaps XMMWORD PTR [rsp+0x20],xmm0
  663710:	mov    QWORD PTR [rsp+0x30],rsi
  663715:	mov    rsi,QWORD PTR [rax]
  663718:	sub    rsi,QWORD PTR [rcx]
  66371b:	mov    rcx,QWORD PTR [rdx]
  66371e:	mov    r8,QWORD PTR [rdx+0x8]
  663722:	lea    rax,[rsp+0x20]
  663727:	mov    QWORD PTR [rsp],rax
  66372b:	lea    rdi,[rsp+0x268]
  663733:	lea    r9,[rsp+0x150]
  66373b:	mov    edx,ebp
  66373d:	vzeroupper 
  663740:	call   65c5d0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_9enumerate17EnumerateProducerINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB2d_EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB5A_s0_0ENCB5A_s_0EEB5E_>
  663745:	cmp    DWORD PTR [rsp+0x1f8],0x2
  66374d:	jb     66377c <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x43c>
  66374f:	mov    r14,QWORD PTR [rsp+0x200]
  663757:	mov    r15,QWORD PTR [rsp+0x208]
  66375f:	mov    rax,QWORD PTR [r15]
  663762:	test   rax,rax
  663765:	je     66376c <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x42c>
  663767:	mov    rdi,r14
  66376a:	call   rax
  66376c:	cmp    QWORD PTR [r15+0x8],0x0
  663771:	je     66377c <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x43c>
  663773:	mov    rdi,r14
  663776:	call   QWORD PTR [rip+0x94aa4]        # 6f8220 <free@GLIBC_2.2.5>
  66377c:	vmovups ymm0,YMMWORD PTR [rsp+0x120]
  663785:	vmovups ymm1,YMMWORD PTR [rsp+0x130]
  66378e:	mov    rax,QWORD PTR [rsp+0x18]
  663793:	vmovups YMMWORD PTR [rax+0x20],ymm1
  663798:	vmovups YMMWORD PTR [rax+0x10],ymm0
  66379d:	mov    QWORD PTR [rax],r12
  6637a0:	mov    QWORD PTR [rax+0x8],rbx
  6637a4:	vmovdqu ymm0,YMMWORD PTR [rsp+0x268]
  6637ad:	vmovups ymm1,YMMWORD PTR [rsp+0x288]
  6637b6:	vmovdqu YMMWORD PTR [rax+0x40],ymm0
  6637bb:	vmovups YMMWORD PTR [rax+0x60],ymm1
  6637c0:	add    rsp,0x2a8
  6637c7:	pop    rbx
  6637c8:	pop    r12
  6637ca:	pop    r13
  6637cc:	pop    r14
  6637ce:	pop    r15
  6637d0:	pop    rbp
  6637d1:	vzeroupper 
  6637d4:	ret    
  6637d5:	mov    esi,0x1
  6637da:	vzeroupper 
  6637dd:	call   QWORD PTR [rip+0x94e1d]        # 6f8600 <_DYNAMIC+0x5e8>
  6637e3:	jmp    6634ac <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x16c>
  6637e8:	lea    rdi,[r14+0x118]
  6637ef:	add    rsi,rsi
  6637f2:	vzeroupper 
  6637f5:	call   66af10 <_RNvMs4_NtCsh4KpTWVPfp2_15crossbeam_deque5dequeINtB5_6WorkerNtNtCsiKyBw8IgE1b_10rayon_core3job6JobRefE6resizeCs5Palesvl9wF_6mikros>
  6637fa:	mov    rsi,QWORD PTR [r14+0x128]
  663801:	jmp    66340d <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0xcd>
  663806:	lea    rdi,[rip+0xffffffffff9aee4f]        # 1265c <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x160e>
  66380d:	lea    rdx,[rip+0x918cc]        # 6f50e0 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x360>
  663814:	mov    esi,0x28
  663819:	vzeroupper 
  66381c:	call   QWORD PTR [rip+0x94b3e]        # 6f8360 <_DYNAMIC+0x348>
  663822:	mov    rdi,r14
  663825:	lea    rsi,[rsp+0x108]
  66382d:	call   QWORD PTR [rip+0x94dd5]        # 6f8608 <_DYNAMIC+0x5f0>
  663833:	jmp    6635bc <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x27c>
  663838:	lea    rdi,[rip+0x91871]        # 6f50b0 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x330>
  66383f:	vzeroupper 
  663842:	call   QWORD PTR [rip+0x94b70]        # 6f83b8 <_DYNAMIC+0x3a0>
  663848:	jmp    663887 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x547>
  66384a:	jmp    66388b <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x54b>
  66384c:	mov    rbx,rax
  66384f:	cmp    QWORD PTR [r15+0x8],0x0
  663854:	je     6638b0 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x570>
  663856:	mov    rdi,r14
  663859:	call   QWORD PTR [rip+0x949c1]        # 6f8220 <free@GLIBC_2.2.5>
  66385f:	mov    rdi,rbx
  663862:	call   6f1930 <_Unwind_Resume@plt>
  663867:	mov    rdi,rax
  66386a:	call   QWORD PTR [rip+0x94de8]        # 6f8658 <_DYNAMIC+0x640>
  663870:	mov    rcx,rdx
  663873:	mov    rdi,r14
  663876:	lea    rsi,[rsp+0x100]
  66387e:	mov    rdx,rax
  663881:	call   QWORD PTR [rip+0x94dd9]        # 6f8660 <_DYNAMIC+0x648>
  663887:	ud2    
  663889:	jmp    66388b <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x54b>
  66388b:	mov    rbx,rax
  66388e:	lea    rdi,[rsp+0x50]
  663893:	call   656e50 <_RINvNtCsk9TLBDxQNFU_4core3ptr13drop_in_placeINtNtCsiKyBw8IgE1b_10rayon_core3job8StackJobNtNtBL_5latch9SpinLatchNCINvNvNtBL_4join12join_context6call_bANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_NCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3s_9enumerate17EnumerateProducerINtNtB3s_3zip11ZipProducerINtNtNtB3u_5slice6chunks17ChunksMutProducerINtNtNtB4_3mem12maybe_uninit11MaybeUninitNtB2q_4F128EEB5B_EEINtNtB3s_3map11MapConsumerINtNtB3s_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8e_s0_0ENCB8e_s_0EEs_0E0B2n_EEB8i_>
  663898:	jmp    6638b0 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_+0x570>
  66389a:	call   QWORD PTR [rip+0x94c28]        # 6f84c8 <_DYNAMIC+0x4b0>
  6638a0:	mov    rbx,rax
  6638a3:	lea    rdi,[rsp+0x1f8]
  6638ab:	call   656760 <_RINvNtCsk9TLBDxQNFU_4core3ptr13drop_in_placeINtNtB4_4cell10UnsafeCellINtNtCsiKyBw8IgE1b_10rayon_core3job9JobResultANtNtCskPVWNEMsUTt_11flock_field7gf2_12813F256Unreducedj2_EEECs5Palesvl9wF_6mikros>
  6638b0:	mov    rdi,rbx
  6638b3:	call   6f1930 <_Unwind_Resume@plt>
  6638b8:	call   QWORD PTR [rip+0x94c0a]        # 6f84c8 <_DYNAMIC+0x4b0>
  6638be:	int3   
  6638bf:	int3   

