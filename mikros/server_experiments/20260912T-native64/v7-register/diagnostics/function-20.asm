0000000000673100 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bal_E0TBal_Bal_EE00BaQ_ENtB5_3Job7executeB8C_>:
  673100:	push   rbp
  673101:	push   r15
  673103:	push   r14
  673105:	push   r13
  673107:	push   r12
  673109:	push   rbx
  67310a:	sub    rsp,0x1c8
  673111:	vmovups xmm0,XMMWORD PTR [rdi+0x8]
  673116:	cmp    QWORD PTR [rdi+0x8],0x0
  67311b:	mov    QWORD PTR [rdi+0x8],0x0
  673123:	je     673317 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bal_E0TBal_Bal_EE00BaQ_ENtB5_3Job7executeB8C_+0x217>
  673129:	mov    rbx,rdi
  67312c:	mov    rax,0xffffffffffffffa8
  673133:	mov    rdx,QWORD PTR fs:[rax]
  673137:	test   rdx,rdx
  67313a:	je     6732fc <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bal_E0TBal_Bal_EE00BaQ_ENtB5_3Job7executeB8C_+0x1fc>
  673140:	lea    rax,[rbx+0x18]
  673144:	lea    rcx,[rbx+0x88]
  67314b:	vmovaps XMMWORD PTR [rsp+0x80],xmm0
  673154:	vmovups ymm0,YMMWORD PTR [rax]
  673158:	vmovups ymm1,YMMWORD PTR [rax+0x20]
  67315d:	vmovups ymm2,YMMWORD PTR [rax+0x40]
  673162:	vmovups ymm3,YMMWORD PTR [rax+0x50]
  673167:	vmovups YMMWORD PTR [rsp+0x90],ymm0
  673170:	vmovups YMMWORD PTR [rsp+0xb0],ymm1
  673179:	vmovups YMMWORD PTR [rsp+0xd0],ymm2
  673182:	vmovups YMMWORD PTR [rsp+0xe0],ymm3
  67318b:	vmovups ymm0,YMMWORD PTR [rcx]
  67318f:	vmovups ymm1,YMMWORD PTR [rcx+0x20]
  673194:	vmovups YMMWORD PTR [rsp+0x100],ymm0
  67319d:	vmovups YMMWORD PTR [rsp+0x120],ymm1
  6731a6:	mov    rax,QWORD PTR [rcx+0x40]
  6731aa:	mov    QWORD PTR [rsp+0x140],rax
  6731b2:	lea    rdi,[rsp+0x148]
  6731ba:	lea    rsi,[rsp+0x80]
  6731c2:	mov    ecx,0x1
  6731c7:	vzeroupper 
  6731ca:	call   6638c0 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8f_E0B6x_>
  6731cf:	mov    r14,QWORD PTR [rsp+0x148]
  6731d7:	vmovups ymm0,YMMWORD PTR [rsp+0x1a8]
  6731e0:	vmovups YMMWORD PTR [rsp+0xd0],ymm0
  6731e9:	vmovups ymm0,YMMWORD PTR [rsp+0x198]
  6731f2:	vmovups YMMWORD PTR [rsp+0xc0],ymm0
  6731fb:	vmovups ymm0,YMMWORD PTR [rsp+0x158]
  673204:	vmovups YMMWORD PTR [rsp+0x80],ymm0
  67320d:	vmovups ymm1,YMMWORD PTR [rsp+0x178]
  673216:	vmovups YMMWORD PTR [rsp+0xa0],ymm1
  67321f:	vmovups YMMWORD PTR [rsp+0x10],ymm0
  673225:	vmovups YMMWORD PTR [rsp+0x30],ymm1
  67322b:	vmovups ymm0,YMMWORD PTR [rsp+0xc0]
  673234:	vmovups ymm1,YMMWORD PTR [rsp+0xd0]
  67323d:	mov    r15,QWORD PTR [rsp+0x150]
  673245:	vmovups YMMWORD PTR [rsp+0x50],ymm0
  67324b:	vmovups YMMWORD PTR [rsp+0x60],ymm1
  673251:	mov    r13d,0x1
  673257:	cmp    DWORD PTR [rbx+0xd0],0x2
  67325e:	jb     673292 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bal_E0TBal_Bal_EE00BaQ_ENtB5_3Job7executeB8C_+0x192>
  673260:	mov    r12,QWORD PTR [rbx+0xd8]
  673267:	mov    rbp,QWORD PTR [rbx+0xe0]
  67326e:	mov    rax,QWORD PTR [rbp+0x0]
  673272:	test   rax,rax
  673275:	je     67327f <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bal_E0TBal_Bal_EE00BaQ_ENtB5_3Job7executeB8C_+0x17f>
  673277:	mov    rdi,r12
  67327a:	vzeroupper 
  67327d:	call   rax
  67327f:	cmp    QWORD PTR [rbp+0x8],0x0
  673284:	je     673292 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bal_E0TBal_Bal_EE00BaQ_ENtB5_3Job7executeB8C_+0x192>
  673286:	mov    rdi,r12
  673289:	vzeroupper 
  67328c:	call   QWORD PTR [rip+0x84f8e]        # 6f8220 <free@GLIBC_2.2.5>
  673292:	mov    QWORD PTR [rbx+0xd0],r13
  673299:	mov    QWORD PTR [rbx+0xd8],r14
  6732a0:	mov    QWORD PTR [rbx+0xe0],r15
  6732a7:	vmovups ymm0,YMMWORD PTR [rsp+0x10]
  6732ad:	vmovups ymm1,YMMWORD PTR [rsp+0x30]
  6732b3:	vmovups ymm2,YMMWORD PTR [rsp+0x50]
  6732b9:	vmovups ymm3,YMMWORD PTR [rsp+0x60]
  6732bf:	vmovups YMMWORD PTR [rbx+0xe8],ymm0
  6732c7:	vmovups YMMWORD PTR [rbx+0x108],ymm1
  6732cf:	vmovups YMMWORD PTR [rbx+0x128],ymm2
  6732d7:	vmovups YMMWORD PTR [rbx+0x138],ymm3
  6732df:	mov    rdi,QWORD PTR [rbx]
  6732e2:	vzeroupper 
  6732e5:	call   676280 <_RNvXsd_NtCsiKyBw8IgE1b_10rayon_core5latchINtB5_8LatchRefNtB5_9LockLatchENtB5_5Latch3setCs5Palesvl9wF_6mikros>
  6732ea:	add    rsp,0x1c8
  6732f1:	pop    rbx
  6732f2:	pop    r12
  6732f4:	pop    r13
  6732f6:	pop    r14
  6732f8:	pop    r15
  6732fa:	pop    rbp
  6732fb:	ret    
  6732fc:	lea    rdi,[rip+0xffffffffff99f381]        # 12684 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1636>
  673303:	lea    rdx,[rip+0x81b1e]        # 6f4e28 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0xa8>
  67330a:	mov    esi,0x36
  67330f:	call   QWORD PTR [rip+0x8504b]        # 6f8360 <_DYNAMIC+0x348>
  673315:	jmp    673393 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bal_E0TBal_Bal_EE00BaQ_ENtB5_3Job7executeB8C_+0x293>
  673317:	lea    rdi,[rip+0x822ea]        # 6f5608 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x888>
  67331e:	call   QWORD PTR [rip+0x85094]        # 6f83b8 <_DYNAMIC+0x3a0>
  673324:	jmp    673393 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bal_E0TBal_Bal_EE00BaQ_ENtB5_3Job7executeB8C_+0x293>
  673326:	cmp    QWORD PTR [rbp+0x8],0x0
  67332b:	je     673336 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bal_E0TBal_Bal_EE00BaQ_ENtB5_3Job7executeB8C_+0x236>
  67332d:	mov    rdi,r12
  673330:	call   QWORD PTR [rip+0x84eea]        # 6f8220 <free@GLIBC_2.2.5>
  673336:	mov    QWORD PTR [rbx+0xd0],r13
  67333d:	mov    QWORD PTR [rbx+0xd8],r14
  673344:	mov    QWORD PTR [rbx+0xe0],r15
  67334b:	vmovups ymm0,YMMWORD PTR [rsp+0x10]
  673351:	vmovups ymm1,YMMWORD PTR [rsp+0x30]
  673357:	vmovups ymm2,YMMWORD PTR [rsp+0x50]
  67335d:	vmovups ymm3,YMMWORD PTR [rsp+0x60]
  673363:	vmovups YMMWORD PTR [rbx+0xe8],ymm0
  67336b:	vmovups YMMWORD PTR [rbx+0x108],ymm1
  673373:	vmovups YMMWORD PTR [rbx+0x128],ymm2
  67337b:	vmovups YMMWORD PTR [rbx+0x138],ymm3
  673383:	jmp    673385 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bal_E0TBal_Bal_EE00BaQ_ENtB5_3Job7executeB8C_+0x285>
  673385:	lea    rdi,[rsp+0xf]
  67338a:	vzeroupper 
  67338d:	call   QWORD PTR [rip+0x853fd]        # 6f8790 <_DYNAMIC+0x778>
  673393:	ud2    
  673395:	call   QWORD PTR [rip+0x8512d]        # 6f84c8 <_DYNAMIC+0x4b0>
  67339b:	mov    rdi,rax
  67339e:	call   QWORD PTR [rip+0x852b4]        # 6f8658 <_DYNAMIC+0x640>
  6733a4:	mov    r14,rax
  6733a7:	mov    r15,rdx
  6733aa:	mov    r13d,0x2
  6733b0:	cmp    DWORD PTR [rbx+0xd0],0x2
  6733b7:	jae    673260 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bal_E0TBal_Bal_EE00BaQ_ENtB5_3Job7executeB8C_+0x160>
  6733bd:	jmp    673292 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup41pack_pair_weighted_with_wide_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bal_E0TBal_Bal_EE00BaQ_ENtB5_3Job7executeB8C_+0x192>
  6733c2:	int3   
  6733c3:	int3   
  6733c4:	int3   
  6733c5:	int3   
  6733c6:	int3   
  6733c7:	int3   
  6733c8:	int3   
  6733c9:	int3   
  6733ca:	int3   
  6733cb:	int3   
  6733cc:	int3   
  6733cd:	int3   
  6733ce:	int3   
  6733cf:	int3   

