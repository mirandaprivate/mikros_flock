0000000000655320 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B99_E0TB99_B99_EEB7q_>:
  655320:	push   r15
  655322:	push   r14
  655324:	push   r12
  655326:	push   rbx
  655327:	sub    rsp,0x158
  65532e:	mov    rax,QWORD PTR fs:0x0
  655337:	lea    rax,[rax-0x50]
  65533e:	mov    QWORD PTR [rsp],rax
  655342:	vmovups ymm0,YMMWORD PTR [rdx]
  655346:	vmovups ymm1,YMMWORD PTR [rdx+0x20]
  65534b:	vmovups ymm2,YMMWORD PTR [rdx+0x40]
  655350:	vmovups ymm3,YMMWORD PTR [rdx+0x60]
  655355:	vmovups YMMWORD PTR [rsp+0x8],ymm0
  65535b:	vmovups YMMWORD PTR [rsp+0x28],ymm1
  655361:	vmovups YMMWORD PTR [rsp+0x48],ymm2
  655367:	vmovups YMMWORD PTR [rsp+0x68],ymm3
  65536d:	vmovups ymm0,YMMWORD PTR [rdx+0x80]
  655375:	vmovups YMMWORD PTR [rsp+0x88],ymm0
  65537e:	vmovups ymm0,YMMWORD PTR [rdx+0xa0]
  655386:	vmovups YMMWORD PTR [rsp+0xa8],ymm0
  65538f:	mov    r14,rsi
  655392:	mov    rbx,rdi
  655395:	mov    rax,QWORD PTR [rdx+0xc0]
  65539c:	mov    QWORD PTR [rsp+0xc8],rax
  6553a4:	mov    QWORD PTR [rsp+0xd0],0x0
  6553b0:	mov    r12,QWORD PTR [rsi]
  6553b3:	mov    r15,QWORD PTR [rsi+0x80]
  6553ba:	lea    rsi,[rip+0x1dd3f]        # 673100 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bal_E0TBal_Bal_EE00BaQ_ENtB5_3Job7executeB8C_>
  6553c1:	mov    rdx,rsp
  6553c4:	mov    rdi,r14
  6553c7:	vzeroupper 
  6553ca:	call   67de50 <_RNvMsg_NtCsh4KpTWVPfp2_15crossbeam_deque5dequeINtB5_8InjectorNtNtCsiKyBw8IgE1b_10rayon_core3job6JobRefE4pushB11_.llvm.10825995516073413387>
  6553cf:	xor    r15,r12
  6553d2:	movabs rdx,0x100000000
  6553dc:	lea    rdi,[r14+0x158]
  6553e3:	lock or DWORD PTR [rsp-0x40],0x0
  6553e9:	nop    DWORD PTR [rax+0x0]
  6553f0:	mov    rax,QWORD PTR [r14+0x170]
  6553f7:	test   rax,rdx
  6553fa:	jne    65541a <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B99_E0TB99_B99_EEB7q_+0xfa>
  6553fc:	mov    rcx,rax
  6553ff:	or     rcx,rdx
  655402:	lock cmpxchg QWORD PTR [r14+0x170],rcx
  65540b:	jne    6553f0 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B99_E0TB99_B99_EEB7q_+0xd0>
  65540d:	mov    rax,rcx
  655410:	and    rax,0xffff
  655416:	jne    655428 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B99_E0TB99_B99_EEB7q_+0x108>
  655418:	jmp    65543d <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B99_E0TB99_B99_EEB7q_+0x11d>
  65541a:	mov    rcx,rax
  65541d:	mov    rax,rcx
  655420:	and    rax,0xffff
  655426:	je     65543d <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B99_E0TB99_B99_EEB7q_+0x11d>
  655428:	cmp    r15,0x2
  65542c:	jae    6554c3 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B99_E0TB99_B99_EEB7q_+0x1a3>
  655432:	shr    ecx,0x10
  655435:	cmp    ecx,eax
  655437:	je     6554c3 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B99_E0TB99_B99_EEB7q_+0x1a3>
  65543d:	mov    rdi,QWORD PTR [rsp]
  655441:	call   QWORD PTR [rip+0xa31a9]        # 6f85f0 <_DYNAMIC+0x5d8>
  655447:	mov    rax,QWORD PTR [rsp+0xd0]
  65544f:	vmovdqu xmm0,XMMWORD PTR [rsp+0xd8]
  655458:	cmp    rax,0x1
  65545c:	je     655475 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B99_E0TB99_B99_EEB7q_+0x155>
  65545e:	cmp    rax,0x2
  655462:	jne    6554d3 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B99_E0TB99_B99_EEB7q_+0x1b3>
  655464:	vmovq  rdi,xmm0
  655469:	vpextrq rsi,xmm0,0x1
  65546f:	call   QWORD PTR [rip+0xa3183]        # 6f85f8 <_DYNAMIC+0x5e0>
  655475:	vmovups ymm1,YMMWORD PTR [rsp+0xe8]
  65547e:	vmovups ymm2,YMMWORD PTR [rsp+0x108]
  655487:	vmovups ymm3,YMMWORD PTR [rsp+0x128]
  655490:	vmovups ymm4,YMMWORD PTR [rsp+0x138]
  655499:	vmovups YMMWORD PTR [rbx+0x60],ymm4
  65549e:	vmovups YMMWORD PTR [rbx+0x50],ymm3
  6554a3:	vmovups YMMWORD PTR [rbx+0x30],ymm2
  6554a8:	vmovups YMMWORD PTR [rbx+0x10],ymm1
  6554ad:	vmovdqu XMMWORD PTR [rbx],xmm0
  6554b1:	add    rsp,0x158
  6554b8:	pop    rbx
  6554b9:	pop    r12
  6554bb:	pop    r14
  6554bd:	pop    r15
  6554bf:	vzeroupper 
  6554c2:	ret    
  6554c3:	mov    esi,0x1
  6554c8:	call   QWORD PTR [rip+0xa3132]        # 6f8600 <_DYNAMIC+0x5e8>
  6554ce:	jmp    65543d <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B99_E0TB99_B99_EEB7q_+0x11d>
  6554d3:	lea    rdi,[rip+0xffffffffff9bd182]        # 1265c <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x160e>
  6554da:	lea    rdx,[rip+0x9fbff]        # 6f50e0 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x360>
  6554e1:	mov    esi,0x28
  6554e6:	call   QWORD PTR [rip+0xa2e74]        # 6f8360 <_DYNAMIC+0x348>
  6554ec:	mov    rbx,rax
  6554ef:	mov    rdi,rsp
  6554f2:	call   656b80 <_RINvNtCsk9TLBDxQNFU_4core3ptr13drop_in_placeINtNtCsiKyBw8IgE1b_10rayon_core3job8StackJobINtNtBL_5latch8LatchRefNtB1r_9LockLatchENCNCINvMs4_NtBL_8registryNtB2d_8Registry14in_worker_coldNCINvNtBL_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3y_9enumerate17EnumerateProducerINtNtB3y_3zip11ZipProducerINtNtNtB3A_5slice6chunks17ChunksMutProducerINtNtNtB4_3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB5H_EEINtNtB3y_3map11MapConsumerINtNtB3y_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8R_s0_0ENCB8R_s_0EE0NCB3r_s_0ANtB75_13F256Unreducedj2_Baz_E0TBaz_Baz_EE00Bb4_EEB8V_>
  6554f7:	mov    rdi,rbx
  6554fa:	call   6f1930 <_Unwind_Resume@plt>
  6554ff:	call   QWORD PTR [rip+0xa2fc3]        # 6f84c8 <_DYNAMIC+0x4b0>
  655505:	int3   
  655506:	int3   
  655507:	int3   
  655508:	int3   
  655509:	int3   
  65550a:	int3   
  65550b:	int3   
  65550c:	int3   
  65550d:	int3   
  65550e:	int3   
  65550f:	int3   

