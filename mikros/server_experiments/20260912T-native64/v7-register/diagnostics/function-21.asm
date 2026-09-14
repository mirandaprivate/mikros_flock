00000000006743b0 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_B9Z_E0TB9Z_B9Z_EE0Bau_ENtB5_3Job7executeB8l_>:
  6743b0:	push   rbp
  6743b1:	push   r15
  6743b3:	push   r14
  6743b5:	push   r13
  6743b7:	push   r12
  6743b9:	push   rbx
  6743ba:	sub    rsp,0x1c8
  6743c1:	vmovups xmm0,XMMWORD PTR [rdi]
  6743c5:	cmp    QWORD PTR [rdi],0x0
  6743c9:	mov    QWORD PTR [rdi],0x0
  6743d0:	je     674636 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_B9Z_E0TB9Z_B9Z_EE0Bau_ENtB5_3Job7executeB8l_+0x286>
  6743d6:	mov    rbx,rdi
  6743d9:	mov    rax,0xffffffffffffffa8
  6743e0:	mov    rdx,QWORD PTR fs:[rax]
  6743e4:	test   rdx,rdx
  6743e7:	je     674618 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_B9Z_E0TB9Z_B9Z_EE0Bau_ENtB5_3Job7executeB8l_+0x268>
  6743ed:	lea    rax,[rbx+0x10]
  6743f1:	lea    rcx,[rbx+0x80]
  6743f8:	vmovaps XMMWORD PTR [rsp+0x80],xmm0
  674401:	vmovups ymm0,YMMWORD PTR [rax]
  674405:	vmovups ymm1,YMMWORD PTR [rax+0x20]
  67440a:	vmovups ymm2,YMMWORD PTR [rax+0x40]
  67440f:	vmovups ymm3,YMMWORD PTR [rax+0x50]
  674414:	vmovups YMMWORD PTR [rsp+0x90],ymm0
  67441d:	vmovups YMMWORD PTR [rsp+0xb0],ymm1
  674426:	vmovups YMMWORD PTR [rsp+0xd0],ymm2
  67442f:	vmovups YMMWORD PTR [rsp+0xe0],ymm3
  674438:	vmovups ymm0,YMMWORD PTR [rcx]
  67443c:	vmovups ymm1,YMMWORD PTR [rcx+0x20]
  674441:	vmovups YMMWORD PTR [rsp+0x100],ymm0
  67444a:	vmovups YMMWORD PTR [rsp+0x120],ymm1
  674453:	mov    rax,QWORD PTR [rcx+0x40]
  674457:	mov    QWORD PTR [rsp+0x140],rax
  67445f:	lea    rdi,[rsp+0x148]
  674467:	lea    rsi,[rsp+0x80]
  67446f:	mov    ecx,0x1
  674474:	vzeroupper 
  674477:	call   663340 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_>
  67447c:	mov    r14,QWORD PTR [rsp+0x148]
  674484:	vmovups ymm0,YMMWORD PTR [rsp+0x1a8]
  67448d:	vmovups YMMWORD PTR [rsp+0xd0],ymm0
  674496:	vmovups ymm0,YMMWORD PTR [rsp+0x198]
  67449f:	vmovups YMMWORD PTR [rsp+0xc0],ymm0
  6744a8:	vmovups ymm0,YMMWORD PTR [rsp+0x158]
  6744b1:	vmovups YMMWORD PTR [rsp+0x80],ymm0
  6744ba:	vmovups ymm1,YMMWORD PTR [rsp+0x178]
  6744c3:	vmovups YMMWORD PTR [rsp+0xa0],ymm1
  6744cc:	vmovups YMMWORD PTR [rsp+0x10],ymm0
  6744d2:	vmovups YMMWORD PTR [rsp+0x30],ymm1
  6744d8:	vmovups ymm0,YMMWORD PTR [rsp+0xc0]
  6744e1:	vmovups ymm1,YMMWORD PTR [rsp+0xd0]
  6744ea:	mov    r15,QWORD PTR [rsp+0x150]
  6744f2:	vmovups YMMWORD PTR [rsp+0x50],ymm0
  6744f8:	vmovups YMMWORD PTR [rsp+0x60],ymm1
  6744fe:	mov    r13d,0x1
  674504:	cmp    DWORD PTR [rbx+0xc8],0x2
  67450b:	jb     67453f <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_B9Z_E0TB9Z_B9Z_EE0Bau_ENtB5_3Job7executeB8l_+0x18f>
  67450d:	mov    r12,QWORD PTR [rbx+0xd0]
  674514:	mov    rbp,QWORD PTR [rbx+0xd8]
  67451b:	mov    rax,QWORD PTR [rbp+0x0]
  67451f:	test   rax,rax
  674522:	je     67452c <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_B9Z_E0TB9Z_B9Z_EE0Bau_ENtB5_3Job7executeB8l_+0x17c>
  674524:	mov    rdi,r12
  674527:	vzeroupper 
  67452a:	call   rax
  67452c:	cmp    QWORD PTR [rbp+0x8],0x0
  674531:	je     67453f <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_B9Z_E0TB9Z_B9Z_EE0Bau_ENtB5_3Job7executeB8l_+0x18f>
  674533:	mov    rdi,r12
  674536:	vzeroupper 
  674539:	call   QWORD PTR [rip+0x83ce1]        # 6f8220 <free@GLIBC_2.2.5>
  67453f:	mov    QWORD PTR [rbx+0xc8],r13
  674546:	mov    QWORD PTR [rbx+0xd0],r14
  67454d:	mov    QWORD PTR [rbx+0xd8],r15
  674554:	vmovups ymm0,YMMWORD PTR [rsp+0x10]
  67455a:	vmovups ymm1,YMMWORD PTR [rsp+0x30]
  674560:	vmovups ymm2,YMMWORD PTR [rsp+0x50]
  674566:	vmovups ymm3,YMMWORD PTR [rsp+0x60]
  67456c:	vmovups YMMWORD PTR [rbx+0xe0],ymm0
  674574:	vmovups YMMWORD PTR [rbx+0x100],ymm1
  67457c:	vmovups YMMWORD PTR [rbx+0x120],ymm2
  674584:	vmovups YMMWORD PTR [rbx+0x130],ymm3
  67458c:	movzx  ebp,BYTE PTR [rbx+0x168]
  674593:	test   bpl,bpl
  674596:	je     6745b3 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_B9Z_E0TB9Z_B9Z_EE0Bau_ENtB5_3Job7executeB8l_+0x203>
  674598:	mov    rax,QWORD PTR [rbx+0x150]
  67459f:	mov    rdi,QWORD PTR [rax]
  6745a2:	lock inc QWORD PTR [rdi]
  6745a6:	jle    6746d8 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_B9Z_E0TB9Z_B9Z_EE0Bau_ENtB5_3Job7executeB8l_+0x328>
  6745ac:	mov    QWORD PTR [rsp+0x10],rdi
  6745b1:	jmp    6745bd <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_B9Z_E0TB9Z_B9Z_EE0Bau_ENtB5_3Job7executeB8l_+0x20d>
  6745b3:	mov    rax,QWORD PTR [rbx+0x150]
  6745ba:	mov    rdi,QWORD PTR [rax]
  6745bd:	mov    rsi,QWORD PTR [rbx+0x160]
  6745c4:	mov    eax,0x3
  6745c9:	xchg   QWORD PTR [rbx+0x158],rax
  6745d0:	cmp    rax,0x2
  6745d4:	jne    6745e5 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_B9Z_E0TB9Z_B9Z_EE0Bau_ENtB5_3Job7executeB8l_+0x235>
  6745d6:	add    rdi,0x1d8
  6745dd:	vzeroupper 
  6745e0:	call   67c660 <_RNvMNtCsiKyBw8IgE1b_10rayon_core5sleepNtB2_5Sleep20wake_specific_thread.llvm.10825995516073413387>
  6745e5:	test   bpl,bpl
  6745e8:	je     674603 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_B9Z_E0TB9Z_B9Z_EE0Bau_ENtB5_3Job7executeB8l_+0x253>
  6745ea:	mov    rax,QWORD PTR [rsp+0x10]
  6745ef:	lock dec QWORD PTR [rax]
  6745f3:	jne    674603 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_B9Z_E0TB9Z_B9Z_EE0Bau_ENtB5_3Job7executeB8l_+0x253>
  6745f5:	lea    rdi,[rsp+0x10]
  6745fa:	vzeroupper 
  6745fd:	call   QWORD PTR [rip+0x84195]        # 6f8798 <_DYNAMIC+0x780>
  674603:	add    rsp,0x1c8
  67460a:	pop    rbx
  67460b:	pop    r12
  67460d:	pop    r13
  67460f:	pop    r14
  674611:	pop    r15
  674613:	pop    rbp
  674614:	vzeroupper 
  674617:	ret    
  674618:	lea    rdi,[rip+0xffffffffff99e065]        # 12684 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1636>
  67461f:	lea    rdx,[rip+0x807ea]        # 6f4e10 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x90>
  674626:	mov    esi,0x36
  67462b:	call   QWORD PTR [rip+0x83d2f]        # 6f8360 <_DYNAMIC+0x348>
  674631:	jmp    6746d8 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_B9Z_E0TB9Z_B9Z_EE0Bau_ENtB5_3Job7executeB8l_+0x328>
  674636:	lea    rdi,[rip+0x80fcb]        # 6f5608 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x888>
  67463d:	call   QWORD PTR [rip+0x83d75]        # 6f83b8 <_DYNAMIC+0x3a0>
  674643:	jmp    6746d8 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_B9Z_E0TB9Z_B9Z_EE0Bau_ENtB5_3Job7executeB8l_+0x328>
  674648:	cmp    QWORD PTR [rbp+0x8],0x0
  67464d:	je     674658 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_B9Z_E0TB9Z_B9Z_EE0Bau_ENtB5_3Job7executeB8l_+0x2a8>
  67464f:	mov    rdi,r12
  674652:	call   QWORD PTR [rip+0x83bc8]        # 6f8220 <free@GLIBC_2.2.5>
  674658:	mov    QWORD PTR [rbx+0xc8],r13
  67465f:	mov    QWORD PTR [rbx+0xd0],r14
  674666:	mov    QWORD PTR [rbx+0xd8],r15
  67466d:	vmovups ymm0,YMMWORD PTR [rsp+0x10]
  674673:	vmovups ymm1,YMMWORD PTR [rsp+0x30]
  674679:	vmovups ymm2,YMMWORD PTR [rsp+0x50]
  67467f:	vmovups ymm3,YMMWORD PTR [rsp+0x60]
  674685:	vmovups YMMWORD PTR [rbx+0xe0],ymm0
  67468d:	vmovups YMMWORD PTR [rbx+0x100],ymm1
  674695:	vmovups YMMWORD PTR [rbx+0x120],ymm2
  67469d:	vmovups YMMWORD PTR [rbx+0x130],ymm3
  6746a5:	jmp    6746ca <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_B9Z_E0TB9Z_B9Z_EE0Bau_ENtB5_3Job7executeB8l_+0x31a>
  6746a7:	test   bpl,bpl
  6746aa:	je     6746ca <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_B9Z_E0TB9Z_B9Z_EE0Bau_ENtB5_3Job7executeB8l_+0x31a>
  6746ac:	mov    rax,QWORD PTR [rsp+0x10]
  6746b1:	lock dec QWORD PTR [rax]
  6746b5:	jne    6746ca <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_B9Z_E0TB9Z_B9Z_EE0Bau_ENtB5_3Job7executeB8l_+0x31a>
  6746b7:	lea    rdi,[rsp+0x10]
  6746bc:	call   QWORD PTR [rip+0x840d6]        # 6f8798 <_DYNAMIC+0x780>
  6746c2:	jmp    6746ca <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_B9Z_E0TB9Z_B9Z_EE0Bau_ENtB5_3Job7executeB8l_+0x31a>
  6746c4:	call   QWORD PTR [rip+0x83dfe]        # 6f84c8 <_DYNAMIC+0x4b0>
  6746ca:	lea    rdi,[rsp+0xf]
  6746cf:	vzeroupper 
  6746d2:	call   QWORD PTR [rip+0x840b8]        # 6f8790 <_DYNAMIC+0x778>
  6746d8:	ud2    
  6746da:	call   QWORD PTR [rip+0x83de8]        # 6f84c8 <_DYNAMIC+0x4b0>
  6746e0:	mov    rdi,rax
  6746e3:	call   QWORD PTR [rip+0x83f6f]        # 6f8658 <_DYNAMIC+0x640>
  6746e9:	mov    r14,rax
  6746ec:	mov    r15,rdx
  6746ef:	mov    r13d,0x2
  6746f5:	cmp    DWORD PTR [rbx+0xc8],0x2
  6746fc:	jae    67450d <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_B9Z_E0TB9Z_B9Z_EE0Bau_ENtB5_3Job7executeB8l_+0x15d>
  674702:	jmp    67453f <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_B9Z_E0TB9Z_B9Z_EE0Bau_ENtB5_3Job7executeB8l_+0x18f>
  674707:	int3   
  674708:	int3   
  674709:	int3   
  67470a:	int3   
  67470b:	int3   
  67470c:	int3   
  67470d:	int3   
  67470e:	int3   
  67470f:	int3   

