0000000000672e30 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bag_E0TBag_Bag_EE00BaL_ENtB5_3Job7executeB8C_>:
  672e30:	push   rbp
  672e31:	push   r15
  672e33:	push   r14
  672e35:	push   r13
  672e37:	push   r12
  672e39:	push   rbx
  672e3a:	sub    rsp,0x1c8
  672e41:	vmovups xmm0,XMMWORD PTR [rdi+0x8]
  672e46:	cmp    QWORD PTR [rdi+0x8],0x0
  672e4b:	mov    QWORD PTR [rdi+0x8],0x0
  672e53:	je     673047 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bag_E0TBag_Bag_EE00BaL_ENtB5_3Job7executeB8C_+0x217>
  672e59:	mov    rbx,rdi
  672e5c:	mov    rax,0xffffffffffffffa8
  672e63:	mov    rdx,QWORD PTR fs:[rax]
  672e67:	test   rdx,rdx
  672e6a:	je     67302c <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bag_E0TBag_Bag_EE00BaL_ENtB5_3Job7executeB8C_+0x1fc>
  672e70:	lea    rax,[rbx+0x18]
  672e74:	lea    rcx,[rbx+0x88]
  672e7b:	vmovaps XMMWORD PTR [rsp+0x80],xmm0
  672e84:	vmovups ymm0,YMMWORD PTR [rax]
  672e88:	vmovups ymm1,YMMWORD PTR [rax+0x20]
  672e8d:	vmovups ymm2,YMMWORD PTR [rax+0x40]
  672e92:	vmovups ymm3,YMMWORD PTR [rax+0x50]
  672e97:	vmovups YMMWORD PTR [rsp+0x90],ymm0
  672ea0:	vmovups YMMWORD PTR [rsp+0xb0],ymm1
  672ea9:	vmovups YMMWORD PTR [rsp+0xd0],ymm2
  672eb2:	vmovups YMMWORD PTR [rsp+0xe0],ymm3
  672ebb:	vmovups ymm0,YMMWORD PTR [rcx]
  672ebf:	vmovups ymm1,YMMWORD PTR [rcx+0x20]
  672ec4:	vmovups YMMWORD PTR [rsp+0x100],ymm0
  672ecd:	vmovups YMMWORD PTR [rsp+0x120],ymm1
  672ed6:	mov    rax,QWORD PTR [rcx+0x40]
  672eda:	mov    QWORD PTR [rsp+0x140],rax
  672ee2:	lea    rdi,[rsp+0x148]
  672eea:	lea    rsi,[rsp+0x80]
  672ef2:	mov    ecx,0x1
  672ef7:	vzeroupper 
  672efa:	call   663340 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_9enumerate17EnumerateProducerINtNtBY_3zip11ZipProducerINtNtNtB10_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB35_EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB6t_s0_0ENCB6t_s_0EE0NCBR_s_0ANtB4J_13F256Unreducedj2_B8a_E0B6x_>
  672eff:	mov    r14,QWORD PTR [rsp+0x148]
  672f07:	vmovups ymm0,YMMWORD PTR [rsp+0x1a8]
  672f10:	vmovups YMMWORD PTR [rsp+0xd0],ymm0
  672f19:	vmovups ymm0,YMMWORD PTR [rsp+0x198]
  672f22:	vmovups YMMWORD PTR [rsp+0xc0],ymm0
  672f2b:	vmovups ymm0,YMMWORD PTR [rsp+0x158]
  672f34:	vmovups YMMWORD PTR [rsp+0x80],ymm0
  672f3d:	vmovups ymm1,YMMWORD PTR [rsp+0x178]
  672f46:	vmovups YMMWORD PTR [rsp+0xa0],ymm1
  672f4f:	vmovups YMMWORD PTR [rsp+0x10],ymm0
  672f55:	vmovups YMMWORD PTR [rsp+0x30],ymm1
  672f5b:	vmovups ymm0,YMMWORD PTR [rsp+0xc0]
  672f64:	vmovups ymm1,YMMWORD PTR [rsp+0xd0]
  672f6d:	mov    r15,QWORD PTR [rsp+0x150]
  672f75:	vmovups YMMWORD PTR [rsp+0x50],ymm0
  672f7b:	vmovups YMMWORD PTR [rsp+0x60],ymm1
  672f81:	mov    r13d,0x1
  672f87:	cmp    DWORD PTR [rbx+0xd0],0x2
  672f8e:	jb     672fc2 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bag_E0TBag_Bag_EE00BaL_ENtB5_3Job7executeB8C_+0x192>
  672f90:	mov    r12,QWORD PTR [rbx+0xd8]
  672f97:	mov    rbp,QWORD PTR [rbx+0xe0]
  672f9e:	mov    rax,QWORD PTR [rbp+0x0]
  672fa2:	test   rax,rax
  672fa5:	je     672faf <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bag_E0TBag_Bag_EE00BaL_ENtB5_3Job7executeB8C_+0x17f>
  672fa7:	mov    rdi,r12
  672faa:	vzeroupper 
  672fad:	call   rax
  672faf:	cmp    QWORD PTR [rbp+0x8],0x0
  672fb4:	je     672fc2 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bag_E0TBag_Bag_EE00BaL_ENtB5_3Job7executeB8C_+0x192>
  672fb6:	mov    rdi,r12
  672fb9:	vzeroupper 
  672fbc:	call   QWORD PTR [rip+0x8525e]        # 6f8220 <free@GLIBC_2.2.5>
  672fc2:	mov    QWORD PTR [rbx+0xd0],r13
  672fc9:	mov    QWORD PTR [rbx+0xd8],r14
  672fd0:	mov    QWORD PTR [rbx+0xe0],r15
  672fd7:	vmovups ymm0,YMMWORD PTR [rsp+0x10]
  672fdd:	vmovups ymm1,YMMWORD PTR [rsp+0x30]
  672fe3:	vmovups ymm2,YMMWORD PTR [rsp+0x50]
  672fe9:	vmovups ymm3,YMMWORD PTR [rsp+0x60]
  672fef:	vmovups YMMWORD PTR [rbx+0xe8],ymm0
  672ff7:	vmovups YMMWORD PTR [rbx+0x108],ymm1
  672fff:	vmovups YMMWORD PTR [rbx+0x128],ymm2
  673007:	vmovups YMMWORD PTR [rbx+0x138],ymm3
  67300f:	mov    rdi,QWORD PTR [rbx]
  673012:	vzeroupper 
  673015:	call   676280 <_RNvXsd_NtCsiKyBw8IgE1b_10rayon_core5latchINtB5_8LatchRefNtB5_9LockLatchENtB5_5Latch3setCs5Palesvl9wF_6mikros>
  67301a:	add    rsp,0x1c8
  673021:	pop    rbx
  673022:	pop    r12
  673024:	pop    r13
  673026:	pop    r14
  673028:	pop    r15
  67302a:	pop    rbp
  67302b:	ret    
  67302c:	lea    rdi,[rip+0xffffffffff99f651]        # 12684 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1636>
  673033:	lea    rdx,[rip+0x81dee]        # 6f4e28 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0xa8>
  67303a:	mov    esi,0x36
  67303f:	call   QWORD PTR [rip+0x8531b]        # 6f8360 <_DYNAMIC+0x348>
  673045:	jmp    6730c3 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bag_E0TBag_Bag_EE00BaL_ENtB5_3Job7executeB8C_+0x293>
  673047:	lea    rdi,[rip+0x825ba]        # 6f5608 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x888>
  67304e:	call   QWORD PTR [rip+0x85364]        # 6f83b8 <_DYNAMIC+0x3a0>
  673054:	jmp    6730c3 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bag_E0TBag_Bag_EE00BaL_ENtB5_3Job7executeB8C_+0x293>
  673056:	cmp    QWORD PTR [rbp+0x8],0x0
  67305b:	je     673066 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bag_E0TBag_Bag_EE00BaL_ENtB5_3Job7executeB8C_+0x236>
  67305d:	mov    rdi,r12
  673060:	call   QWORD PTR [rip+0x851ba]        # 6f8220 <free@GLIBC_2.2.5>
  673066:	mov    QWORD PTR [rbx+0xd0],r13
  67306d:	mov    QWORD PTR [rbx+0xd8],r14
  673074:	mov    QWORD PTR [rbx+0xe0],r15
  67307b:	vmovups ymm0,YMMWORD PTR [rsp+0x10]
  673081:	vmovups ymm1,YMMWORD PTR [rsp+0x30]
  673087:	vmovups ymm2,YMMWORD PTR [rsp+0x50]
  67308d:	vmovups ymm3,YMMWORD PTR [rsp+0x60]
  673093:	vmovups YMMWORD PTR [rbx+0xe8],ymm0
  67309b:	vmovups YMMWORD PTR [rbx+0x108],ymm1
  6730a3:	vmovups YMMWORD PTR [rbx+0x128],ymm2
  6730ab:	vmovups YMMWORD PTR [rbx+0x138],ymm3
  6730b3:	jmp    6730b5 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bag_E0TBag_Bag_EE00BaL_ENtB5_3Job7executeB8C_+0x285>
  6730b5:	lea    rdi,[rsp+0xf]
  6730ba:	vzeroupper 
  6730bd:	call   QWORD PTR [rip+0x856cd]        # 6f8790 <_DYNAMIC+0x778>
  6730c3:	ud2    
  6730c5:	call   QWORD PTR [rip+0x853fd]        # 6f84c8 <_DYNAMIC+0x4b0>
  6730cb:	mov    rdi,rax
  6730ce:	call   QWORD PTR [rip+0x85584]        # 6f8658 <_DYNAMIC+0x640>
  6730d4:	mov    r14,rax
  6730d7:	mov    r15,rdx
  6730da:	mov    r13d,0x2
  6730e0:	cmp    DWORD PTR [rbx+0xd0],0x2
  6730e7:	jae    672f90 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bag_E0TBag_Bag_EE00BaL_ENtB5_3Job7executeB8C_+0x160>
  6730ed:	jmp    672fc2 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bag_E0TBag_Bag_EE00BaL_ENtB5_3Job7executeB8C_+0x192>
  6730f2:	int3   
  6730f3:	int3   
  6730f4:	int3   
  6730f5:	int3   
  6730f6:	int3   
  6730f7:	int3   
  6730f8:	int3   
  6730f9:	int3   
  6730fa:	int3   
  6730fb:	int3   
  6730fc:	int3   
  6730fd:	int3   
  6730fe:	int3   
  6730ff:	int3   

