00000000006560a0 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B95_E0TB95_B95_EEB7r_>:
  6560a0:	push   rbp
  6560a1:	push   r15
  6560a3:	push   r14
  6560a5:	push   r13
  6560a7:	push   r12
  6560a9:	push   rbx
  6560aa:	sub    rsp,0x178
  6560b1:	mov    rbx,rdx
  6560b4:	lea    rax,[rdx+0x110]
  6560bb:	mov    rdx,QWORD PTR [rdx+0x100]
  6560c2:	mov    QWORD PTR [rsp+0x150],rax
  6560ca:	mov    QWORD PTR [rsp+0x158],0x0
  6560d6:	mov    QWORD PTR [rsp+0x160],rdx
  6560de:	mov    BYTE PTR [rsp+0x168],0x1
  6560e6:	vmovups ymm0,YMMWORD PTR [rcx]
  6560ea:	vmovups ymm1,YMMWORD PTR [rcx+0x20]
  6560ef:	vmovups ymm2,YMMWORD PTR [rcx+0x40]
  6560f4:	vmovups ymm3,YMMWORD PTR [rcx+0x60]
  6560f9:	vmovups YMMWORD PTR [rsp],ymm0
  6560fe:	vmovups YMMWORD PTR [rsp+0x20],ymm1
  656104:	vmovups YMMWORD PTR [rsp+0x40],ymm2
  65610a:	vmovups YMMWORD PTR [rsp+0x60],ymm3
  656110:	vmovups ymm0,YMMWORD PTR [rcx+0x80]
  656118:	vmovups YMMWORD PTR [rsp+0x80],ymm0
  656121:	vmovups ymm0,YMMWORD PTR [rcx+0xa0]
  656129:	vmovups YMMWORD PTR [rsp+0xa0],ymm0
  656132:	mov    r12,rsi
  656135:	mov    r14,rdi
  656138:	mov    rax,QWORD PTR [rcx+0xc0]
  65613f:	mov    QWORD PTR [rsp+0xc0],rax
  656147:	mov    QWORD PTR [rsp+0xc8],0x0
  656153:	mov    rbp,QWORD PTR [rsi]
  656156:	mov    r13,QWORD PTR [rsi+0x80]
  65615d:	lea    rsi,[rip+0x1e24c]        # 6743b0 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_B9Z_E0TB9Z_B9Z_EE0Bau_ENtB5_3Job7executeB8l_>
  656164:	mov    rdx,rsp
  656167:	mov    rdi,r12
  65616a:	vzeroupper 
  65616d:	call   67de50 <_RNvMsg_NtCsh4KpTWVPfp2_15crossbeam_deque5dequeINtB5_8InjectorNtNtCsiKyBw8IgE1b_10rayon_core3job6JobRefE4pushB11_.llvm.10825995516073413387>
  656172:	lea    r15,[rsp+0x158]
  65617a:	xor    r13,rbp
  65617d:	movabs rdx,0x100000000
  656187:	lea    rdi,[r12+0x158]
  65618f:	lock or DWORD PTR [rsp-0x40],0x0
  656195:	data16 cs nop WORD PTR [rax+rax*1+0x0]
  6561a0:	mov    rax,QWORD PTR [r12+0x170]
  6561a8:	test   rax,rdx
  6561ab:	jne    6561cc <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B95_E0TB95_B95_EEB7r_+0x12c>
  6561ad:	mov    rcx,rax
  6561b0:	or     rcx,rdx
  6561b3:	lock cmpxchg QWORD PTR [r12+0x170],rcx
  6561bd:	jne    6561a0 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B95_E0TB95_B95_EEB7r_+0x100>
  6561bf:	mov    rax,rcx
  6561c2:	and    rax,0xffff
  6561c8:	jne    6561da <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B95_E0TB95_B95_EEB7r_+0x13a>
  6561ca:	jmp    6561ef <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B95_E0TB95_B95_EEB7r_+0x14f>
  6561cc:	mov    rcx,rax
  6561cf:	mov    rax,rcx
  6561d2:	and    rax,0xffff
  6561d8:	je     6561ef <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B95_E0TB95_B95_EEB7r_+0x14f>
  6561da:	cmp    r13,0x2
  6561de:	jae    656289 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B95_E0TB95_B95_EEB7r_+0x1e9>
  6561e4:	shr    ecx,0x10
  6561e7:	cmp    ecx,eax
  6561e9:	je     656289 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B95_E0TB95_B95_EEB7r_+0x1e9>
  6561ef:	mov    rax,QWORD PTR [rsp+0x158]
  6561f7:	cmp    rax,0x3
  6561fb:	jne    656299 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B95_E0TB95_B95_EEB7r_+0x1f9>
  656201:	mov    rax,QWORD PTR [rsp+0xc8]
  656209:	vmovdqa xmm0,XMMWORD PTR [rsp+0xd0]
  656212:	cmp    rax,0x1
  656216:	je     656233 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B95_E0TB95_B95_EEB7r_+0x193>
  656218:	cmp    rax,0x2
  65621c:	jne    6562aa <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B95_E0TB95_B95_EEB7r_+0x20a>
  656222:	vmovq  rdi,xmm0
  656227:	vpextrq rsi,xmm0,0x1
  65622d:	call   QWORD PTR [rip+0xa23c5]        # 6f85f8 <_DYNAMIC+0x5e0>
  656233:	vmovdqu XMMWORD PTR [r14],xmm0
  656238:	vmovups ymm0,YMMWORD PTR [rsp+0xe0]
  656241:	vmovups ymm1,YMMWORD PTR [rsp+0x100]
  65624a:	vmovups ymm2,YMMWORD PTR [rsp+0x120]
  656253:	vmovups ymm3,YMMWORD PTR [rsp+0x130]
  65625c:	vmovups YMMWORD PTR [r14+0x10],ymm0
  656262:	vmovups YMMWORD PTR [r14+0x30],ymm1
  656268:	vmovups YMMWORD PTR [r14+0x50],ymm2
  65626e:	vmovups YMMWORD PTR [r14+0x60],ymm3
  656274:	add    rsp,0x178
  65627b:	pop    rbx
  65627c:	pop    r12
  65627e:	pop    r13
  656280:	pop    r14
  656282:	pop    r15
  656284:	pop    rbp
  656285:	vzeroupper 
  656288:	ret    
  656289:	mov    esi,0x1
  65628e:	call   QWORD PTR [rip+0xa236c]        # 6f8600 <_DYNAMIC+0x5e8>
  656294:	jmp    6561ef <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B95_E0TB95_B95_EEB7r_+0x14f>
  656299:	mov    rdi,rbx
  65629c:	mov    rsi,r15
  65629f:	call   QWORD PTR [rip+0xa2363]        # 6f8608 <_DYNAMIC+0x5f0>
  6562a5:	jmp    656201 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_9enumerate17EnumerateProducerINtNtB1O_3zip11ZipProducerINtNtNtB1Q_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3X_EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7n_s0_0ENCB7n_s_0EE0NCB1H_s_0ANtB5B_13F256Unreducedj2_B95_E0TB95_B95_EEB7r_+0x161>
  6562aa:	lea    rdi,[rip+0xffffffffff9bc3ab]        # 1265c <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x160e>
  6562b1:	lea    rdx,[rip+0x9ee28]        # 6f50e0 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x360>
  6562b8:	mov    esi,0x28
  6562bd:	call   QWORD PTR [rip+0xa209d]        # 6f8360 <_DYNAMIC+0x348>
  6562c3:	mov    rbx,rax
  6562c6:	mov    rdi,rsp
  6562c9:	call   656d90 <_RINvNtCsk9TLBDxQNFU_4core3ptr13drop_in_placeINtNtCsiKyBw8IgE1b_10rayon_core3job8StackJobNtNtBL_5latch9SpinLatchNCINvMs4_NtBL_8registryNtB1U_8Registry15in_worker_crossNCINvNtBL_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3g_9enumerate17EnumerateProducerINtNtB3g_3zip11ZipProducerINtNtNtB3i_5slice6chunks17ChunksMutProducerINtNtNtB4_3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB5p_EEINtNtB3g_3map11MapConsumerINtNtB3g_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8z_s0_0ENCB8z_s_0EE0NCB39_s_0ANtB6N_13F256Unreducedj2_Bah_E0TBah_Bah_EE0BaM_EEB8D_>
  6562ce:	mov    rdi,rbx
  6562d1:	call   6f1930 <_Unwind_Resume@plt>
  6562d6:	call   QWORD PTR [rip+0xa21ec]        # 6f84c8 <_DYNAMIC+0x4b0>
  6562dc:	int3   
  6562dd:	int3   
  6562de:	int3   
  6562df:	int3   

