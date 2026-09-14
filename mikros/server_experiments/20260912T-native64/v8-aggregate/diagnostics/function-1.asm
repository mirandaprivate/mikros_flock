000000000065f040 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_3zip11ZipProducerIB2W_INtNtNtB1Q_5slice6chunks14ChunksProducerhEB3q_EIB3r_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5Y_s_0ENCB5Y_0EE0NCB1H_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7g_E0TB7g_B7g_EEB62_>:
  65f040:	55                   	push   rbp
  65f041:	41 57                	push   r15
  65f043:	41 56                	push   r14
  65f045:	41 55                	push   r13
  65f047:	41 54                	push   r12
  65f049:	53                   	push   rbx
  65f04a:	48 81 ec 38 01 00 00 	sub    rsp,0x138
  65f051:	49 89 d6             	mov    r14,rdx
  65f054:	49 89 f7             	mov    r15,rsi
  65f057:	48 89 fb             	mov    rbx,rdi
  65f05a:	48 8d 82 10 01 00 00 	lea    rax,[rdx+0x110]
  65f061:	48 8b 92 00 01 00 00 	mov    rdx,QWORD PTR [rdx+0x100]
  65f068:	48 89 84 24 18 01 00 	mov    QWORD PTR [rsp+0x118],rax
  65f06f:	00 
  65f070:	48 c7 84 24 20 01 00 	mov    QWORD PTR [rsp+0x120],0x0
  65f077:	00 00 00 00 00 
  65f07c:	48 89 94 24 28 01 00 	mov    QWORD PTR [rsp+0x128],rdx
  65f083:	00 
  65f084:	c6 84 24 30 01 00 00 	mov    BYTE PTR [rsp+0x130],0x1
  65f08b:	01 
  65f08c:	c5 fc 10 01          	vmovups ymm0,YMMWORD PTR [rcx]
  65f090:	c5 fc 10 49 20       	vmovups ymm1,YMMWORD PTR [rcx+0x20]
  65f095:	c5 fc 10 51 40       	vmovups ymm2,YMMWORD PTR [rcx+0x40]
  65f09a:	c5 fc 10 59 60       	vmovups ymm3,YMMWORD PTR [rcx+0x60]
  65f09f:	c5 fc 11 44 24 30    	vmovups YMMWORD PTR [rsp+0x30],ymm0
  65f0a5:	c5 fc 11 4c 24 50    	vmovups YMMWORD PTR [rsp+0x50],ymm1
  65f0ab:	c5 fc 11 54 24 70    	vmovups YMMWORD PTR [rsp+0x70],ymm2
  65f0b1:	c5 fc 11 9c 24 90 00 	vmovups YMMWORD PTR [rsp+0x90],ymm3
  65f0b8:	00 00 
  65f0ba:	c5 fc 10 81 80 00 00 	vmovups ymm0,YMMWORD PTR [rcx+0x80]
  65f0c1:	00 
  65f0c2:	c5 fc 11 84 24 b0 00 	vmovups YMMWORD PTR [rsp+0xb0],ymm0
  65f0c9:	00 00 
  65f0cb:	c5 fc 10 81 a0 00 00 	vmovups ymm0,YMMWORD PTR [rcx+0xa0]
  65f0d2:	00 
  65f0d3:	c5 fc 11 84 24 d0 00 	vmovups YMMWORD PTR [rsp+0xd0],ymm0
  65f0da:	00 00 
  65f0dc:	c5 fc 10 81 c0 00 00 	vmovups ymm0,YMMWORD PTR [rcx+0xc0]
  65f0e3:	00 
  65f0e4:	c5 fc 11 84 24 f0 00 	vmovups YMMWORD PTR [rsp+0xf0],ymm0
  65f0eb:	00 00 
  65f0ed:	49 bd 00 00 00 00 00 	movabs r13,0x8000000000000000
  65f0f4:	00 00 80 
  65f0f7:	48 8b 81 e0 00 00 00 	mov    rax,QWORD PTR [rcx+0xe0]
  65f0fe:	48 89 84 24 10 01 00 	mov    QWORD PTR [rsp+0x110],rax
  65f105:	00 
  65f106:	4c 89 2c 24          	mov    QWORD PTR [rsp],r13
  65f10a:	4c 8b 26             	mov    r12,QWORD PTR [rsi]
  65f10d:	48 8b ae 80 00 00 00 	mov    rbp,QWORD PTR [rsi+0x80]
  65f114:	48 8d 35 55 33 02 00 	lea    rsi,[rip+0x23355]        # 682470 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_>
  65f11b:	48 89 e2             	mov    rdx,rsp
  65f11e:	4c 89 ff             	mov    rdi,r15
  65f121:	c5 f8 77             	vzeroupper 
  65f124:	e8 f7 da 02 00       	call   68cc20 <_RNvMsg_NtCsh4KpTWVPfp2_15crossbeam_deque5dequeINtB5_8InjectorNtNtCsiKyBw8IgE1b_10rayon_core3job6JobRefE4pushB11_.llvm.10825995516073413387>
  65f129:	4c 31 e5             	xor    rbp,r12
  65f12c:	48 ba 00 00 00 00 01 	movabs rdx,0x100000000
  65f133:	00 00 00 
  65f136:	49 8d bf 58 01 00 00 	lea    rdi,[r15+0x158]
  65f13d:	f0 83 4c 24 c0 00    	lock or DWORD PTR [rsp-0x40],0x0
  65f143:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  65f14a:	84 00 00 00 00 00 
  65f150:	49 8b 87 70 01 00 00 	mov    rax,QWORD PTR [r15+0x170]
  65f157:	48 85 d0             	test   rax,rdx
  65f15a:	75 1e                	jne    65f17a <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_3zip11ZipProducerIB2W_INtNtNtB1Q_5slice6chunks14ChunksProducerhEB3q_EIB3r_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5Y_s_0ENCB5Y_0EE0NCB1H_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7g_E0TB7g_B7g_EEB62_+0x13a>
  65f15c:	48 89 c1             	mov    rcx,rax
  65f15f:	48 09 d1             	or     rcx,rdx
  65f162:	f0 49 0f b1 8f 70 01 	lock cmpxchg QWORD PTR [r15+0x170],rcx
  65f169:	00 00 
  65f16b:	75 e3                	jne    65f150 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_3zip11ZipProducerIB2W_INtNtNtB1Q_5slice6chunks14ChunksProducerhEB3q_EIB3r_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5Y_s_0ENCB5Y_0EE0NCB1H_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7g_E0TB7g_B7g_EEB62_+0x110>
  65f16d:	48 89 c8             	mov    rax,rcx
  65f170:	48 25 ff ff 00 00    	and    rax,0xffff
  65f176:	75 10                	jne    65f188 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_3zip11ZipProducerIB2W_INtNtNtB1Q_5slice6chunks14ChunksProducerhEB3q_EIB3r_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5Y_s_0ENCB5Y_0EE0NCB1H_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7g_E0TB7g_B7g_EEB62_+0x148>
  65f178:	eb 1b                	jmp    65f195 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_3zip11ZipProducerIB2W_INtNtNtB1Q_5slice6chunks14ChunksProducerhEB3q_EIB3r_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5Y_s_0ENCB5Y_0EE0NCB1H_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7g_E0TB7g_B7g_EEB62_+0x155>
  65f17a:	48 89 c1             	mov    rcx,rax
  65f17d:	48 89 c8             	mov    rax,rcx
  65f180:	48 25 ff ff 00 00    	and    rax,0xffff
  65f186:	74 0d                	je     65f195 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_3zip11ZipProducerIB2W_INtNtNtB1Q_5slice6chunks14ChunksProducerhEB3q_EIB3r_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5Y_s_0ENCB5Y_0EE0NCB1H_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7g_E0TB7g_B7g_EEB62_+0x155>
  65f188:	48 83 fd 02          	cmp    rbp,0x2
  65f18c:	73 79                	jae    65f207 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_3zip11ZipProducerIB2W_INtNtNtB1Q_5slice6chunks14ChunksProducerhEB3q_EIB3r_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5Y_s_0ENCB5Y_0EE0NCB1H_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7g_E0TB7g_B7g_EEB62_+0x1c7>
  65f18e:	c1 e9 10             	shr    ecx,0x10
  65f191:	39 c1                	cmp    ecx,eax
  65f193:	74 72                	je     65f207 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_3zip11ZipProducerIB2W_INtNtNtB1Q_5slice6chunks14ChunksProducerhEB3q_EIB3r_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5Y_s_0ENCB5Y_0EE0NCB1H_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7g_E0TB7g_B7g_EEB62_+0x1c7>
  65f195:	48 8b 84 24 20 01 00 	mov    rax,QWORD PTR [rsp+0x120]
  65f19c:	00 
  65f19d:	48 83 f8 03          	cmp    rax,0x3
  65f1a1:	75 71                	jne    65f214 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_3zip11ZipProducerIB2W_INtNtNtB1Q_5slice6chunks14ChunksProducerhEB3q_EIB3r_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5Y_s_0ENCB5Y_0EE0NCB1H_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7g_E0TB7g_B7g_EEB62_+0x1d4>
  65f1a3:	c5 fa 6f 44 24 08    	vmovdqu xmm0,XMMWORD PTR [rsp+0x8]
  65f1a9:	48 8b 04 24          	mov    rax,QWORD PTR [rsp]
  65f1ad:	49 31 c5             	xor    r13,rax
  65f1b0:	48 85 c0             	test   rax,rax
  65f1b3:	b9 01 00 00 00       	mov    ecx,0x1
  65f1b8:	49 0f 48 cd          	cmovs  rcx,r13
  65f1bc:	48 83 f9 01          	cmp    rcx,0x1
  65f1c0:	74 17                	je     65f1d9 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_3zip11ZipProducerIB2W_INtNtNtB1Q_5slice6chunks14ChunksProducerhEB3q_EIB3r_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5Y_s_0ENCB5Y_0EE0NCB1H_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7g_E0TB7g_B7g_EEB62_+0x199>
  65f1c2:	48 83 f9 02          	cmp    rcx,0x2
  65f1c6:	75 62                	jne    65f22a <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_3zip11ZipProducerIB2W_INtNtNtB1Q_5slice6chunks14ChunksProducerhEB3q_EIB3r_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5Y_s_0ENCB5Y_0EE0NCB1H_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7g_E0TB7g_B7g_EEB62_+0x1ea>
  65f1c8:	c4 e1 f9 7e c7       	vmovq  rdi,xmm0
  65f1cd:	c4 e3 f9 16 c6 01    	vpextrq rsi,xmm0,0x1
  65f1d3:	ff 15 97 85 0a 00    	call   QWORD PTR [rip+0xa8597]        # 707770 <_DYNAMIC+0x5e0>
  65f1d9:	48 89 03             	mov    QWORD PTR [rbx],rax
  65f1dc:	c5 fa 7f 43 08       	vmovdqu XMMWORD PTR [rbx+0x8],xmm0
  65f1e1:	c5 f8 10 44 24 18    	vmovups xmm0,XMMWORD PTR [rsp+0x18]
  65f1e7:	c5 f8 11 43 18       	vmovups XMMWORD PTR [rbx+0x18],xmm0
  65f1ec:	48 8b 44 24 28       	mov    rax,QWORD PTR [rsp+0x28]
  65f1f1:	48 89 43 28          	mov    QWORD PTR [rbx+0x28],rax
  65f1f5:	48 81 c4 38 01 00 00 	add    rsp,0x138
  65f1fc:	5b                   	pop    rbx
  65f1fd:	41 5c                	pop    r12
  65f1ff:	41 5d                	pop    r13
  65f201:	41 5e                	pop    r14
  65f203:	41 5f                	pop    r15
  65f205:	5d                   	pop    rbp
  65f206:	c3                   	ret    
  65f207:	be 01 00 00 00       	mov    esi,0x1
  65f20c:	ff 15 66 85 0a 00    	call   QWORD PTR [rip+0xa8566]        # 707778 <_DYNAMIC+0x5e8>
  65f212:	eb 81                	jmp    65f195 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_3zip11ZipProducerIB2W_INtNtNtB1Q_5slice6chunks14ChunksProducerhEB3q_EIB3r_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5Y_s_0ENCB5Y_0EE0NCB1H_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7g_E0TB7g_B7g_EEB62_+0x155>
  65f214:	4c 89 f7             	mov    rdi,r14
  65f217:	48 8d b4 24 20 01 00 	lea    rsi,[rsp+0x120]
  65f21e:	00 
  65f21f:	ff 15 5b 85 0a 00    	call   QWORD PTR [rip+0xa855b]        # 707780 <_DYNAMIC+0x5f0>
  65f225:	e9 79 ff ff ff       	jmp    65f1a3 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_3zip11ZipProducerIB2W_INtNtNtB1Q_5slice6chunks14ChunksProducerhEB3q_EIB3r_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5Y_s_0ENCB5Y_0EE0NCB1H_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7g_E0TB7g_B7g_EEB62_+0x163>
  65f22a:	48 8d 3d bf 36 9b ff 	lea    rdi,[rip+0xffffffffff9b36bf]        # 128f0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x17a2>
  65f231:	48 8d 15 08 50 0a 00 	lea    rdx,[rip+0xa5008]        # 704240 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x3c0>
  65f238:	be 28 00 00 00       	mov    esi,0x28
  65f23d:	ff 15 95 82 0a 00    	call   QWORD PTR [rip+0xa8295]        # 7074d8 <_DYNAMIC+0x348>
  65f243:	48 89 c3             	mov    rbx,rax
  65f246:	48 89 e7             	mov    rdi,rsp
  65f249:	e8 62 17 00 00       	call   6609b0 <_RINvNtCsk9TLBDxQNFU_4core3ptr13drop_in_placeINtNtCsiKyBw8IgE1b_10rayon_core3job9JobResultTINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB1q_EEECs5Palesvl9wF_6mikros>
  65f24e:	48 89 df             	mov    rdi,rbx
  65f251:	e8 da 17 0a 00       	call   700a30 <_Unwind_Resume@plt>
  65f256:	ff 15 e4 83 0a 00    	call   QWORD PTR [rip+0xa83e4]        # 707640 <_DYNAMIC+0x4b0>
  65f25c:	cc                   	int3   
  65f25d:	cc                   	int3   
  65f25e:	cc                   	int3   
  65f25f:	cc                   	int3   

