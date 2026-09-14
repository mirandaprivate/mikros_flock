00000000006562e0 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B9a_E0TB9a_B9a_EEB7r_>:
  6562e0:	push   rbp
  6562e1:	push   r15
  6562e3:	push   r14
  6562e5:	push   r13
  6562e7:	push   r12
  6562e9:	push   rbx
  6562ea:	sub    rsp,0x178
  6562f1:	mov    rbx,rdx
  6562f4:	lea    rax,[rdx+0x110]
  6562fb:	mov    rdx,QWORD PTR [rdx+0x100]
  656302:	mov    QWORD PTR [rsp+0x150],rax
  65630a:	mov    QWORD PTR [rsp+0x158],0x0
  656316:	mov    QWORD PTR [rsp+0x160],rdx
  65631e:	mov    BYTE PTR [rsp+0x168],0x1
  656326:	vmovups ymm0,YMMWORD PTR [rcx]
  65632a:	vmovups ymm1,YMMWORD PTR [rcx+0x20]
  65632f:	vmovups ymm2,YMMWORD PTR [rcx+0x40]
  656334:	vmovups ymm3,YMMWORD PTR [rcx+0x60]
  656339:	vmovups YMMWORD PTR [rsp],ymm0
  65633e:	vmovups YMMWORD PTR [rsp+0x20],ymm1
  656344:	vmovups YMMWORD PTR [rsp+0x40],ymm2
  65634a:	vmovups YMMWORD PTR [rsp+0x60],ymm3
  656350:	vmovups ymm0,YMMWORD PTR [rcx+0x80]
  656358:	vmovups YMMWORD PTR [rsp+0x80],ymm0
  656361:	vmovups ymm0,YMMWORD PTR [rcx+0xa0]
  656369:	vmovups YMMWORD PTR [rsp+0xa0],ymm0
  656372:	mov    r12,rsi
  656375:	mov    r14,rdi
  656378:	mov    rax,QWORD PTR [rcx+0xc0]
  65637f:	mov    QWORD PTR [rsp+0xc0],rax
  656387:	mov    QWORD PTR [rsp+0xc8],0x0
  656393:	mov    rbp,QWORD PTR [rsi]
  656396:	mov    r13,QWORD PTR [rsi+0x80]
  65639d:	lea    rsi,[rip+0x1e36c]        # 674710 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_Ba4_E0TBa4_Ba4_EE0Baz_ENtB5_3Job7executeB8l_>
  6563a4:	mov    rdx,rsp
  6563a7:	mov    rdi,r12
  6563aa:	vzeroupper 
  6563ad:	call   67de50 <_RNvMsg_NtCsh4KpTWVPfp2_15crossbeam_deque5dequeINtB5_8InjectorNtNtCsiKyBw8IgE1b_10rayon_core3job6JobRefE4pushB11_.llvm.10825995516073413387>
  6563b2:	lea    r15,[rsp+0x158]
  6563ba:	xor    r13,rbp
  6563bd:	movabs rdx,0x100000000
  6563c7:	lea    rdi,[r12+0x158]
  6563cf:	lock or DWORD PTR [rsp-0x40],0x0
  6563d5:	data16 cs nop WORD PTR [rax+rax*1+0x0]
  6563e0:	mov    rax,QWORD PTR [r12+0x170]
  6563e8:	test   rax,rdx
  6563eb:	jne    65640c <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B9a_E0TB9a_B9a_EEB7r_+0x12c>
  6563ed:	mov    rcx,rax
  6563f0:	or     rcx,rdx
  6563f3:	lock cmpxchg QWORD PTR [r12+0x170],rcx
  6563fd:	jne    6563e0 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B9a_E0TB9a_B9a_EEB7r_+0x100>
  6563ff:	mov    rax,rcx
  656402:	and    rax,0xffff
  656408:	jne    65641a <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B9a_E0TB9a_B9a_EEB7r_+0x13a>
  65640a:	jmp    65642f <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B9a_E0TB9a_B9a_EEB7r_+0x14f>
  65640c:	mov    rcx,rax
  65640f:	mov    rax,rcx
  656412:	and    rax,0xffff
  656418:	je     65642f <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B9a_E0TB9a_B9a_EEB7r_+0x14f>
  65641a:	cmp    r13,0x2
  65641e:	jae    6564c9 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B9a_E0TB9a_B9a_EEB7r_+0x1e9>
  656424:	shr    ecx,0x10
  656427:	cmp    ecx,eax
  656429:	je     6564c9 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B9a_E0TB9a_B9a_EEB7r_+0x1e9>
  65642f:	mov    rax,QWORD PTR [rsp+0x158]
  656437:	cmp    rax,0x3
  65643b:	jne    6564d9 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B9a_E0TB9a_B9a_EEB7r_+0x1f9>
  656441:	mov    rax,QWORD PTR [rsp+0xc8]
  656449:	vmovdqa xmm0,XMMWORD PTR [rsp+0xd0]
  656452:	cmp    rax,0x1
  656456:	je     656473 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B9a_E0TB9a_B9a_EEB7r_+0x193>
  656458:	cmp    rax,0x2
  65645c:	jne    6564ea <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B9a_E0TB9a_B9a_EEB7r_+0x20a>
  656462:	vmovq  rdi,xmm0
  656467:	vpextrq rsi,xmm0,0x1
  65646d:	call   QWORD PTR [rip+0xa2185]        # 6f85f8 <_DYNAMIC+0x5e0>
  656473:	vmovdqu XMMWORD PTR [r14],xmm0
  656478:	vmovups ymm0,YMMWORD PTR [rsp+0xe0]
  656481:	vmovups ymm1,YMMWORD PTR [rsp+0x100]
  65648a:	vmovups ymm2,YMMWORD PTR [rsp+0x120]
  656493:	vmovups ymm3,YMMWORD PTR [rsp+0x130]
  65649c:	vmovups YMMWORD PTR [r14+0x10],ymm0
  6564a2:	vmovups YMMWORD PTR [r14+0x30],ymm1
  6564a8:	vmovups YMMWORD PTR [r14+0x50],ymm2
  6564ae:	vmovups YMMWORD PTR [r14+0x60],ymm3
  6564b4:	add    rsp,0x178
  6564bb:	pop    rbx
  6564bc:	pop    r12
  6564be:	pop    r13
  6564c0:	pop    r14
  6564c2:	pop    r15
  6564c4:	pop    rbp
  6564c5:	vzeroupper 
  6564c8:	ret    
  6564c9:	mov    esi,0x1
  6564ce:	call   QWORD PTR [rip+0xa212c]        # 6f8600 <_DYNAMIC+0x5e8>
  6564d4:	jmp    65642f <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B9a_E0TB9a_B9a_EEB7r_+0x14f>
  6564d9:	mov    rdi,rbx
  6564dc:	mov    rsi,r15
  6564df:	call   QWORD PTR [rip+0xa2123]        # 6f8608 <_DYNAMIC+0x5f0>
  6564e5:	jmp    656441 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B9a_E0TB9a_B9a_EEB7r_+0x161>
  6564ea:	lea    rdi,[rip+0xffffffffff9bc16b]        # 1265c <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x160e>
  6564f1:	lea    rdx,[rip+0x9ebe8]        # 6f50e0 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x360>
  6564f8:	mov    esi,0x28
  6564fd:	call   QWORD PTR [rip+0xa1e5d]        # 6f8360 <_DYNAMIC+0x348>
  656503:	mov    rbx,rax
  656506:	mov    rdi,rsp
  656509:	call   656d90 <_RINvNtCsk9TLBDxQNFU_4core3ptr13drop_in_placeINtNtCsiKyBw8IgE1b_10rayon_core3job8StackJobNtNtBL_5latch9SpinLatchNCINvMs4_NtBL_8registryNtB1U_8Registry15in_worker_crossNCINvNtBL_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3g_9enumerate17EnumerateProducerINtNtB3g_3zip11ZipProducerINtNtNtB3i_5slice6chunks17ChunksMutProducerINtNtNtB4_3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB5p_EEINtNtB3g_3map11MapConsumerINtNtB3g_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8z_s0_0ENCB8z_s_0EE0NCB39_s_0ANtB6N_13F256Unreducedj2_Bah_E0TBah_Bah_EE0BaM_EEB8D_>
  65650e:	mov    rdi,rbx
  656511:	call   6f1930 <_Unwind_Resume@plt>
  656516:	call   QWORD PTR [rip+0xa1fac]        # 6f84c8 <_DYNAMIC+0x4b0>
  65651c:	int3   
  65651d:	int3   
  65651e:	int3   
  65651f:	int3   

