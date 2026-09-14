0000000000684590 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_ENCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2C_3zip11ZipProducerIB3K_INtNtNtB2E_5slice6chunks14ChunksProducerhEB4e_EIB4f_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2C_3map11MapConsumerINtNtB2C_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6M_s_0ENCB6M_0EEs_0E0B1P_ENtB5_3Job7executeB6Q_>:
  684590:	55                   	push   rbp
  684591:	41 57                	push   r15
  684593:	41 56                	push   r14
  684595:	41 55                	push   r13
  684597:	41 54                	push   r12
  684599:	53                   	push   rbx
  68459a:	48 81 ec a8 00 00 00 	sub    rsp,0xa8
  6845a1:	48 8b 17             	mov    rdx,QWORD PTR [rdi]
  6845a4:	48 8b 4f 08          	mov    rcx,QWORD PTR [rdi+0x8]
  6845a8:	48 8b 47 10          	mov    rax,QWORD PTR [rdi+0x10]
  6845ac:	48 c7 07 00 00 00 00 	mov    QWORD PTR [rdi],0x0
  6845b3:	48 85 d2             	test   rdx,rdx
  6845b6:	0f 84 71 01 00 00    	je     68472d <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_ENCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2C_3zip11ZipProducerIB3K_INtNtNtB2E_5slice6chunks14ChunksProducerhEB4e_EIB4f_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2C_3map11MapConsumerINtNtB2C_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6M_s_0ENCB6M_0EEs_0E0B1P_ENtB5_3Job7executeB6Q_+0x19d>
  6845bc:	48 89 fb             	mov    rbx,rdi
  6845bf:	49 bc 00 00 00 00 00 	movabs r12,0x8000000000000000
  6845c6:	00 00 80 
  6845c9:	4c 8d 4c 24 48       	lea    r9,[rsp+0x48]
  6845ce:	c5 fc 10 47 18       	vmovups ymm0,YMMWORD PTR [rdi+0x18]
  6845d3:	c5 fc 10 4f 38       	vmovups ymm1,YMMWORD PTR [rdi+0x38]
  6845d8:	c5 fc 10 57 58       	vmovups ymm2,YMMWORD PTR [rdi+0x58]
  6845dd:	c5 fc 11 94 24 88 00 	vmovups YMMWORD PTR [rsp+0x88],ymm2
  6845e4:	00 00 
  6845e6:	c5 fc 11 4c 24 68    	vmovups YMMWORD PTR [rsp+0x68],ymm1
  6845ec:	c5 fc 11 44 24 48    	vmovups YMMWORD PTR [rsp+0x48],ymm0
  6845f2:	48 89 54 24 30       	mov    QWORD PTR [rsp+0x30],rdx
  6845f7:	48 89 4c 24 38       	mov    QWORD PTR [rsp+0x38],rcx
  6845fc:	48 89 44 24 40       	mov    QWORD PTR [rsp+0x40],rax
  684601:	48 8d bc 24 90 00 00 	lea    rdi,[rsp+0x90]
  684608:	00 
  684609:	48 8b 32             	mov    rsi,QWORD PTR [rdx]
  68460c:	48 2b 31             	sub    rsi,QWORD PTR [rcx]
  68460f:	48 8b 08             	mov    rcx,QWORD PTR [rax]
  684612:	4c 8b 40 08          	mov    r8,QWORD PTR [rax+0x8]
  684616:	4d 8d 6c 24 02       	lea    r13,[r12+0x2]
  68461b:	48 89 3c 24          	mov    QWORD PTR [rsp],rdi
  68461f:	48 8d 7c 24 18       	lea    rdi,[rsp+0x18]
  684624:	ba 01 00 00 00       	mov    edx,0x1
  684629:	c5 f8 77             	vzeroupper 
  68462c:	e8 7f c8 fd ff       	call   660eb0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_>
  684631:	4c 8b 6c 24 18       	mov    r13,QWORD PTR [rsp+0x18]
  684636:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
  68463b:	4c 8b 74 24 28       	mov    r14,QWORD PTR [rsp+0x28]
  684640:	48 8b 43 78          	mov    rax,QWORD PTR [rbx+0x78]
  684644:	49 31 c4             	xor    r12,rax
  684647:	48 85 c0             	test   rax,rax
  68464a:	b9 01 00 00 00       	mov    ecx,0x1
  68464f:	49 0f 48 cc          	cmovs  rcx,r12
  684653:	48 85 c9             	test   rcx,rcx
  684656:	74 40                	je     684698 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_ENCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2C_3zip11ZipProducerIB3K_INtNtNtB2E_5slice6chunks14ChunksProducerhEB4e_EIB4f_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2C_3map11MapConsumerINtNtB2C_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6M_s_0ENCB6M_0EEs_0E0B1P_ENtB5_3Job7executeB6Q_+0x108>
  684658:	48 83 f9 01          	cmp    rcx,0x1
  68465c:	75 0e                	jne    68466c <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_ENCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2C_3zip11ZipProducerIB3K_INtNtNtB2E_5slice6chunks14ChunksProducerhEB4e_EIB4f_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2C_3map11MapConsumerINtNtB2C_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6M_s_0ENCB6M_0EEs_0E0B1P_ENtB5_3Job7executeB6Q_+0xdc>
  68465e:	48 85 c0             	test   rax,rax
  684661:	74 35                	je     684698 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_ENCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2C_3zip11ZipProducerIB3K_INtNtNtB2E_5slice6chunks14ChunksProducerhEB4e_EIB4f_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2C_3map11MapConsumerINtNtB2C_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6M_s_0ENCB6M_0EEs_0E0B1P_ENtB5_3Job7executeB6Q_+0x108>
  684663:	4c 8b a3 80 00 00 00 	mov    r12,QWORD PTR [rbx+0x80]
  68466a:	eb 23                	jmp    68468f <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_ENCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2C_3zip11ZipProducerIB3K_INtNtNtB2E_5slice6chunks14ChunksProducerhEB4e_EIB4f_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2C_3map11MapConsumerINtNtB2C_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6M_s_0ENCB6M_0EEs_0E0B1P_ENtB5_3Job7executeB6Q_+0xff>
  68466c:	4c 8b a3 80 00 00 00 	mov    r12,QWORD PTR [rbx+0x80]
  684673:	48 8b ab 88 00 00 00 	mov    rbp,QWORD PTR [rbx+0x88]
  68467a:	48 8b 45 00          	mov    rax,QWORD PTR [rbp+0x0]
  68467e:	48 85 c0             	test   rax,rax
  684681:	74 05                	je     684688 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_ENCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2C_3zip11ZipProducerIB3K_INtNtNtB2E_5slice6chunks14ChunksProducerhEB4e_EIB4f_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2C_3map11MapConsumerINtNtB2C_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6M_s_0ENCB6M_0EEs_0E0B1P_ENtB5_3Job7executeB6Q_+0xf8>
  684683:	4c 89 e7             	mov    rdi,r12
  684686:	ff d0                	call   rax
  684688:	48 83 7d 08 00       	cmp    QWORD PTR [rbp+0x8],0x0
  68468d:	74 09                	je     684698 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_ENCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2C_3zip11ZipProducerIB3K_INtNtNtB2E_5slice6chunks14ChunksProducerhEB4e_EIB4f_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2C_3map11MapConsumerINtNtB2C_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6M_s_0ENCB6M_0EEs_0E0B1P_ENtB5_3Job7executeB6Q_+0x108>
  68468f:	4c 89 e7             	mov    rdi,r12
  684692:	ff 15 00 2d 08 00    	call   QWORD PTR [rip+0x82d00]        # 707398 <free@GLIBC_2.2.5>
  684698:	4c 89 6b 78          	mov    QWORD PTR [rbx+0x78],r13
  68469c:	4c 89 bb 80 00 00 00 	mov    QWORD PTR [rbx+0x80],r15
  6846a3:	4c 89 b3 88 00 00 00 	mov    QWORD PTR [rbx+0x88],r14
  6846aa:	0f b6 ab a8 00 00 00 	movzx  ebp,BYTE PTR [rbx+0xa8]
  6846b1:	40 84 ed             	test   bpl,bpl
  6846b4:	74 1b                	je     6846d1 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_ENCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2C_3zip11ZipProducerIB3K_INtNtNtB2E_5slice6chunks14ChunksProducerhEB4e_EIB4f_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2C_3map11MapConsumerINtNtB2C_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6M_s_0ENCB6M_0EEs_0E0B1P_ENtB5_3Job7executeB6Q_+0x141>
  6846b6:	48 8b 83 90 00 00 00 	mov    rax,QWORD PTR [rbx+0x90]
  6846bd:	48 8b 38             	mov    rdi,QWORD PTR [rax]
  6846c0:	f0 48 ff 07          	lock inc QWORD PTR [rdi]
  6846c4:	0f 8e d8 00 00 00    	jle    6847a2 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_ENCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2C_3zip11ZipProducerIB3K_INtNtNtB2E_5slice6chunks14ChunksProducerhEB4e_EIB4f_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2C_3map11MapConsumerINtNtB2C_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6M_s_0ENCB6M_0EEs_0E0B1P_ENtB5_3Job7executeB6Q_+0x212>
  6846ca:	48 89 7c 24 10       	mov    QWORD PTR [rsp+0x10],rdi
  6846cf:	eb 0a                	jmp    6846db <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_ENCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2C_3zip11ZipProducerIB3K_INtNtNtB2E_5slice6chunks14ChunksProducerhEB4e_EIB4f_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2C_3map11MapConsumerINtNtB2C_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6M_s_0ENCB6M_0EEs_0E0B1P_ENtB5_3Job7executeB6Q_+0x14b>
  6846d1:	48 8b 83 90 00 00 00 	mov    rax,QWORD PTR [rbx+0x90]
  6846d8:	48 8b 38             	mov    rdi,QWORD PTR [rax]
  6846db:	48 8b b3 a0 00 00 00 	mov    rsi,QWORD PTR [rbx+0xa0]
  6846e2:	b8 03 00 00 00       	mov    eax,0x3
  6846e7:	48 87 83 98 00 00 00 	xchg   QWORD PTR [rbx+0x98],rax
  6846ee:	48 83 f8 02          	cmp    rax,0x2
  6846f2:	75 0c                	jne    684700 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_ENCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2C_3zip11ZipProducerIB3K_INtNtNtB2E_5slice6chunks14ChunksProducerhEB4e_EIB4f_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2C_3map11MapConsumerINtNtB2C_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6M_s_0ENCB6M_0EEs_0E0B1P_ENtB5_3Job7executeB6Q_+0x170>
  6846f4:	48 81 c7 d8 01 00 00 	add    rdi,0x1d8
  6846fb:	e8 30 6d 00 00       	call   68b430 <_RNvMNtCsiKyBw8IgE1b_10rayon_core5sleepNtB2_5Sleep20wake_specific_thread.llvm.10825995516073413387>
  684700:	40 84 ed             	test   bpl,bpl
  684703:	74 16                	je     68471b <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_ENCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2C_3zip11ZipProducerIB3K_INtNtNtB2E_5slice6chunks14ChunksProducerhEB4e_EIB4f_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2C_3map11MapConsumerINtNtB2C_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6M_s_0ENCB6M_0EEs_0E0B1P_ENtB5_3Job7executeB6Q_+0x18b>
  684705:	48 8b 44 24 10       	mov    rax,QWORD PTR [rsp+0x10]
  68470a:	f0 48 ff 08          	lock dec QWORD PTR [rax]
  68470e:	75 0b                	jne    68471b <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_ENCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2C_3zip11ZipProducerIB3K_INtNtNtB2E_5slice6chunks14ChunksProducerhEB4e_EIB4f_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2C_3map11MapConsumerINtNtB2C_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6M_s_0ENCB6M_0EEs_0E0B1P_ENtB5_3Job7executeB6Q_+0x18b>
  684710:	48 8d 7c 24 10       	lea    rdi,[rsp+0x10]
  684715:	ff 15 f5 31 08 00    	call   QWORD PTR [rip+0x831f5]        # 707910 <_DYNAMIC+0x780>
  68471b:	48 81 c4 a8 00 00 00 	add    rsp,0xa8
  684722:	5b                   	pop    rbx
  684723:	41 5c                	pop    r12
  684725:	41 5d                	pop    r13
  684727:	41 5e                	pop    r14
  684729:	41 5f                	pop    r15
  68472b:	5d                   	pop    rbp
  68472c:	c3                   	ret    
  68472d:	48 8d 3d 4c 00 08 00 	lea    rdi,[rip+0x8004c]        # 704780 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x900>
  684734:	ff 15 f6 2d 08 00    	call   QWORD PTR [rip+0x82df6]        # 707530 <_DYNAMIC+0x3a0>
  68473a:	eb 66                	jmp    6847a2 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_ENCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2C_3zip11ZipProducerIB3K_INtNtNtB2E_5slice6chunks14ChunksProducerhEB4e_EIB4f_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2C_3map11MapConsumerINtNtB2C_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6M_s_0ENCB6M_0EEs_0E0B1P_ENtB5_3Job7executeB6Q_+0x212>
  68473c:	48 83 7d 08 00       	cmp    QWORD PTR [rbp+0x8],0x0
  684741:	74 09                	je     68474c <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_ENCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2C_3zip11ZipProducerIB3K_INtNtNtB2E_5slice6chunks14ChunksProducerhEB4e_EIB4f_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2C_3map11MapConsumerINtNtB2C_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6M_s_0ENCB6M_0EEs_0E0B1P_ENtB5_3Job7executeB6Q_+0x1bc>
  684743:	4c 89 e7             	mov    rdi,r12
  684746:	ff 15 4c 2c 08 00    	call   QWORD PTR [rip+0x82c4c]        # 707398 <free@GLIBC_2.2.5>
  68474c:	4c 89 6b 78          	mov    QWORD PTR [rbx+0x78],r13
  684750:	4c 89 bb 80 00 00 00 	mov    QWORD PTR [rbx+0x80],r15
  684757:	4c 89 b3 88 00 00 00 	mov    QWORD PTR [rbx+0x88],r14
  68475e:	eb 37                	jmp    684797 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_ENCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2C_3zip11ZipProducerIB3K_INtNtNtB2E_5slice6chunks14ChunksProducerhEB4e_EIB4f_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2C_3map11MapConsumerINtNtB2C_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6M_s_0ENCB6M_0EEs_0E0B1P_ENtB5_3Job7executeB6Q_+0x207>
  684760:	40 84 ed             	test   bpl,bpl
  684763:	74 32                	je     684797 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_ENCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2C_3zip11ZipProducerIB3K_INtNtNtB2E_5slice6chunks14ChunksProducerhEB4e_EIB4f_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2C_3map11MapConsumerINtNtB2C_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6M_s_0ENCB6M_0EEs_0E0B1P_ENtB5_3Job7executeB6Q_+0x207>
  684765:	48 8b 44 24 10       	mov    rax,QWORD PTR [rsp+0x10]
  68476a:	f0 48 ff 08          	lock dec QWORD PTR [rax]
  68476e:	75 27                	jne    684797 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_ENCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2C_3zip11ZipProducerIB3K_INtNtNtB2E_5slice6chunks14ChunksProducerhEB4e_EIB4f_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2C_3map11MapConsumerINtNtB2C_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6M_s_0ENCB6M_0EEs_0E0B1P_ENtB5_3Job7executeB6Q_+0x207>
  684770:	48 8d 7c 24 10       	lea    rdi,[rsp+0x10]
  684775:	ff 15 95 31 08 00    	call   QWORD PTR [rip+0x83195]        # 707910 <_DYNAMIC+0x780>
  68477b:	eb 1a                	jmp    684797 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_ENCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2C_3zip11ZipProducerIB3K_INtNtNtB2E_5slice6chunks14ChunksProducerhEB4e_EIB4f_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2C_3map11MapConsumerINtNtB2C_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6M_s_0ENCB6M_0EEs_0E0B1P_ENtB5_3Job7executeB6Q_+0x207>
  68477d:	ff 15 bd 2e 08 00    	call   QWORD PTR [rip+0x82ebd]        # 707640 <_DYNAMIC+0x4b0>
  684783:	48 89 c7             	mov    rdi,rax
  684786:	ff 15 44 30 08 00    	call   QWORD PTR [rip+0x83044]        # 7077d0 <_DYNAMIC+0x640>
  68478c:	49 89 c7             	mov    r15,rax
  68478f:	49 89 d6             	mov    r14,rdx
  684792:	e9 a9 fe ff ff       	jmp    684640 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_ENCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2C_3zip11ZipProducerIB3K_INtNtNtB2E_5slice6chunks14ChunksProducerhEB4e_EIB4f_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2C_3map11MapConsumerINtNtB2C_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6M_s_0ENCB6M_0EEs_0E0B1P_ENtB5_3Job7executeB6Q_+0xb0>
  684797:	48 8d 7c 24 0f       	lea    rdi,[rsp+0xf]
  68479c:	ff 15 66 31 08 00    	call   QWORD PTR [rip+0x83166]        # 707908 <_DYNAMIC+0x778>
  6847a2:	0f 0b                	ud2    
  6847a4:	ff 15 96 2e 08 00    	call   QWORD PTR [rip+0x82e96]        # 707640 <_DYNAMIC+0x4b0>
  6847aa:	cc                   	int3   
  6847ab:	cc                   	int3   
  6847ac:	cc                   	int3   
  6847ad:	cc                   	int3   
  6847ae:	cc                   	int3   
  6847af:	cc                   	int3   

