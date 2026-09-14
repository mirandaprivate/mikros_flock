0000000000674710 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_Ba4_E0TBa4_Ba4_EE0Baz_ENtB5_3Job7executeB8l_>:
  674710:	push   rbp
  674711:	push   r15
  674713:	push   r14
  674715:	push   r13
  674717:	push   r12
  674719:	push   rbx
  67471a:	sub    rsp,0x1c8
  674721:	vmovups xmm0,XMMWORD PTR [rdi]
  674725:	cmp    QWORD PTR [rdi],0x0
  674729:	mov    QWORD PTR [rdi],0x0
  674730:	je     674996 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_Ba4_E0TBa4_Ba4_EE0Baz_ENtB5_3Job7executeB8l_+0x286>
  674736:	mov    rbx,rdi
  674739:	mov    rax,0xffffffffffffffa8
  674740:	mov    rdx,QWORD PTR fs:[rax]
  674744:	test   rdx,rdx
  674747:	je     674978 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_Ba4_E0TBa4_Ba4_EE0Baz_ENtB5_3Job7executeB8l_+0x268>
  67474d:	lea    rax,[rbx+0x10]
  674751:	lea    rcx,[rbx+0x80]
  674758:	vmovaps XMMWORD PTR [rsp+0x80],xmm0
  674761:	vmovups ymm0,YMMWORD PTR [rax]
  674765:	vmovups ymm1,YMMWORD PTR [rax+0x20]
  67476a:	vmovups ymm2,YMMWORD PTR [rax+0x40]
  67476f:	vmovups ymm3,YMMWORD PTR [rax+0x50]
  674774:	vmovups YMMWORD PTR [rsp+0x90],ymm0
  67477d:	vmovups YMMWORD PTR [rsp+0xb0],ymm1
  674786:	vmovups YMMWORD PTR [rsp+0xd0],ymm2
  67478f:	vmovups YMMWORD PTR [rsp+0xe0],ymm3
  674798:	vmovups ymm0,YMMWORD PTR [rcx]
  67479c:	vmovups ymm1,YMMWORD PTR [rcx+0x20]
  6747a1:	vmovups YMMWORD PTR [rsp+0x100],ymm0
  6747aa:	vmovups YMMWORD PTR [rsp+0x120],ymm1
  6747b3:	mov    rax,QWORD PTR [rcx+0x40]
  6747b7:	mov    QWORD PTR [rsp+0x140],rax
  6747bf:	lea    rdi,[rsp+0x148]
  6747c7:	lea    rsi,[rsp+0x80]
  6747cf:	mov    ecx,0x1
  6747d4:	vzeroupper 
  6747d7:	call   6638c0 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_>
  6747dc:	mov    r14,QWORD PTR [rsp+0x148]
  6747e4:	vmovups ymm0,YMMWORD PTR [rsp+0x1a8]
  6747ed:	vmovups YMMWORD PTR [rsp+0xd0],ymm0
  6747f6:	vmovups ymm0,YMMWORD PTR [rsp+0x198]
  6747ff:	vmovups YMMWORD PTR [rsp+0xc0],ymm0
  674808:	vmovups ymm0,YMMWORD PTR [rsp+0x158]
  674811:	vmovups YMMWORD PTR [rsp+0x80],ymm0
  67481a:	vmovups ymm1,YMMWORD PTR [rsp+0x178]
  674823:	vmovups YMMWORD PTR [rsp+0xa0],ymm1
  67482c:	vmovups YMMWORD PTR [rsp+0x10],ymm0
  674832:	vmovups YMMWORD PTR [rsp+0x30],ymm1
  674838:	vmovups ymm0,YMMWORD PTR [rsp+0xc0]
  674841:	vmovups ymm1,YMMWORD PTR [rsp+0xd0]
  67484a:	mov    r15,QWORD PTR [rsp+0x150]
  674852:	vmovups YMMWORD PTR [rsp+0x50],ymm0
  674858:	vmovups YMMWORD PTR [rsp+0x60],ymm1
  67485e:	mov    r13d,0x1
  674864:	cmp    DWORD PTR [rbx+0xc8],0x2
  67486b:	jb     67489f <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_Ba4_E0TBa4_Ba4_EE0Baz_ENtB5_3Job7executeB8l_+0x18f>
  67486d:	mov    r12,QWORD PTR [rbx+0xd0]
  674874:	mov    rbp,QWORD PTR [rbx+0xd8]
  67487b:	mov    rax,QWORD PTR [rbp+0x0]
  67487f:	test   rax,rax
  674882:	je     67488c <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_Ba4_E0TBa4_Ba4_EE0Baz_ENtB5_3Job7executeB8l_+0x17c>
  674884:	mov    rdi,r12
  674887:	vzeroupper 
  67488a:	call   rax
  67488c:	cmp    QWORD PTR [rbp+0x8],0x0
  674891:	je     67489f <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_Ba4_E0TBa4_Ba4_EE0Baz_ENtB5_3Job7executeB8l_+0x18f>
  674893:	mov    rdi,r12
  674896:	vzeroupper 
  674899:	call   QWORD PTR [rip+0x83981]        # 6f8220 <free@GLIBC_2.2.5>
  67489f:	mov    QWORD PTR [rbx+0xc8],r13
  6748a6:	mov    QWORD PTR [rbx+0xd0],r14
  6748ad:	mov    QWORD PTR [rbx+0xd8],r15
  6748b4:	vmovups ymm0,YMMWORD PTR [rsp+0x10]
  6748ba:	vmovups ymm1,YMMWORD PTR [rsp+0x30]
  6748c0:	vmovups ymm2,YMMWORD PTR [rsp+0x50]
  6748c6:	vmovups ymm3,YMMWORD PTR [rsp+0x60]
  6748cc:	vmovups YMMWORD PTR [rbx+0xe0],ymm0
  6748d4:	vmovups YMMWORD PTR [rbx+0x100],ymm1
  6748dc:	vmovups YMMWORD PTR [rbx+0x120],ymm2
  6748e4:	vmovups YMMWORD PTR [rbx+0x130],ymm3
  6748ec:	movzx  ebp,BYTE PTR [rbx+0x168]
  6748f3:	test   bpl,bpl
  6748f6:	je     674913 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_Ba4_E0TBa4_Ba4_EE0Baz_ENtB5_3Job7executeB8l_+0x203>
  6748f8:	mov    rax,QWORD PTR [rbx+0x150]
  6748ff:	mov    rdi,QWORD PTR [rax]
  674902:	lock inc QWORD PTR [rdi]
  674906:	jle    674a38 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_Ba4_E0TBa4_Ba4_EE0Baz_ENtB5_3Job7executeB8l_+0x328>
  67490c:	mov    QWORD PTR [rsp+0x10],rdi
  674911:	jmp    67491d <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_Ba4_E0TBa4_Ba4_EE0Baz_ENtB5_3Job7executeB8l_+0x20d>
  674913:	mov    rax,QWORD PTR [rbx+0x150]
  67491a:	mov    rdi,QWORD PTR [rax]
  67491d:	mov    rsi,QWORD PTR [rbx+0x160]
  674924:	mov    eax,0x3
  674929:	xchg   QWORD PTR [rbx+0x158],rax
  674930:	cmp    rax,0x2
  674934:	jne    674945 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_Ba4_E0TBa4_Ba4_EE0Baz_ENtB5_3Job7executeB8l_+0x235>
  674936:	add    rdi,0x1d8
  67493d:	vzeroupper 
  674940:	call   67c660 <_RNvMNtCsiKyBw8IgE1b_10rayon_core5sleepNtB2_5Sleep20wake_specific_thread.llvm.10825995516073413387>
  674945:	test   bpl,bpl
  674948:	je     674963 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_Ba4_E0TBa4_Ba4_EE0Baz_ENtB5_3Job7executeB8l_+0x253>
  67494a:	mov    rax,QWORD PTR [rsp+0x10]
  67494f:	lock dec QWORD PTR [rax]
  674953:	jne    674963 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_Ba4_E0TBa4_Ba4_EE0Baz_ENtB5_3Job7executeB8l_+0x253>
  674955:	lea    rdi,[rsp+0x10]
  67495a:	vzeroupper 
  67495d:	call   QWORD PTR [rip+0x83e35]        # 6f8798 <_DYNAMIC+0x780>
  674963:	add    rsp,0x1c8
  67496a:	pop    rbx
  67496b:	pop    r12
  67496d:	pop    r13
  67496f:	pop    r14
  674971:	pop    r15
  674973:	pop    rbp
  674974:	vzeroupper 
  674977:	ret    
  674978:	lea    rdi,[rip+0xffffffffff99dd05]        # 12684 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1636>
  67497f:	lea    rdx,[rip+0x8048a]        # 6f4e10 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x90>
  674986:	mov    esi,0x36
  67498b:	call   QWORD PTR [rip+0x839cf]        # 6f8360 <_DYNAMIC+0x348>
  674991:	jmp    674a38 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_Ba4_E0TBa4_Ba4_EE0Baz_ENtB5_3Job7executeB8l_+0x328>
  674996:	lea    rdi,[rip+0x80c6b]        # 6f5608 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x888>
  67499d:	call   QWORD PTR [rip+0x83a15]        # 6f83b8 <_DYNAMIC+0x3a0>
  6749a3:	jmp    674a38 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_Ba4_E0TBa4_Ba4_EE0Baz_ENtB5_3Job7executeB8l_+0x328>
  6749a8:	cmp    QWORD PTR [rbp+0x8],0x0
  6749ad:	je     6749b8 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_Ba4_E0TBa4_Ba4_EE0Baz_ENtB5_3Job7executeB8l_+0x2a8>
  6749af:	mov    rdi,r12
  6749b2:	call   QWORD PTR [rip+0x83868]        # 6f8220 <free@GLIBC_2.2.5>
  6749b8:	mov    QWORD PTR [rbx+0xc8],r13
  6749bf:	mov    QWORD PTR [rbx+0xd0],r14
  6749c6:	mov    QWORD PTR [rbx+0xd8],r15
  6749cd:	vmovups ymm0,YMMWORD PTR [rsp+0x10]
  6749d3:	vmovups ymm1,YMMWORD PTR [rsp+0x30]
  6749d9:	vmovups ymm2,YMMWORD PTR [rsp+0x50]
  6749df:	vmovups ymm3,YMMWORD PTR [rsp+0x60]
  6749e5:	vmovups YMMWORD PTR [rbx+0xe0],ymm0
  6749ed:	vmovups YMMWORD PTR [rbx+0x100],ymm1
  6749f5:	vmovups YMMWORD PTR [rbx+0x120],ymm2
  6749fd:	vmovups YMMWORD PTR [rbx+0x130],ymm3
  674a05:	jmp    674a2a <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_Ba4_E0TBa4_Ba4_EE0Baz_ENtB5_3Job7executeB8l_+0x31a>
  674a07:	test   bpl,bpl
  674a0a:	je     674a2a <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_Ba4_E0TBa4_Ba4_EE0Baz_ENtB5_3Job7executeB8l_+0x31a>
  674a0c:	mov    rax,QWORD PTR [rsp+0x10]
  674a11:	lock dec QWORD PTR [rax]
  674a15:	jne    674a2a <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_Ba4_E0TBa4_Ba4_EE0Baz_ENtB5_3Job7executeB8l_+0x31a>
  674a17:	lea    rdi,[rsp+0x10]
  674a1c:	call   QWORD PTR [rip+0x83d76]        # 6f8798 <_DYNAMIC+0x780>
  674a22:	jmp    674a2a <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_Ba4_E0TBa4_Ba4_EE0Baz_ENtB5_3Job7executeB8l_+0x31a>
  674a24:	call   QWORD PTR [rip+0x83a9e]        # 6f84c8 <_DYNAMIC+0x4b0>
  674a2a:	lea    rdi,[rsp+0xf]
  674a2f:	vzeroupper 
  674a32:	call   QWORD PTR [rip+0x83d58]        # 6f8790 <_DYNAMIC+0x778>
  674a38:	ud2    
  674a3a:	call   QWORD PTR [rip+0x83a88]        # 6f84c8 <_DYNAMIC+0x4b0>
  674a40:	mov    rdi,rax
  674a43:	call   QWORD PTR [rip+0x83c0f]        # 6f8658 <_DYNAMIC+0x640>
  674a49:	mov    r14,rax
  674a4c:	mov    r15,rdx
  674a4f:	mov    r13d,0x2
  674a55:	cmp    DWORD PTR [rbx+0xc8],0x2
  674a5c:	jae    67486d <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_Ba4_E0TBa4_Ba4_EE0Baz_ENtB5_3Job7executeB8l_+0x15d>
  674a62:	jmp    67489f <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_9enumerate17EnumerateProducerINtNtB2I_3zip11ZipProducerINtNtNtB2K_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB4R_EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8h_s0_0ENCB8h_s_0EE0NCB2B_s_0ANtB6v_13F256Unreducedj2_Ba4_E0TBa4_Ba4_EE0Baz_ENtB5_3Job7executeB8l_+0x18f>
  674a67:	int3   
  674a68:	int3   
  674a69:	int3   
  674a6a:	int3   
  674a6b:	int3   
  674a6c:	int3   
  674a6d:	int3   
  674a6e:	int3   
  674a6f:	int3   

