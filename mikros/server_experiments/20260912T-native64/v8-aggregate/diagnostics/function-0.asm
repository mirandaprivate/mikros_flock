000000000065e1d0 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_3zip11ZipProducerIB2V_INtNtNtB1P_5slice6chunks14ChunksProducerhEB3p_EIB3q_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5X_s_0ENCB5X_0EE0NCB1G_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7f_E0TB7f_B7f_EEB61_>:
  65e1d0:	41 57                	push   r15
  65e1d2:	41 56                	push   r14
  65e1d4:	41 55                	push   r13
  65e1d6:	41 54                	push   r12
  65e1d8:	53                   	push   rbx
  65e1d9:	48 81 ec 40 01 00 00 	sub    rsp,0x140
  65e1e0:	64 48 8b 04 25 00 00 	mov    rax,QWORD PTR fs:0x0
  65e1e7:	00 00 
  65e1e9:	48 8d 80 b0 ff ff ff 	lea    rax,[rax-0x50]
  65e1f0:	49 89 f6             	mov    r14,rsi
  65e1f3:	48 89 fb             	mov    rbx,rdi
  65e1f6:	48 89 44 24 50       	mov    QWORD PTR [rsp+0x50],rax
  65e1fb:	c5 fc 10 02          	vmovups ymm0,YMMWORD PTR [rdx]
  65e1ff:	c5 fc 10 4a 20       	vmovups ymm1,YMMWORD PTR [rdx+0x20]
  65e204:	c5 fc 10 52 40       	vmovups ymm2,YMMWORD PTR [rdx+0x40]
  65e209:	c5 fc 10 5a 60       	vmovups ymm3,YMMWORD PTR [rdx+0x60]
  65e20e:	c5 fc 11 44 24 58    	vmovups YMMWORD PTR [rsp+0x58],ymm0
  65e214:	c5 fc 11 4c 24 78    	vmovups YMMWORD PTR [rsp+0x78],ymm1
  65e21a:	c5 fc 11 94 24 98 00 	vmovups YMMWORD PTR [rsp+0x98],ymm2
  65e221:	00 00 
  65e223:	c5 fc 11 9c 24 b8 00 	vmovups YMMWORD PTR [rsp+0xb8],ymm3
  65e22a:	00 00 
  65e22c:	c5 fc 10 82 80 00 00 	vmovups ymm0,YMMWORD PTR [rdx+0x80]
  65e233:	00 
  65e234:	c5 fc 11 84 24 d8 00 	vmovups YMMWORD PTR [rsp+0xd8],ymm0
  65e23b:	00 00 
  65e23d:	c5 fc 10 82 a0 00 00 	vmovups ymm0,YMMWORD PTR [rdx+0xa0]
  65e244:	00 
  65e245:	c5 fc 11 84 24 f8 00 	vmovups YMMWORD PTR [rsp+0xf8],ymm0
  65e24c:	00 00 
  65e24e:	c5 fc 10 82 c0 00 00 	vmovups ymm0,YMMWORD PTR [rdx+0xc0]
  65e255:	00 
  65e256:	c5 fc 11 84 24 18 01 	vmovups YMMWORD PTR [rsp+0x118],ymm0
  65e25d:	00 00 
  65e25f:	49 bf 00 00 00 00 00 	movabs r15,0x8000000000000000
  65e266:	00 00 80 
  65e269:	48 8b 82 e0 00 00 00 	mov    rax,QWORD PTR [rdx+0xe0]
  65e270:	48 89 84 24 38 01 00 	mov    QWORD PTR [rsp+0x138],rax
  65e277:	00 
  65e278:	4c 89 7c 24 20       	mov    QWORD PTR [rsp+0x20],r15
  65e27d:	4c 8b 2e             	mov    r13,QWORD PTR [rsi]
  65e280:	4c 8b a6 80 00 00 00 	mov    r12,QWORD PTR [rsi+0x80]
  65e287:	48 8d 35 02 2f 02 00 	lea    rsi,[rip+0x22f02]        # 681190 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_3zip11ZipProducerIB47_INtNtNtB31_5slice6chunks14ChunksProducerhEB4B_EIB4C_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB79_s_0ENCB79_0EE0NCB2S_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8r_E0TB8r_B8r_EE00B9b_ENtB5_3Job7executeB7d_>
  65e28e:	48 8d 54 24 20       	lea    rdx,[rsp+0x20]
  65e293:	4c 89 f7             	mov    rdi,r14
  65e296:	c5 f8 77             	vzeroupper 
  65e299:	e8 82 e9 02 00       	call   68cc20 <_RNvMsg_NtCsh4KpTWVPfp2_15crossbeam_deque5dequeINtB5_8InjectorNtNtCsiKyBw8IgE1b_10rayon_core3job6JobRefE4pushB11_.llvm.10825995516073413387>
  65e29e:	4d 31 ec             	xor    r12,r13
  65e2a1:	48 ba 00 00 00 00 01 	movabs rdx,0x100000000
  65e2a8:	00 00 00 
  65e2ab:	49 8d be 58 01 00 00 	lea    rdi,[r14+0x158]
  65e2b2:	f0 83 4c 24 c0 00    	lock or DWORD PTR [rsp-0x40],0x0
  65e2b8:	0f 1f 84 00 00 00 00 	nop    DWORD PTR [rax+rax*1+0x0]
  65e2bf:	00 
  65e2c0:	49 8b 86 70 01 00 00 	mov    rax,QWORD PTR [r14+0x170]
  65e2c7:	48 85 d0             	test   rax,rdx
  65e2ca:	75 1e                	jne    65e2ea <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_3zip11ZipProducerIB2V_INtNtNtB1P_5slice6chunks14ChunksProducerhEB3p_EIB3q_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5X_s_0ENCB5X_0EE0NCB1G_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7f_E0TB7f_B7f_EEB61_+0x11a>
  65e2cc:	48 89 c1             	mov    rcx,rax
  65e2cf:	48 09 d1             	or     rcx,rdx
  65e2d2:	f0 49 0f b1 8e 70 01 	lock cmpxchg QWORD PTR [r14+0x170],rcx
  65e2d9:	00 00 
  65e2db:	75 e3                	jne    65e2c0 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_3zip11ZipProducerIB2V_INtNtNtB1P_5slice6chunks14ChunksProducerhEB3p_EIB3q_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5X_s_0ENCB5X_0EE0NCB1G_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7f_E0TB7f_B7f_EEB61_+0xf0>
  65e2dd:	48 89 c8             	mov    rax,rcx
  65e2e0:	48 25 ff ff 00 00    	and    rax,0xffff
  65e2e6:	75 10                	jne    65e2f8 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_3zip11ZipProducerIB2V_INtNtNtB1P_5slice6chunks14ChunksProducerhEB3p_EIB3q_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5X_s_0ENCB5X_0EE0NCB1G_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7f_E0TB7f_B7f_EEB61_+0x128>
  65e2e8:	eb 23                	jmp    65e30d <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_3zip11ZipProducerIB2V_INtNtNtB1P_5slice6chunks14ChunksProducerhEB3p_EIB3q_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5X_s_0ENCB5X_0EE0NCB1G_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7f_E0TB7f_B7f_EEB61_+0x13d>
  65e2ea:	48 89 c1             	mov    rcx,rax
  65e2ed:	48 89 c8             	mov    rax,rcx
  65e2f0:	48 25 ff ff 00 00    	and    rax,0xffff
  65e2f6:	74 15                	je     65e30d <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_3zip11ZipProducerIB2V_INtNtNtB1P_5slice6chunks14ChunksProducerhEB3p_EIB3q_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5X_s_0ENCB5X_0EE0NCB1G_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7f_E0TB7f_B7f_EEB61_+0x13d>
  65e2f8:	49 83 fc 02          	cmp    r12,0x2
  65e2fc:	0f 83 96 00 00 00    	jae    65e398 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_3zip11ZipProducerIB2V_INtNtNtB1P_5slice6chunks14ChunksProducerhEB3p_EIB3q_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5X_s_0ENCB5X_0EE0NCB1G_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7f_E0TB7f_B7f_EEB61_+0x1c8>
  65e302:	c1 e9 10             	shr    ecx,0x10
  65e305:	39 c1                	cmp    ecx,eax
  65e307:	0f 84 8b 00 00 00    	je     65e398 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_3zip11ZipProducerIB2V_INtNtNtB1P_5slice6chunks14ChunksProducerhEB3p_EIB3q_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5X_s_0ENCB5X_0EE0NCB1G_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7f_E0TB7f_B7f_EEB61_+0x1c8>
  65e30d:	48 8b 7c 24 50       	mov    rdi,QWORD PTR [rsp+0x50]
  65e312:	ff 15 50 94 0a 00    	call   QWORD PTR [rip+0xa9450]        # 707768 <_DYNAMIC+0x5d8>
  65e318:	c5 fa 6f 44 24 28    	vmovdqu xmm0,XMMWORD PTR [rsp+0x28]
  65e31e:	48 8b 44 24 20       	mov    rax,QWORD PTR [rsp+0x20]
  65e323:	48 89 c2             	mov    rdx,rax
  65e326:	4c 31 fa             	xor    rdx,r15
  65e329:	48 85 c0             	test   rax,rax
  65e32c:	b9 01 00 00 00       	mov    ecx,0x1
  65e331:	48 0f 48 ca          	cmovs  rcx,rdx
  65e335:	48 83 f9 01          	cmp    rcx,0x1
  65e339:	74 17                	je     65e352 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_3zip11ZipProducerIB2V_INtNtNtB1P_5slice6chunks14ChunksProducerhEB3p_EIB3q_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5X_s_0ENCB5X_0EE0NCB1G_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7f_E0TB7f_B7f_EEB61_+0x182>
  65e33b:	48 83 f9 02          	cmp    rcx,0x2
  65e33f:	75 67                	jne    65e3a8 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_3zip11ZipProducerIB2V_INtNtNtB1P_5slice6chunks14ChunksProducerhEB3p_EIB3q_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5X_s_0ENCB5X_0EE0NCB1G_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7f_E0TB7f_B7f_EEB61_+0x1d8>
  65e341:	c4 e1 f9 7e c7       	vmovq  rdi,xmm0
  65e346:	c4 e3 f9 16 c6 01    	vpextrq rsi,xmm0,0x1
  65e34c:	ff 15 1e 94 0a 00    	call   QWORD PTR [rip+0xa941e]        # 707770 <_DYNAMIC+0x5e0>
  65e352:	48 8b 4c 24 48       	mov    rcx,QWORD PTR [rsp+0x48]
  65e357:	48 89 4c 24 10       	mov    QWORD PTR [rsp+0x10],rcx
  65e35c:	c5 f8 10 4c 24 38    	vmovups xmm1,XMMWORD PTR [rsp+0x38]
  65e362:	c5 f8 29 0c 24       	vmovaps XMMWORD PTR [rsp],xmm1
  65e367:	4c 39 f8             	cmp    rax,r15
  65e36a:	74 55                	je     65e3c1 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_3zip11ZipProducerIB2V_INtNtNtB1P_5slice6chunks14ChunksProducerhEB3p_EIB3q_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5X_s_0ENCB5X_0EE0NCB1G_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7f_E0TB7f_B7f_EEB61_+0x1f1>
  65e36c:	48 89 03             	mov    QWORD PTR [rbx],rax
  65e36f:	c5 fa 7f 43 08       	vmovdqu XMMWORD PTR [rbx+0x8],xmm0
  65e374:	c5 f8 28 04 24       	vmovaps xmm0,XMMWORD PTR [rsp]
  65e379:	c5 f8 11 43 18       	vmovups XMMWORD PTR [rbx+0x18],xmm0
  65e37e:	48 8b 44 24 10       	mov    rax,QWORD PTR [rsp+0x10]
  65e383:	48 89 43 28          	mov    QWORD PTR [rbx+0x28],rax
  65e387:	48 81 c4 40 01 00 00 	add    rsp,0x140
  65e38e:	5b                   	pop    rbx
  65e38f:	41 5c                	pop    r12
  65e391:	41 5d                	pop    r13
  65e393:	41 5e                	pop    r14
  65e395:	41 5f                	pop    r15
  65e397:	c3                   	ret    
  65e398:	be 01 00 00 00       	mov    esi,0x1
  65e39d:	ff 15 d5 93 0a 00    	call   QWORD PTR [rip+0xa93d5]        # 707778 <_DYNAMIC+0x5e8>
  65e3a3:	e9 65 ff ff ff       	jmp    65e30d <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_3zip11ZipProducerIB2V_INtNtNtB1P_5slice6chunks14ChunksProducerhEB3p_EIB3q_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5X_s_0ENCB5X_0EE0NCB1G_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7f_E0TB7f_B7f_EEB61_+0x13d>
  65e3a8:	48 8d 3d 41 45 9b ff 	lea    rdi,[rip+0xffffffffff9b4541]        # 128f0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x17a2>
  65e3af:	48 8d 15 8a 5e 0a 00 	lea    rdx,[rip+0xa5e8a]        # 704240 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x3c0>
  65e3b6:	be 28 00 00 00       	mov    esi,0x28
  65e3bb:	ff 15 17 91 0a 00    	call   QWORD PTR [rip+0xa9117]        # 7074d8 <_DYNAMIC+0x348>
  65e3c1:	48 8d 3d f8 5a 0a 00 	lea    rdi,[rip+0xa5af8]        # 703ec0 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x40>
  65e3c8:	ff 15 aa 95 0a 00    	call   QWORD PTR [rip+0xa95aa]        # 707978 <_DYNAMIC+0x7e8>
  65e3ce:	48 89 c3             	mov    rbx,rax
  65e3d1:	48 8d 7c 24 20       	lea    rdi,[rsp+0x20]
  65e3d6:	e8 d5 25 00 00       	call   6609b0 <_RINvNtCsk9TLBDxQNFU_4core3ptr13drop_in_placeINtNtCsiKyBw8IgE1b_10rayon_core3job9JobResultTINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB1q_EEECs5Palesvl9wF_6mikros>
  65e3db:	48 89 df             	mov    rdi,rbx
  65e3de:	e8 4d 26 0a 00       	call   700a30 <_Unwind_Resume@plt>
  65e3e3:	ff 15 57 92 0a 00    	call   QWORD PTR [rip+0xa9257]        # 707640 <_DYNAMIC+0x4b0>
  65e3e9:	cc                   	int3   
  65e3ea:	cc                   	int3   
  65e3eb:	cc                   	int3   
  65e3ec:	cc                   	int3   
  65e3ed:	cc                   	int3   
  65e3ee:	cc                   	int3   
  65e3ef:	cc                   	int3   

