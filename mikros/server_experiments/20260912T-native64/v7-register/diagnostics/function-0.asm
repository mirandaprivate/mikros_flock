0000000000655130 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B94_E0TB94_B94_EEB7q_>:
  655130:	push   r15
  655132:	push   r14
  655134:	push   r12
  655136:	push   rbx
  655137:	sub    rsp,0x158
  65513e:	mov    rax,QWORD PTR fs:0x0
  655147:	lea    rax,[rax-0x50]
  65514e:	mov    QWORD PTR [rsp],rax
  655152:	vmovups ymm0,YMMWORD PTR [rdx]
  655156:	vmovups ymm1,YMMWORD PTR [rdx+0x20]
  65515b:	vmovups ymm2,YMMWORD PTR [rdx+0x40]
  655160:	vmovups ymm3,YMMWORD PTR [rdx+0x60]
  655165:	vmovups YMMWORD PTR [rsp+0x8],ymm0
  65516b:	vmovups YMMWORD PTR [rsp+0x28],ymm1
  655171:	vmovups YMMWORD PTR [rsp+0x48],ymm2
  655177:	vmovups YMMWORD PTR [rsp+0x68],ymm3
  65517d:	vmovups ymm0,YMMWORD PTR [rdx+0x80]
  655185:	vmovups YMMWORD PTR [rsp+0x88],ymm0
  65518e:	vmovups ymm0,YMMWORD PTR [rdx+0xa0]
  655196:	vmovups YMMWORD PTR [rsp+0xa8],ymm0
  65519f:	mov    r14,rsi
  6551a2:	mov    rbx,rdi
  6551a5:	mov    rax,QWORD PTR [rdx+0xc0]
  6551ac:	mov    QWORD PTR [rsp+0xc8],rax
  6551b4:	mov    QWORD PTR [rsp+0xd0],0x0
  6551c0:	mov    r12,QWORD PTR [rsi]
  6551c3:	mov    r15,QWORD PTR [rsi+0x80]
  6551ca:	lea    rsi,[rip+0x1dc5f]        # 672e30 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_9enumerate17EnumerateProducerINtNtB2Z_3zip11ZipProducerINtNtNtB31_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB58_EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8y_s0_0ENCB8y_s_0EE0NCB2S_s_0ANtB6M_13F256Unreducedj2_Bag_E0TBag_Bag_EE00BaL_ENtB5_3Job7executeB8C_>
  6551d1:	mov    rdx,rsp
  6551d4:	mov    rdi,r14
  6551d7:	vzeroupper 
  6551da:	call   67de50 <_RNvMsg_NtCsh4KpTWVPfp2_15crossbeam_deque5dequeINtB5_8InjectorNtNtCsiKyBw8IgE1b_10rayon_core3job6JobRefE4pushB11_.llvm.10825995516073413387>
  6551df:	xor    r15,r12
  6551e2:	movabs rdx,0x100000000
  6551ec:	lea    rdi,[r14+0x158]
  6551f3:	lock or DWORD PTR [rsp-0x40],0x0
  6551f9:	nop    DWORD PTR [rax+0x0]
  655200:	mov    rax,QWORD PTR [r14+0x170]
  655207:	test   rax,rdx
  65520a:	jne    65522a <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B94_E0TB94_B94_EEB7q_+0xfa>
  65520c:	mov    rcx,rax
  65520f:	or     rcx,rdx
  655212:	lock cmpxchg QWORD PTR [r14+0x170],rcx
  65521b:	jne    655200 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B94_E0TB94_B94_EEB7q_+0xd0>
  65521d:	mov    rax,rcx
  655220:	and    rax,0xffff
  655226:	jne    655238 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B94_E0TB94_B94_EEB7q_+0x108>
  655228:	jmp    65524d <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B94_E0TB94_B94_EEB7q_+0x11d>
  65522a:	mov    rcx,rax
  65522d:	mov    rax,rcx
  655230:	and    rax,0xffff
  655236:	je     65524d <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B94_E0TB94_B94_EEB7q_+0x11d>
  655238:	cmp    r15,0x2
  65523c:	jae    6552d3 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B94_E0TB94_B94_EEB7q_+0x1a3>
  655242:	shr    ecx,0x10
  655245:	cmp    ecx,eax
  655247:	je     6552d3 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B94_E0TB94_B94_EEB7q_+0x1a3>
  65524d:	mov    rdi,QWORD PTR [rsp]
  655251:	call   QWORD PTR [rip+0xa3399]        # 6f85f0 <_DYNAMIC+0x5d8>
  655257:	mov    rax,QWORD PTR [rsp+0xd0]
  65525f:	vmovdqu xmm0,XMMWORD PTR [rsp+0xd8]
  655268:	cmp    rax,0x1
  65526c:	je     655285 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B94_E0TB94_B94_EEB7q_+0x155>
  65526e:	cmp    rax,0x2
  655272:	jne    6552e3 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B94_E0TB94_B94_EEB7q_+0x1b3>
  655274:	vmovq  rdi,xmm0
  655279:	vpextrq rsi,xmm0,0x1
  65527f:	call   QWORD PTR [rip+0xa3373]        # 6f85f8 <_DYNAMIC+0x5e0>
  655285:	vmovups ymm1,YMMWORD PTR [rsp+0xe8]
  65528e:	vmovups ymm2,YMMWORD PTR [rsp+0x108]
  655297:	vmovups ymm3,YMMWORD PTR [rsp+0x128]
  6552a0:	vmovups ymm4,YMMWORD PTR [rsp+0x138]
  6552a9:	vmovups YMMWORD PTR [rbx+0x60],ymm4
  6552ae:	vmovups YMMWORD PTR [rbx+0x50],ymm3
  6552b3:	vmovups YMMWORD PTR [rbx+0x30],ymm2
  6552b8:	vmovups YMMWORD PTR [rbx+0x10],ymm1
  6552bd:	vmovdqu XMMWORD PTR [rbx],xmm0
  6552c1:	add    rsp,0x158
  6552c8:	pop    rbx
  6552c9:	pop    r12
  6552cb:	pop    r14
  6552cd:	pop    r15
  6552cf:	vzeroupper 
  6552d2:	ret    
  6552d3:	mov    esi,0x1
  6552d8:	call   QWORD PTR [rip+0xa3322]        # 6f8600 <_DYNAMIC+0x5e8>
  6552de:	jmp    65524d <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_9enumerate17EnumerateProducerINtNtB1N_3zip11ZipProducerINtNtNtB1P_5slice6chunks17ChunksMutProducerINtNtNtCsk9TLBDxQNFU_4core3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB3W_EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB7m_s0_0ENCB7m_s_0EE0NCB1G_s_0ANtB5A_13F256Unreducedj2_B94_E0TB94_B94_EEB7q_+0x11d>
  6552e3:	lea    rdi,[rip+0xffffffffff9bd372]        # 1265c <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x160e>
  6552ea:	lea    rdx,[rip+0x9fdef]        # 6f50e0 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x360>
  6552f1:	mov    esi,0x28
  6552f6:	call   QWORD PTR [rip+0xa3064]        # 6f8360 <_DYNAMIC+0x348>
  6552fc:	mov    rbx,rax
  6552ff:	mov    rdi,rsp
  655302:	call   656b80 <_RINvNtCsk9TLBDxQNFU_4core3ptr13drop_in_placeINtNtCsiKyBw8IgE1b_10rayon_core3job8StackJobINtNtBL_5latch8LatchRefNtB1r_9LockLatchENCNCINvMs4_NtBL_8registryNtB2d_8Registry14in_worker_coldNCINvNtBL_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3y_9enumerate17EnumerateProducerINtNtB3y_3zip11ZipProducerINtNtNtB3A_5slice6chunks17ChunksMutProducerINtNtNtB4_3mem12maybe_uninit11MaybeUninitNtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEB5H_EEINtNtB3y_3map11MapConsumerINtNtB3y_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_intos1_0NCB8R_s0_0ENCB8R_s_0EE0NCB3r_s_0ANtB75_13F256Unreducedj2_Baz_E0TBaz_Baz_EE00Bb4_EEB8V_>
  655307:	mov    rdi,rbx
  65530a:	call   6f1930 <_Unwind_Resume@plt>
  65530f:	call   QWORD PTR [rip+0xa31b3]        # 6f84c8 <_DYNAMIC+0x4b0>
  655315:	int3   
  655316:	int3   
  655317:	int3   
  655318:	int3   
  655319:	int3   
  65531a:	int3   
  65531b:	int3   
  65531c:	int3   
  65531d:	int3   
  65531e:	int3   
  65531f:	int3   

