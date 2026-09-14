0000000000682470 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_>:
  682470:	55                   	push   rbp
  682471:	41 57                	push   r15
  682473:	41 56                	push   r14
  682475:	41 55                	push   r13
  682477:	41 54                	push   r12
  682479:	53                   	push   rbx
  68247a:	48 81 ec 48 01 00 00 	sub    rsp,0x148
  682481:	48 8b 47 30          	mov    rax,QWORD PTR [rdi+0x30]
  682485:	c5 f8 10 47 38       	vmovups xmm0,XMMWORD PTR [rdi+0x38]
  68248a:	48 c7 47 30 00 00 00 	mov    QWORD PTR [rdi+0x30],0x0
  682491:	00 
  682492:	48 85 c0             	test   rax,rax
  682495:	0f 84 0b 02 00 00    	je     6826a6 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_+0x236>
  68249b:	48 89 fb             	mov    rbx,rdi
  68249e:	49 bc 00 00 00 00 00 	movabs r12,0x8000000000000000
  6824a5:	00 00 80 
  6824a8:	48 c7 c1 a8 ff ff ff 	mov    rcx,0xffffffffffffffa8
  6824af:	64 48 8b 11          	mov    rdx,QWORD PTR fs:[rcx]
  6824b3:	4d 8d 6c 24 02       	lea    r13,[r12+0x2]
  6824b8:	48 85 d2             	test   rdx,rdx
  6824bb:	0f 84 ca 01 00 00    	je     68268b <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_+0x21b>
  6824c1:	48 8d 4b 48          	lea    rcx,[rbx+0x48]
  6824c5:	48 8d 73 60          	lea    rsi,[rbx+0x60]
  6824c9:	48 89 44 24 60       	mov    QWORD PTR [rsp+0x60],rax
  6824ce:	c5 f8 11 44 24 68    	vmovups XMMWORD PTR [rsp+0x68],xmm0
  6824d4:	c5 f8 10 01          	vmovups xmm0,XMMWORD PTR [rcx]
  6824d8:	c5 f8 11 44 24 78    	vmovups XMMWORD PTR [rsp+0x78],xmm0
  6824de:	48 8b 41 10          	mov    rax,QWORD PTR [rcx+0x10]
  6824e2:	48 89 84 24 88 00 00 	mov    QWORD PTR [rsp+0x88],rax
  6824e9:	00 
  6824ea:	c5 fc 10 06          	vmovups ymm0,YMMWORD PTR [rsi]
  6824ee:	c5 fc 10 4e 20       	vmovups ymm1,YMMWORD PTR [rsi+0x20]
  6824f3:	c5 fc 10 56 40       	vmovups ymm2,YMMWORD PTR [rsi+0x40]
  6824f8:	c5 fc 10 5e 60       	vmovups ymm3,YMMWORD PTR [rsi+0x60]
  6824fd:	c5 fc 11 84 24 90 00 	vmovups YMMWORD PTR [rsp+0x90],ymm0
  682504:	00 00 
  682506:	c5 fc 11 8c 24 b0 00 	vmovups YMMWORD PTR [rsp+0xb0],ymm1
  68250d:	00 00 
  68250f:	c5 fc 11 94 24 d0 00 	vmovups YMMWORD PTR [rsp+0xd0],ymm2
  682516:	00 00 
  682518:	c5 fc 11 9c 24 f0 00 	vmovups YMMWORD PTR [rsp+0xf0],ymm3
  68251f:	00 00 
  682521:	c5 fc 10 86 80 00 00 	vmovups ymm0,YMMWORD PTR [rsi+0x80]
  682528:	00 
  682529:	c5 fc 11 84 24 10 01 	vmovups YMMWORD PTR [rsp+0x110],ymm0
  682530:	00 00 
  682532:	c5 fc 10 86 98 00 00 	vmovups ymm0,YMMWORD PTR [rsi+0x98]
  682539:	00 
  68253a:	c5 fc 11 84 24 28 01 	vmovups YMMWORD PTR [rsp+0x128],ymm0
  682541:	00 00 
  682543:	48 8d 7c 24 30       	lea    rdi,[rsp+0x30]
  682548:	48 8d 74 24 60       	lea    rsi,[rsp+0x60]
  68254d:	b9 01 00 00 00       	mov    ecx,0x1
  682552:	c5 f8 77             	vzeroupper 
  682555:	e8 76 57 fe ff       	call   667cd0 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_>
  68255a:	4c 8b 6c 24 30       	mov    r13,QWORD PTR [rsp+0x30]
  68255f:	4c 8b 74 24 38       	mov    r14,QWORD PTR [rsp+0x38]
  682564:	c5 f8 10 44 24 48    	vmovups xmm0,XMMWORD PTR [rsp+0x48]
  68256a:	c5 f8 29 44 24 60    	vmovaps XMMWORD PTR [rsp+0x60],xmm0
  682570:	4c 8b 7c 24 40       	mov    r15,QWORD PTR [rsp+0x40]
  682575:	48 8b 44 24 58       	mov    rax,QWORD PTR [rsp+0x58]
  68257a:	48 89 44 24 70       	mov    QWORD PTR [rsp+0x70],rax
  68257f:	c5 f8 29 44 24 10    	vmovaps XMMWORD PTR [rsp+0x10],xmm0
  682585:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
  68258a:	48 8b 03             	mov    rax,QWORD PTR [rbx]
  68258d:	49 31 c4             	xor    r12,rax
  682590:	48 85 c0             	test   rax,rax
  682593:	b9 01 00 00 00       	mov    ecx,0x1
  682598:	49 0f 48 cc          	cmovs  rcx,r12
  68259c:	48 85 c9             	test   rcx,rcx
  68259f:	74 48                	je     6825e9 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_+0x179>
  6825a1:	48 83 f9 01          	cmp    rcx,0x1
  6825a5:	75 1c                	jne    6825c3 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_+0x153>
  6825a7:	48 85 c0             	test   rax,rax
  6825aa:	74 0a                	je     6825b6 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_+0x146>
  6825ac:	48 8b 7b 08          	mov    rdi,QWORD PTR [rbx+0x8]
  6825b0:	ff 15 e2 4d 08 00    	call   QWORD PTR [rip+0x84de2]        # 707398 <free@GLIBC_2.2.5>
  6825b6:	48 83 7b 18 00       	cmp    QWORD PTR [rbx+0x18],0x0
  6825bb:	74 2c                	je     6825e9 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_+0x179>
  6825bd:	4c 8b 63 20          	mov    r12,QWORD PTR [rbx+0x20]
  6825c1:	eb 1d                	jmp    6825e0 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_+0x170>
  6825c3:	4c 8b 63 08          	mov    r12,QWORD PTR [rbx+0x8]
  6825c7:	48 8b 6b 10          	mov    rbp,QWORD PTR [rbx+0x10]
  6825cb:	48 8b 45 00          	mov    rax,QWORD PTR [rbp+0x0]
  6825cf:	48 85 c0             	test   rax,rax
  6825d2:	74 05                	je     6825d9 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_+0x169>
  6825d4:	4c 89 e7             	mov    rdi,r12
  6825d7:	ff d0                	call   rax
  6825d9:	48 83 7d 08 00       	cmp    QWORD PTR [rbp+0x8],0x0
  6825de:	74 09                	je     6825e9 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_+0x179>
  6825e0:	4c 89 e7             	mov    rdi,r12
  6825e3:	ff 15 af 4d 08 00    	call   QWORD PTR [rip+0x84daf]        # 707398 <free@GLIBC_2.2.5>
  6825e9:	4c 89 2b             	mov    QWORD PTR [rbx],r13
  6825ec:	4c 89 73 08          	mov    QWORD PTR [rbx+0x8],r14
  6825f0:	4c 89 7b 10          	mov    QWORD PTR [rbx+0x10],r15
  6825f4:	c5 f8 28 44 24 10    	vmovaps xmm0,XMMWORD PTR [rsp+0x10]
  6825fa:	c5 f8 11 43 18       	vmovups XMMWORD PTR [rbx+0x18],xmm0
  6825ff:	48 8b 44 24 20       	mov    rax,QWORD PTR [rsp+0x20]
  682604:	48 89 43 28          	mov    QWORD PTR [rbx+0x28],rax
  682608:	0f b6 ab 30 01 00 00 	movzx  ebp,BYTE PTR [rbx+0x130]
  68260f:	40 84 ed             	test   bpl,bpl
  682612:	74 1b                	je     68262f <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_+0x1bf>
  682614:	48 8b 83 18 01 00 00 	mov    rax,QWORD PTR [rbx+0x118]
  68261b:	48 8b 38             	mov    rdi,QWORD PTR [rax]
  68261e:	f0 48 ff 07          	lock inc QWORD PTR [rdi]
  682622:	0f 8e ec 00 00 00    	jle    682714 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_+0x2a4>
  682628:	48 89 7c 24 10       	mov    QWORD PTR [rsp+0x10],rdi
  68262d:	eb 0a                	jmp    682639 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_+0x1c9>
  68262f:	48 8b 83 18 01 00 00 	mov    rax,QWORD PTR [rbx+0x118]
  682636:	48 8b 38             	mov    rdi,QWORD PTR [rax]
  682639:	48 8b b3 28 01 00 00 	mov    rsi,QWORD PTR [rbx+0x128]
  682640:	b8 03 00 00 00       	mov    eax,0x3
  682645:	48 87 83 20 01 00 00 	xchg   QWORD PTR [rbx+0x120],rax
  68264c:	48 83 f8 02          	cmp    rax,0x2
  682650:	75 0c                	jne    68265e <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_+0x1ee>
  682652:	48 81 c7 d8 01 00 00 	add    rdi,0x1d8
  682659:	e8 d2 8d 00 00       	call   68b430 <_RNvMNtCsiKyBw8IgE1b_10rayon_core5sleepNtB2_5Sleep20wake_specific_thread.llvm.10825995516073413387>
  68265e:	40 84 ed             	test   bpl,bpl
  682661:	74 16                	je     682679 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_+0x209>
  682663:	48 8b 44 24 10       	mov    rax,QWORD PTR [rsp+0x10]
  682668:	f0 48 ff 08          	lock dec QWORD PTR [rax]
  68266c:	75 0b                	jne    682679 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_+0x209>
  68266e:	48 8d 7c 24 10       	lea    rdi,[rsp+0x10]
  682673:	ff 15 97 52 08 00    	call   QWORD PTR [rip+0x85297]        # 707910 <_DYNAMIC+0x780>
  682679:	48 81 c4 48 01 00 00 	add    rsp,0x148
  682680:	5b                   	pop    rbx
  682681:	41 5c                	pop    r12
  682683:	41 5d                	pop    r13
  682685:	41 5e                	pop    r14
  682687:	41 5f                	pop    r15
  682689:	5d                   	pop    rbp
  68268a:	c3                   	ret    
  68268b:	48 8d 3d 86 02 99 ff 	lea    rdi,[rip+0xffffffffff990286]        # 12918 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x17ca>
  682692:	48 8d 15 77 18 08 00 	lea    rdx,[rip+0x81877]        # 703f10 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x90>
  682699:	be 36 00 00 00       	mov    esi,0x36
  68269e:	ff 15 34 4e 08 00    	call   QWORD PTR [rip+0x84e34]        # 7074d8 <_DYNAMIC+0x348>
  6826a4:	eb 6e                	jmp    682714 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_+0x2a4>
  6826a6:	48 8d 3d d3 20 08 00 	lea    rdi,[rip+0x820d3]        # 704780 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x900>
  6826ad:	ff 15 7d 4e 08 00    	call   QWORD PTR [rip+0x84e7d]        # 707530 <_DYNAMIC+0x3a0>
  6826b3:	eb 5f                	jmp    682714 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_+0x2a4>
  6826b5:	48 83 7d 08 00       	cmp    QWORD PTR [rbp+0x8],0x0
  6826ba:	74 09                	je     6826c5 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_+0x255>
  6826bc:	4c 89 e7             	mov    rdi,r12
  6826bf:	ff 15 d3 4c 08 00    	call   QWORD PTR [rip+0x84cd3]        # 707398 <free@GLIBC_2.2.5>
  6826c5:	4c 89 2b             	mov    QWORD PTR [rbx],r13
  6826c8:	4c 89 73 08          	mov    QWORD PTR [rbx+0x8],r14
  6826cc:	4c 89 7b 10          	mov    QWORD PTR [rbx+0x10],r15
  6826d0:	c5 f8 28 44 24 10    	vmovaps xmm0,XMMWORD PTR [rsp+0x10]
  6826d6:	c5 f8 11 43 18       	vmovups XMMWORD PTR [rbx+0x18],xmm0
  6826db:	48 8b 44 24 20       	mov    rax,QWORD PTR [rsp+0x20]
  6826e0:	48 89 43 28          	mov    QWORD PTR [rbx+0x28],rax
  6826e4:	eb 23                	jmp    682709 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_+0x299>
  6826e6:	40 84 ed             	test   bpl,bpl
  6826e9:	74 1e                	je     682709 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_+0x299>
  6826eb:	48 8b 44 24 10       	mov    rax,QWORD PTR [rsp+0x10]
  6826f0:	f0 48 ff 08          	lock dec QWORD PTR [rax]
  6826f4:	75 13                	jne    682709 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_+0x299>
  6826f6:	48 8d 7c 24 10       	lea    rdi,[rsp+0x10]
  6826fb:	ff 15 0f 52 08 00    	call   QWORD PTR [rip+0x8520f]        # 707910 <_DYNAMIC+0x780>
  682701:	eb 06                	jmp    682709 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_+0x299>
  682703:	ff 15 37 4f 08 00    	call   QWORD PTR [rip+0x84f37]        # 707640 <_DYNAMIC+0x4b0>
  682709:	48 8d 7c 24 0f       	lea    rdi,[rsp+0xf]
  68270e:	ff 15 f4 51 08 00    	call   QWORD PTR [rip+0x851f4]        # 707908 <_DYNAMIC+0x778>
  682714:	0f 0b                	ud2    
  682716:	ff 15 24 4f 08 00    	call   QWORD PTR [rip+0x84f24]        # 707640 <_DYNAMIC+0x4b0>
  68271c:	48 89 c7             	mov    rdi,rax
  68271f:	ff 15 ab 50 08 00    	call   QWORD PTR [rip+0x850ab]        # 7077d0 <_DYNAMIC+0x640>
  682725:	49 89 c6             	mov    r14,rax
  682728:	49 89 d7             	mov    r15,rdx
  68272b:	e9 5a fe ff ff       	jmp    68258a <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2I_3zip11ZipProducerIB3Q_INtNtNtB2K_5slice6chunks14ChunksProducerhEB4k_EIB4l_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2I_3map11MapConsumerINtNtB2I_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6S_s_0ENCB6S_0EE0NCB2B_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB8a_E0TB8a_B8a_EE0B8U_ENtB5_3Job7executeB6W_+0x11a>

