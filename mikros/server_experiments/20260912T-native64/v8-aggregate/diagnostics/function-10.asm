0000000000681190 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_3zip11ZipProducerIB47_INtNtNtB31_5slice6chunks14ChunksProducerhEB4B_EIB4C_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB79_s_0ENCB79_0EE0NCB2S_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8r_E0TB8r_B8r_EE00B9b_ENtB5_3Job7executeB7d_>:
  681190:	55                   	push   rbp
  681191:	41 57                	push   r15
  681193:	41 56                	push   r14
  681195:	41 55                	push   r13
  681197:	41 54                	push   r12
  681199:	53                   	push   rbx
  68119a:	48 81 ec 48 01 00 00 	sub    rsp,0x148
  6811a1:	48 8b 47 38          	mov    rax,QWORD PTR [rdi+0x38]
  6811a5:	c5 f8 10 47 40       	vmovups xmm0,XMMWORD PTR [rdi+0x40]
  6811aa:	48 c7 47 38 00 00 00 	mov    QWORD PTR [rdi+0x38],0x0
  6811b1:	00 
  6811b2:	48 85 c0             	test   rax,rax
  6811b5:	0f 84 a3 01 00 00    	je     68135e <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_3zip11ZipProducerIB47_INtNtNtB31_5slice6chunks14ChunksProducerhEB4B_EIB4C_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB79_s_0ENCB79_0EE0NCB2S_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8r_E0TB8r_B8r_EE00B9b_ENtB5_3Job7executeB7d_+0x1ce>
  6811bb:	48 89 fb             	mov    rbx,rdi
  6811be:	49 bc 00 00 00 00 00 	movabs r12,0x8000000000000000
  6811c5:	00 00 80 
  6811c8:	48 c7 c1 a8 ff ff ff 	mov    rcx,0xffffffffffffffa8
  6811cf:	64 48 8b 11          	mov    rdx,QWORD PTR fs:[rcx]
  6811d3:	4d 8d 6c 24 02       	lea    r13,[r12+0x2]
  6811d8:	48 85 d2             	test   rdx,rdx
  6811db:	0f 84 62 01 00 00    	je     681343 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_3zip11ZipProducerIB47_INtNtNtB31_5slice6chunks14ChunksProducerhEB4B_EIB4C_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB79_s_0ENCB79_0EE0NCB2S_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8r_E0TB8r_B8r_EE00B9b_ENtB5_3Job7executeB7d_+0x1b3>
  6811e1:	48 8d 4b 50          	lea    rcx,[rbx+0x50]
  6811e5:	48 8d 73 68          	lea    rsi,[rbx+0x68]
  6811e9:	48 89 44 24 60       	mov    QWORD PTR [rsp+0x60],rax
  6811ee:	c5 f8 11 44 24 68    	vmovups XMMWORD PTR [rsp+0x68],xmm0
  6811f4:	c5 f8 10 01          	vmovups xmm0,XMMWORD PTR [rcx]
  6811f8:	c5 f8 11 44 24 78    	vmovups XMMWORD PTR [rsp+0x78],xmm0
  6811fe:	48 8b 41 10          	mov    rax,QWORD PTR [rcx+0x10]
  681202:	48 89 84 24 88 00 00 	mov    QWORD PTR [rsp+0x88],rax
  681209:	00 
  68120a:	c5 fc 10 06          	vmovups ymm0,YMMWORD PTR [rsi]
  68120e:	c5 fc 10 4e 20       	vmovups ymm1,YMMWORD PTR [rsi+0x20]
  681213:	c5 fc 10 56 40       	vmovups ymm2,YMMWORD PTR [rsi+0x40]
  681218:	c5 fc 10 5e 60       	vmovups ymm3,YMMWORD PTR [rsi+0x60]
  68121d:	c5 fc 11 84 24 90 00 	vmovups YMMWORD PTR [rsp+0x90],ymm0
  681224:	00 00 
  681226:	c5 fc 11 8c 24 b0 00 	vmovups YMMWORD PTR [rsp+0xb0],ymm1
  68122d:	00 00 
  68122f:	c5 fc 11 94 24 d0 00 	vmovups YMMWORD PTR [rsp+0xd0],ymm2
  681236:	00 00 
  681238:	c5 fc 11 9c 24 f0 00 	vmovups YMMWORD PTR [rsp+0xf0],ymm3
  68123f:	00 00 
  681241:	c5 fc 10 86 80 00 00 	vmovups ymm0,YMMWORD PTR [rsi+0x80]
  681248:	00 
  681249:	c5 fc 11 84 24 10 01 	vmovups YMMWORD PTR [rsp+0x110],ymm0
  681250:	00 00 
  681252:	c5 fc 10 86 98 00 00 	vmovups ymm0,YMMWORD PTR [rsi+0x98]
  681259:	00 
  68125a:	c5 fc 11 84 24 28 01 	vmovups YMMWORD PTR [rsp+0x128],ymm0
  681261:	00 00 
  681263:	48 8d 7c 24 30       	lea    rdi,[rsp+0x30]
  681268:	48 8d 74 24 60       	lea    rsi,[rsp+0x60]
  68126d:	b9 01 00 00 00       	mov    ecx,0x1
  681272:	c5 f8 77             	vzeroupper 
  681275:	e8 56 6a fe ff       	call   667cd0 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_>
  68127a:	4c 8b 6c 24 30       	mov    r13,QWORD PTR [rsp+0x30]
  68127f:	4c 8b 74 24 38       	mov    r14,QWORD PTR [rsp+0x38]
  681284:	c5 f8 10 44 24 48    	vmovups xmm0,XMMWORD PTR [rsp+0x48]
  68128a:	c5 f8 29 44 24 60    	vmovaps XMMWORD PTR [rsp+0x60],xmm0
  681290:	4c 8b 7c 24 40       	mov    r15,QWORD PTR [rsp+0x40]
  681295:	48 8b 44 24 58       	mov    rax,QWORD PTR [rsp+0x58]
  68129a:	48 89 44 24 70       	mov    QWORD PTR [rsp+0x70],rax
  68129f:	c5 f8 29 44 24 10    	vmovaps XMMWORD PTR [rsp+0x10],xmm0
  6812a5:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
  6812aa:	48 8b 03             	mov    rax,QWORD PTR [rbx]
  6812ad:	49 31 c4             	xor    r12,rax
  6812b0:	48 85 c0             	test   rax,rax
  6812b3:	b9 01 00 00 00       	mov    ecx,0x1
  6812b8:	49 0f 48 cc          	cmovs  rcx,r12
  6812bc:	48 85 c9             	test   rcx,rcx
  6812bf:	74 48                	je     681309 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_3zip11ZipProducerIB47_INtNtNtB31_5slice6chunks14ChunksProducerhEB4B_EIB4C_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB79_s_0ENCB79_0EE0NCB2S_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8r_E0TB8r_B8r_EE00B9b_ENtB5_3Job7executeB7d_+0x179>
  6812c1:	48 83 f9 01          	cmp    rcx,0x1
  6812c5:	75 1c                	jne    6812e3 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_3zip11ZipProducerIB47_INtNtNtB31_5slice6chunks14ChunksProducerhEB4B_EIB4C_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB79_s_0ENCB79_0EE0NCB2S_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8r_E0TB8r_B8r_EE00B9b_ENtB5_3Job7executeB7d_+0x153>
  6812c7:	48 85 c0             	test   rax,rax
  6812ca:	74 0a                	je     6812d6 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_3zip11ZipProducerIB47_INtNtNtB31_5slice6chunks14ChunksProducerhEB4B_EIB4C_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB79_s_0ENCB79_0EE0NCB2S_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8r_E0TB8r_B8r_EE00B9b_ENtB5_3Job7executeB7d_+0x146>
  6812cc:	48 8b 7b 08          	mov    rdi,QWORD PTR [rbx+0x8]
  6812d0:	ff 15 c2 60 08 00    	call   QWORD PTR [rip+0x860c2]        # 707398 <free@GLIBC_2.2.5>
  6812d6:	48 83 7b 18 00       	cmp    QWORD PTR [rbx+0x18],0x0
  6812db:	74 2c                	je     681309 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_3zip11ZipProducerIB47_INtNtNtB31_5slice6chunks14ChunksProducerhEB4B_EIB4C_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB79_s_0ENCB79_0EE0NCB2S_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8r_E0TB8r_B8r_EE00B9b_ENtB5_3Job7executeB7d_+0x179>
  6812dd:	4c 8b 63 20          	mov    r12,QWORD PTR [rbx+0x20]
  6812e1:	eb 1d                	jmp    681300 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_3zip11ZipProducerIB47_INtNtNtB31_5slice6chunks14ChunksProducerhEB4B_EIB4C_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB79_s_0ENCB79_0EE0NCB2S_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8r_E0TB8r_B8r_EE00B9b_ENtB5_3Job7executeB7d_+0x170>
  6812e3:	4c 8b 63 08          	mov    r12,QWORD PTR [rbx+0x8]
  6812e7:	48 8b 6b 10          	mov    rbp,QWORD PTR [rbx+0x10]
  6812eb:	48 8b 45 00          	mov    rax,QWORD PTR [rbp+0x0]
  6812ef:	48 85 c0             	test   rax,rax
  6812f2:	74 05                	je     6812f9 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_3zip11ZipProducerIB47_INtNtNtB31_5slice6chunks14ChunksProducerhEB4B_EIB4C_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB79_s_0ENCB79_0EE0NCB2S_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8r_E0TB8r_B8r_EE00B9b_ENtB5_3Job7executeB7d_+0x169>
  6812f4:	4c 89 e7             	mov    rdi,r12
  6812f7:	ff d0                	call   rax
  6812f9:	48 83 7d 08 00       	cmp    QWORD PTR [rbp+0x8],0x0
  6812fe:	74 09                	je     681309 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_3zip11ZipProducerIB47_INtNtNtB31_5slice6chunks14ChunksProducerhEB4B_EIB4C_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB79_s_0ENCB79_0EE0NCB2S_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8r_E0TB8r_B8r_EE00B9b_ENtB5_3Job7executeB7d_+0x179>
  681300:	4c 89 e7             	mov    rdi,r12
  681303:	ff 15 8f 60 08 00    	call   QWORD PTR [rip+0x8608f]        # 707398 <free@GLIBC_2.2.5>
  681309:	4c 89 2b             	mov    QWORD PTR [rbx],r13
  68130c:	4c 89 73 08          	mov    QWORD PTR [rbx+0x8],r14
  681310:	4c 89 7b 10          	mov    QWORD PTR [rbx+0x10],r15
  681314:	c5 f8 28 44 24 10    	vmovaps xmm0,XMMWORD PTR [rsp+0x10]
  68131a:	c5 f8 11 43 18       	vmovups XMMWORD PTR [rbx+0x18],xmm0
  68131f:	48 8b 44 24 20       	mov    rax,QWORD PTR [rsp+0x20]
  681324:	48 89 43 28          	mov    QWORD PTR [rbx+0x28],rax
  681328:	48 8b 7b 30          	mov    rdi,QWORD PTR [rbx+0x30]
  68132c:	e8 1f 3d 00 00       	call   685050 <_RNvXsd_NtCsiKyBw8IgE1b_10rayon_core5latchINtB5_8LatchRefNtB5_9LockLatchENtB5_5Latch3setCs5Palesvl9wF_6mikros>
  681331:	48 81 c4 48 01 00 00 	add    rsp,0x148
  681338:	5b                   	pop    rbx
  681339:	41 5c                	pop    r12
  68133b:	41 5d                	pop    r13
  68133d:	41 5e                	pop    r14
  68133f:	41 5f                	pop    r15
  681341:	5d                   	pop    rbp
  681342:	c3                   	ret    
  681343:	48 8d 3d ce 15 99 ff 	lea    rdi,[rip+0xffffffffff9915ce]        # 12918 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x17ca>
  68134a:	48 8d 15 ef 2b 08 00 	lea    rdx,[rip+0x82bef]        # 703f40 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0xc0>
  681351:	be 36 00 00 00       	mov    esi,0x36
  681356:	ff 15 7c 61 08 00    	call   QWORD PTR [rip+0x8617c]        # 7074d8 <_DYNAMIC+0x348>
  68135c:	eb 4b                	jmp    6813a9 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_3zip11ZipProducerIB47_INtNtNtB31_5slice6chunks14ChunksProducerhEB4B_EIB4C_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB79_s_0ENCB79_0EE0NCB2S_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8r_E0TB8r_B8r_EE00B9b_ENtB5_3Job7executeB7d_+0x219>
  68135e:	48 8d 3d 1b 34 08 00 	lea    rdi,[rip+0x8341b]        # 704780 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x900>
  681365:	ff 15 c5 61 08 00    	call   QWORD PTR [rip+0x861c5]        # 707530 <_DYNAMIC+0x3a0>
  68136b:	eb 3c                	jmp    6813a9 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_3zip11ZipProducerIB47_INtNtNtB31_5slice6chunks14ChunksProducerhEB4B_EIB4C_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB79_s_0ENCB79_0EE0NCB2S_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8r_E0TB8r_B8r_EE00B9b_ENtB5_3Job7executeB7d_+0x219>
  68136d:	48 83 7d 08 00       	cmp    QWORD PTR [rbp+0x8],0x0
  681372:	74 09                	je     68137d <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_3zip11ZipProducerIB47_INtNtNtB31_5slice6chunks14ChunksProducerhEB4B_EIB4C_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB79_s_0ENCB79_0EE0NCB2S_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8r_E0TB8r_B8r_EE00B9b_ENtB5_3Job7executeB7d_+0x1ed>
  681374:	4c 89 e7             	mov    rdi,r12
  681377:	ff 15 1b 60 08 00    	call   QWORD PTR [rip+0x8601b]        # 707398 <free@GLIBC_2.2.5>
  68137d:	4c 89 2b             	mov    QWORD PTR [rbx],r13
  681380:	4c 89 73 08          	mov    QWORD PTR [rbx+0x8],r14
  681384:	4c 89 7b 10          	mov    QWORD PTR [rbx+0x10],r15
  681388:	c5 f8 28 44 24 10    	vmovaps xmm0,XMMWORD PTR [rsp+0x10]
  68138e:	c5 f8 11 43 18       	vmovups XMMWORD PTR [rbx+0x18],xmm0
  681393:	48 8b 44 24 20       	mov    rax,QWORD PTR [rsp+0x20]
  681398:	48 89 43 28          	mov    QWORD PTR [rbx+0x28],rax
  68139c:	eb 00                	jmp    68139e <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_3zip11ZipProducerIB47_INtNtNtB31_5slice6chunks14ChunksProducerhEB4B_EIB4C_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB79_s_0ENCB79_0EE0NCB2S_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8r_E0TB8r_B8r_EE00B9b_ENtB5_3Job7executeB7d_+0x20e>
  68139e:	48 8d 7c 24 0f       	lea    rdi,[rsp+0xf]
  6813a3:	ff 15 5f 65 08 00    	call   QWORD PTR [rip+0x8655f]        # 707908 <_DYNAMIC+0x778>
  6813a9:	0f 0b                	ud2    
  6813ab:	ff 15 8f 62 08 00    	call   QWORD PTR [rip+0x8628f]        # 707640 <_DYNAMIC+0x4b0>
  6813b1:	48 89 c7             	mov    rdi,rax
  6813b4:	ff 15 16 64 08 00    	call   QWORD PTR [rip+0x86416]        # 7077d0 <_DYNAMIC+0x640>
  6813ba:	49 89 c6             	mov    r14,rax
  6813bd:	49 89 d7             	mov    r15,rdx
  6813c0:	e9 e5 fe ff ff       	jmp    6812aa <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2Z_3zip11ZipProducerIB47_INtNtNtB31_5slice6chunks14ChunksProducerhEB4B_EIB4C_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2Z_3map11MapConsumerINtNtB2Z_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB79_s_0ENCB79_0EE0NCB2S_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8r_E0TB8r_B8r_EE00B9b_ENtB5_3Job7executeB7d_+0x11a>
  6813c5:	cc                   	int3   
  6813c6:	cc                   	int3   
  6813c7:	cc                   	int3   
  6813c8:	cc                   	int3   
  6813c9:	cc                   	int3   
  6813ca:	cc                   	int3   
  6813cb:	cc                   	int3   
  6813cc:	cc                   	int3   
  6813cd:	cc                   	int3   
  6813ce:	cc                   	int3   
  6813cf:	cc                   	int3   

