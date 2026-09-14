0000000000667cd0 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_>:
  667cd0:	55                   	push   rbp
  667cd1:	41 57                	push   r15
  667cd3:	41 56                	push   r14
  667cd5:	41 55                	push   r13
  667cd7:	41 54                	push   r12
  667cd9:	53                   	push   rbx
  667cda:	48 81 ec 08 02 00 00 	sub    rsp,0x208
  667ce1:	89 4c 24 10          	mov    DWORD PTR [rsp+0x10],ecx
  667ce5:	49 89 d7             	mov    r15,rdx
  667ce8:	49 89 f6             	mov    r14,rsi
  667ceb:	48 89 7c 24 18       	mov    QWORD PTR [rsp+0x18],rdi
  667cf0:	48 ba 00 00 00 00 00 	movabs rdx,0x8000000000000000
  667cf7:	00 00 80 
  667cfa:	49 8d 87 10 01 00 00 	lea    rax,[r15+0x110]
  667d01:	49 8b 8f 00 01 00 00 	mov    rcx,QWORD PTR [r15+0x100]
  667d08:	48 89 84 24 b0 00 00 	mov    QWORD PTR [rsp+0xb0],rax
  667d0f:	00 
  667d10:	48 c7 84 24 b8 00 00 	mov    QWORD PTR [rsp+0xb8],0x0
  667d17:	00 00 00 00 00 
  667d1c:	48 89 8c 24 c0 00 00 	mov    QWORD PTR [rsp+0xc0],rcx
  667d23:	00 
  667d24:	c6 84 24 c8 00 00 00 	mov    BYTE PTR [rsp+0xc8],0x0
  667d2b:	00 
  667d2c:	c5 fc 10 06          	vmovups ymm0,YMMWORD PTR [rsi]
  667d30:	c5 fc 10 4e 20       	vmovups ymm1,YMMWORD PTR [rsi+0x20]
  667d35:	c5 fc 10 56 40       	vmovups ymm2,YMMWORD PTR [rsi+0x40]
  667d3a:	c5 fc 10 5e 58       	vmovups ymm3,YMMWORD PTR [rsi+0x58]
  667d3f:	c5 fc 11 44 24 20    	vmovups YMMWORD PTR [rsp+0x20],ymm0
  667d45:	c5 fc 11 4c 24 40    	vmovups YMMWORD PTR [rsp+0x40],ymm1
  667d4b:	c5 fc 11 54 24 60    	vmovups YMMWORD PTR [rsp+0x60],ymm2
  667d51:	c5 fc 11 5c 24 78    	vmovups YMMWORD PTR [rsp+0x78],ymm3
  667d57:	48 89 94 24 98 00 00 	mov    QWORD PTR [rsp+0x98],rdx
  667d5e:	00 
  667d5f:	49 8b 87 18 01 00 00 	mov    rax,QWORD PTR [r15+0x118]
  667d66:	48 8b a8 08 01 00 00 	mov    rbp,QWORD PTR [rax+0x108]
  667d6d:	4c 8b a0 00 01 00 00 	mov    r12,QWORD PTR [rax+0x100]
  667d74:	49 8b 87 18 01 00 00 	mov    rax,QWORD PTR [r15+0x118]
  667d7b:	48 8b 98 08 01 00 00 	mov    rbx,QWORD PTR [rax+0x108]
  667d82:	48 8b 80 00 01 00 00 	mov    rax,QWORD PTR [rax+0x100]
  667d89:	49 8b b7 28 01 00 00 	mov    rsi,QWORD PTR [r15+0x128]
  667d90:	48 89 d9             	mov    rcx,rbx
  667d93:	48 29 c1             	sub    rcx,rax
  667d96:	48 39 f1             	cmp    rcx,rsi
  667d99:	0f 8d cb 03 00 00    	jge    66816a <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x49a>
  667d9f:	4c 29 e5             	sub    rbp,r12
  667da2:	48 b9 00 00 00 00 01 	movabs rcx,0x100000000
  667da9:	00 00 00 
  667dac:	49 8b 87 20 01 00 00 	mov    rax,QWORD PTR [r15+0x120]
  667db3:	48 ff ce             	dec    rsi
  667db6:	48 21 de             	and    rsi,rbx
  667db9:	48 c1 e6 04          	shl    rsi,0x4
  667dbd:	4c 8d 2d cc c7 01 00 	lea    r13,[rip+0x1c7cc]        # 684590 <_RNvXs2_NtCsiKyBw8IgE1b_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_ENCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2C_3zip11ZipProducerIB3K_INtNtNtB2E_5slice6chunks14ChunksProducerhEB4e_EIB4f_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB2C_3map11MapConsumerINtNtB2C_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB6M_s_0ENCB6M_0EEs_0E0B1P_ENtB5_3Job7executeB6Q_>
  667dc4:	4c 89 2c 30          	mov    QWORD PTR [rax+rsi*1],r13
  667dc8:	4c 8d 64 24 20       	lea    r12,[rsp+0x20]
  667dcd:	4c 89 64 30 08       	mov    QWORD PTR [rax+rsi*1+0x8],r12
  667dd2:	49 8b 87 18 01 00 00 	mov    rax,QWORD PTR [r15+0x118]
  667dd9:	48 ff c3             	inc    rbx
  667ddc:	48 89 98 08 01 00 00 	mov    QWORD PTR [rax+0x108],rbx
  667de3:	49 8b b7 10 01 00 00 	mov    rsi,QWORD PTR [r15+0x110]
  667dea:	48 8d be d8 01 00 00 	lea    rdi,[rsi+0x1d8]
  667df1:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  667df8:	0f 1f 84 00 00 00 00 
  667dff:	00 
  667e00:	48 8b 86 f0 01 00 00 	mov    rax,QWORD PTR [rsi+0x1f0]
  667e07:	48 85 c8             	test   rax,rcx
  667e0a:	75 1e                	jne    667e2a <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x15a>
  667e0c:	48 89 c2             	mov    rdx,rax
  667e0f:	48 09 ca             	or     rdx,rcx
  667e12:	f0 48 0f b1 96 f0 01 	lock cmpxchg QWORD PTR [rsi+0x1f0],rdx
  667e19:	00 00 
  667e1b:	75 e3                	jne    667e00 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x130>
  667e1d:	48 89 d0             	mov    rax,rdx
  667e20:	48 25 ff ff 00 00    	and    rax,0xffff
  667e26:	75 10                	jne    667e38 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x168>
  667e28:	eb 22                	jmp    667e4c <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x17c>
  667e2a:	48 89 c2             	mov    rdx,rax
  667e2d:	48 89 d0             	mov    rax,rdx
  667e30:	48 25 ff ff 00 00    	and    rax,0xffff
  667e36:	74 14                	je     667e4c <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x17c>
  667e38:	48 85 ed             	test   rbp,rbp
  667e3b:	0f 8f 16 03 00 00    	jg     668157 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x487>
  667e41:	c1 ea 10             	shr    edx,0x10
  667e44:	39 c2                	cmp    edx,eax
  667e46:	0f 84 0b 03 00 00    	je     668157 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x487>
  667e4c:	49 8b 46 78          	mov    rax,QWORD PTR [r14+0x78]
  667e50:	49 8b 96 80 00 00 00 	mov    rdx,QWORD PTR [r14+0x80]
  667e57:	49 8b be 88 00 00 00 	mov    rdi,QWORD PTR [r14+0x88]
  667e5e:	48 8b 30             	mov    rsi,QWORD PTR [rax]
  667e61:	48 8b 0a             	mov    rcx,QWORD PTR [rdx]
  667e64:	4c 8b 42 08          	mov    r8,QWORD PTR [rdx+0x8]
  667e68:	c4 c1 7c 10 86 90 00 	vmovups ymm0,YMMWORD PTR [r14+0x90]
  667e6f:	00 00 
  667e71:	c4 c1 7c 10 8e b0 00 	vmovups ymm1,YMMWORD PTR [r14+0xb0]
  667e78:	00 00 
  667e7a:	48 89 bc 24 50 01 00 	mov    QWORD PTR [rsp+0x150],rdi
  667e81:	00 
  667e82:	c5 fc 11 84 24 58 01 	vmovups YMMWORD PTR [rsp+0x158],ymm0
  667e89:	00 00 
  667e8b:	c5 fc 11 8c 24 78 01 	vmovups YMMWORD PTR [rsp+0x178],ymm1
  667e92:	00 00 
  667e94:	49 81 c6 d0 00 00 00 	add    r14,0xd0
  667e9b:	4c 89 34 24          	mov    QWORD PTR [rsp],r14
  667e9f:	0f b6 6c 24 10       	movzx  ebp,BYTE PTR [rsp+0x10]
  667ea4:	48 8d bc 24 00 01 00 	lea    rdi,[rsp+0x100]
  667eab:	00 
  667eac:	4c 8d 8c 24 50 01 00 	lea    r9,[rsp+0x150]
  667eb3:	00 
  667eb4:	89 ea                	mov    edx,ebp
  667eb6:	c5 f8 77             	vzeroupper 
  667eb9:	e8 f2 8f ff ff       	call   660eb0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_>
  667ebe:	48 8b 84 24 00 01 00 	mov    rax,QWORD PTR [rsp+0x100]
  667ec5:	00 
  667ec6:	48 89 44 24 10       	mov    QWORD PTR [rsp+0x10],rax
  667ecb:	48 8b 84 24 08 01 00 	mov    rax,QWORD PTR [rsp+0x108]
  667ed2:	00 
  667ed3:	48 89 44 24 08       	mov    QWORD PTR [rsp+0x8],rax
  667ed8:	4c 8b b4 24 10 01 00 	mov    r14,QWORD PTR [rsp+0x110]
  667edf:	00 
  667ee0:	48 8b 84 24 b8 00 00 	mov    rax,QWORD PTR [rsp+0xb8]
  667ee7:	00 
  667ee8:	48 83 f8 03          	cmp    rax,0x3
  667eec:	74 3b                	je     667f29 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x259>
  667eee:	4c 89 ff             	mov    rdi,r15
  667ef1:	e8 0a fe 00 00       	call   677d00 <_RNvMs8_NtCsiKyBw8IgE1b_10rayon_core8registryNtB5_12WorkerThread14take_local_job>
  667ef6:	48 85 c0             	test   rax,rax
  667ef9:	74 1c                	je     667f17 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x247>
  667efb:	4c 89 e1             	mov    rcx,r12
  667efe:	48 31 d1             	xor    rcx,rdx
  667f01:	48 89 c6             	mov    rsi,rax
  667f04:	4c 31 ee             	xor    rsi,r13
  667f07:	48 09 ce             	or     rsi,rcx
  667f0a:	0f 84 90 00 00 00    	je     667fa0 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x2d0>
  667f10:	48 89 d7             	mov    rdi,rdx
  667f13:	ff d0                	call   rax
  667f15:	eb c9                	jmp    667ee0 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x210>
  667f17:	48 8b 84 24 b8 00 00 	mov    rax,QWORD PTR [rsp+0xb8]
  667f1e:	00 
  667f1f:	48 83 f8 03          	cmp    rax,0x3
  667f23:	0f 85 7a 02 00 00    	jne    6681a3 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x4d3>
  667f29:	48 8b 84 24 98 00 00 	mov    rax,QWORD PTR [rsp+0x98]
  667f30:	00 
  667f31:	48 8b bc 24 a0 00 00 	mov    rdi,QWORD PTR [rsp+0xa0]
  667f38:	00 
  667f39:	48 8b b4 24 a8 00 00 	mov    rsi,QWORD PTR [rsp+0xa8]
  667f40:	00 
  667f41:	48 ba 00 00 00 00 00 	movabs rdx,0x8000000000000000
  667f48:	00 00 80 
  667f4b:	48 31 c2             	xor    rdx,rax
  667f4e:	48 85 c0             	test   rax,rax
  667f51:	b9 01 00 00 00       	mov    ecx,0x1
  667f56:	48 0f 48 ca          	cmovs  rcx,rdx
  667f5a:	48 83 f9 01          	cmp    rcx,0x1
  667f5e:	74 15                	je     667f75 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x2a5>
  667f60:	48 83 f9 02          	cmp    rcx,0x2
  667f64:	0f 85 1e 02 00 00    	jne    668188 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x4b8>
  667f6a:	ff 15 00 f8 09 00    	call   QWORD PTR [rip+0x9f800]        # 707770 <_DYNAMIC+0x5e0>
  667f70:	e9 8e 02 00 00       	jmp    668203 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x533>
  667f75:	48 8b 4c 24 18       	mov    rcx,QWORD PTR [rsp+0x18]
  667f7a:	48 8b 54 24 10       	mov    rdx,QWORD PTR [rsp+0x10]
  667f7f:	48 89 11             	mov    QWORD PTR [rcx],rdx
  667f82:	48 8b 54 24 08       	mov    rdx,QWORD PTR [rsp+0x8]
  667f87:	48 89 51 08          	mov    QWORD PTR [rcx+0x8],rdx
  667f8b:	4c 89 71 10          	mov    QWORD PTR [rcx+0x10],r14
  667f8f:	48 89 41 18          	mov    QWORD PTR [rcx+0x18],rax
  667f93:	48 89 79 20          	mov    QWORD PTR [rcx+0x20],rdi
  667f97:	48 89 71 28          	mov    QWORD PTR [rcx+0x28],rsi
  667f9b:	e9 a5 01 00 00       	jmp    668145 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x475>
  667fa0:	c5 fc 10 44 24 20    	vmovups ymm0,YMMWORD PTR [rsp+0x20]
  667fa6:	c5 fc 10 4c 24 40    	vmovups ymm1,YMMWORD PTR [rsp+0x40]
  667fac:	c5 fc 10 54 24 60    	vmovups ymm2,YMMWORD PTR [rsp+0x60]
  667fb2:	c5 fc 10 9c 24 80 00 	vmovups ymm3,YMMWORD PTR [rsp+0x80]
  667fb9:	00 00 
  667fbb:	c5 fc 11 84 24 50 01 	vmovups YMMWORD PTR [rsp+0x150],ymm0
  667fc2:	00 00 
  667fc4:	c5 fc 10 84 24 b0 00 	vmovups ymm0,YMMWORD PTR [rsp+0xb0]
  667fcb:	00 00 
  667fcd:	c5 fc 11 84 24 e0 01 	vmovups YMMWORD PTR [rsp+0x1e0],ymm0
  667fd4:	00 00 
  667fd6:	c5 fc 10 84 24 a0 00 	vmovups ymm0,YMMWORD PTR [rsp+0xa0]
  667fdd:	00 00 
  667fdf:	c5 fc 11 84 24 d0 01 	vmovups YMMWORD PTR [rsp+0x1d0],ymm0
  667fe6:	00 00 
  667fe8:	c5 fc 11 9c 24 b0 01 	vmovups YMMWORD PTR [rsp+0x1b0],ymm3
  667fef:	00 00 
  667ff1:	c5 fc 11 94 24 90 01 	vmovups YMMWORD PTR [rsp+0x190],ymm2
  667ff8:	00 00 
  667ffa:	c5 fc 11 8c 24 70 01 	vmovups YMMWORD PTR [rsp+0x170],ymm1
  668001:	00 00 
  668003:	48 8b 84 24 50 01 00 	mov    rax,QWORD PTR [rsp+0x150]
  66800a:	00 
  66800b:	48 85 c0             	test   rax,rax
  66800e:	0f 84 a5 01 00 00    	je     6681b9 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x4e9>
  668014:	48 8b 8c 24 58 01 00 	mov    rcx,QWORD PTR [rsp+0x158]
  66801b:	00 
  66801c:	48 8b 94 24 60 01 00 	mov    rdx,QWORD PTR [rsp+0x160]
  668023:	00 
  668024:	48 8b 74 24 78       	mov    rsi,QWORD PTR [rsp+0x78]
  668029:	c5 fc 10 44 24 38    	vmovups ymm0,YMMWORD PTR [rsp+0x38]
  66802f:	c5 fc 10 4c 24 58    	vmovups ymm1,YMMWORD PTR [rsp+0x58]
  668035:	c5 fc 11 8c 24 20 01 	vmovups YMMWORD PTR [rsp+0x120],ymm1
  66803c:	00 00 
  66803e:	c5 fc 11 84 24 00 01 	vmovups YMMWORD PTR [rsp+0x100],ymm0
  668045:	00 00 
  668047:	48 89 b4 24 40 01 00 	mov    QWORD PTR [rsp+0x140],rsi
  66804e:	00 
  66804f:	48 8b b4 24 90 00 00 	mov    rsi,QWORD PTR [rsp+0x90]
  668056:	00 
  668057:	c5 f8 28 84 24 80 00 	vmovaps xmm0,XMMWORD PTR [rsp+0x80]
  66805e:	00 00 
  668060:	c5 f8 29 84 24 d0 00 	vmovaps XMMWORD PTR [rsp+0xd0],xmm0
  668067:	00 00 
  668069:	48 89 b4 24 e0 00 00 	mov    QWORD PTR [rsp+0xe0],rsi
  668070:	00 
  668071:	48 8b 30             	mov    rsi,QWORD PTR [rax]
  668074:	48 2b 31             	sub    rsi,QWORD PTR [rcx]
  668077:	48 8b 0a             	mov    rcx,QWORD PTR [rdx]
  66807a:	4c 8b 42 08          	mov    r8,QWORD PTR [rdx+0x8]
  66807e:	48 8d 84 24 d0 00 00 	lea    rax,[rsp+0xd0]
  668085:	00 
  668086:	48 89 04 24          	mov    QWORD PTR [rsp],rax
  66808a:	48 8d bc 24 e8 00 00 	lea    rdi,[rsp+0xe8]
  668091:	00 
  668092:	4c 8d 8c 24 00 01 00 	lea    r9,[rsp+0x100]
  668099:	00 
  66809a:	89 ea                	mov    edx,ebp
  66809c:	c5 f8 77             	vzeroupper 
  66809f:	e8 0c 8e ff ff       	call   660eb0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_>
  6680a4:	48 8b 84 24 c8 01 00 	mov    rax,QWORD PTR [rsp+0x1c8]
  6680ab:	00 
  6680ac:	48 89 c2             	mov    rdx,rax
  6680af:	48 b9 00 00 00 00 00 	movabs rcx,0x8000000000000000
  6680b6:	00 00 80 
  6680b9:	48 31 ca             	xor    rdx,rcx
  6680bc:	48 85 c0             	test   rax,rax
  6680bf:	b9 01 00 00 00       	mov    ecx,0x1
  6680c4:	48 0f 48 ca          	cmovs  rcx,rdx
  6680c8:	48 85 c9             	test   rcx,rcx
  6680cb:	74 44                	je     668111 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x441>
  6680cd:	48 83 f9 01          	cmp    rcx,0x1
  6680d1:	75 0f                	jne    6680e2 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x412>
  6680d3:	48 85 c0             	test   rax,rax
  6680d6:	74 39                	je     668111 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x441>
  6680d8:	4c 8b bc 24 d0 01 00 	mov    r15,QWORD PTR [rsp+0x1d0]
  6680df:	00 
  6680e0:	eb 26                	jmp    668108 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x438>
  6680e2:	4c 8b bc 24 d0 01 00 	mov    r15,QWORD PTR [rsp+0x1d0]
  6680e9:	00 
  6680ea:	4c 8b a4 24 d8 01 00 	mov    r12,QWORD PTR [rsp+0x1d8]
  6680f1:	00 
  6680f2:	49 8b 04 24          	mov    rax,QWORD PTR [r12]
  6680f6:	48 85 c0             	test   rax,rax
  6680f9:	74 05                	je     668100 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x430>
  6680fb:	4c 89 ff             	mov    rdi,r15
  6680fe:	ff d0                	call   rax
  668100:	49 83 7c 24 08 00    	cmp    QWORD PTR [r12+0x8],0x0
  668106:	74 09                	je     668111 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x441>
  668108:	4c 89 ff             	mov    rdi,r15
  66810b:	ff 15 87 f2 09 00    	call   QWORD PTR [rip+0x9f287]        # 707398 <free@GLIBC_2.2.5>
  668111:	48 8b 4c 24 18       	mov    rcx,QWORD PTR [rsp+0x18]
  668116:	48 8b 44 24 10       	mov    rax,QWORD PTR [rsp+0x10]
  66811b:	48 89 01             	mov    QWORD PTR [rcx],rax
  66811e:	48 8b 44 24 08       	mov    rax,QWORD PTR [rsp+0x8]
  668123:	48 89 41 08          	mov    QWORD PTR [rcx+0x8],rax
  668127:	4c 89 71 10          	mov    QWORD PTR [rcx+0x10],r14
  66812b:	c5 f8 10 84 24 e8 00 	vmovups xmm0,XMMWORD PTR [rsp+0xe8]
  668132:	00 00 
  668134:	c5 f8 11 41 18       	vmovups XMMWORD PTR [rcx+0x18],xmm0
  668139:	48 8b 84 24 f8 00 00 	mov    rax,QWORD PTR [rsp+0xf8]
  668140:	00 
  668141:	48 89 41 28          	mov    QWORD PTR [rcx+0x28],rax
  668145:	48 81 c4 08 02 00 00 	add    rsp,0x208
  66814c:	5b                   	pop    rbx
  66814d:	41 5c                	pop    r12
  66814f:	41 5d                	pop    r13
  668151:	41 5e                	pop    r14
  668153:	41 5f                	pop    r15
  668155:	5d                   	pop    rbp
  668156:	c3                   	ret    
  668157:	be 01 00 00 00       	mov    esi,0x1
  66815c:	c5 f8 77             	vzeroupper 
  66815f:	ff 15 13 f6 09 00    	call   QWORD PTR [rip+0x9f613]        # 707778 <_DYNAMIC+0x5e8>
  668165:	e9 e2 fc ff ff       	jmp    667e4c <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x17c>
  66816a:	49 8d bf 18 01 00 00 	lea    rdi,[r15+0x118]
  668171:	48 01 f6             	add    rsi,rsi
  668174:	c5 f8 77             	vzeroupper 
  668177:	e8 34 f7 00 00       	call   6778b0 <_RNvMs4_NtCsh4KpTWVPfp2_15crossbeam_deque5dequeINtB5_6WorkerNtNtCsiKyBw8IgE1b_10rayon_core3job6JobRefE6resizeCs5Palesvl9wF_6mikros>
  66817c:	49 8b b7 28 01 00 00 	mov    rsi,QWORD PTR [r15+0x128]
  668183:	e9 17 fc ff ff       	jmp    667d9f <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0xcf>
  668188:	48 8d 3d 61 a7 9a ff 	lea    rdi,[rip+0xffffffffff9aa761]        # 128f0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x17a2>
  66818f:	48 8d 15 aa c0 09 00 	lea    rdx,[rip+0x9c0aa]        # 704240 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x3c0>
  668196:	be 28 00 00 00       	mov    esi,0x28
  66819b:	ff 15 37 f3 09 00    	call   QWORD PTR [rip+0x9f337]        # 7074d8 <_DYNAMIC+0x348>
  6681a1:	eb 60                	jmp    668203 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x533>
  6681a3:	4c 89 ff             	mov    rdi,r15
  6681a6:	48 8d b4 24 b8 00 00 	lea    rsi,[rsp+0xb8]
  6681ad:	00 
  6681ae:	ff 15 cc f5 09 00    	call   QWORD PTR [rip+0x9f5cc]        # 707780 <_DYNAMIC+0x5f0>
  6681b4:	e9 70 fd ff ff       	jmp    667f29 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x259>
  6681b9:	48 8d 3d 50 c0 09 00 	lea    rdi,[rip+0x9c050]        # 704210 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x390>
  6681c0:	c5 f8 77             	vzeroupper 
  6681c3:	ff 15 67 f3 09 00    	call   QWORD PTR [rip+0x9f367]        # 707530 <_DYNAMIC+0x3a0>
  6681c9:	eb 38                	jmp    668203 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x533>
  6681cb:	eb 3d                	jmp    66820a <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x53a>
  6681cd:	48 89 c3             	mov    rbx,rax
  6681d0:	49 83 7c 24 08 00    	cmp    QWORD PTR [r12+0x8],0x0
  6681d6:	74 4a                	je     668222 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x552>
  6681d8:	4c 89 ff             	mov    rdi,r15
  6681db:	ff 15 b7 f1 09 00    	call   QWORD PTR [rip+0x9f1b7]        # 707398 <free@GLIBC_2.2.5>
  6681e1:	eb 3f                	jmp    668222 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x552>
  6681e3:	48 89 c7             	mov    rdi,rax
  6681e6:	ff 15 e4 f5 09 00    	call   QWORD PTR [rip+0x9f5e4]        # 7077d0 <_DYNAMIC+0x640>
  6681ec:	48 89 d1             	mov    rcx,rdx
  6681ef:	4c 89 ff             	mov    rdi,r15
  6681f2:	48 8d b4 24 b0 00 00 	lea    rsi,[rsp+0xb0]
  6681f9:	00 
  6681fa:	48 89 c2             	mov    rdx,rax
  6681fd:	ff 15 d5 f5 09 00    	call   QWORD PTR [rip+0x9f5d5]        # 7077d8 <_DYNAMIC+0x648>
  668203:	0f 0b                	ud2    
  668205:	48 89 c3             	mov    rbx,rax
  668208:	eb 36                	jmp    668240 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x570>
  66820a:	48 89 c3             	mov    rbx,rax
  66820d:	40 b5 01             	mov    bpl,0x1
  668210:	eb 12                	jmp    668224 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x554>
  668212:	48 89 c3             	mov    rbx,rax
  668215:	48 8d bc 24 c8 01 00 	lea    rdi,[rsp+0x1c8]
  66821c:	00 
  66821d:	e8 fe 86 ff ff       	call   660920 <_RINvNtCsk9TLBDxQNFU_4core3ptr13drop_in_placeINtNtCsiKyBw8IgE1b_10rayon_core3job9JobResultINtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EEECs5Palesvl9wF_6mikros>
  668222:	31 ed                	xor    ebp,ebp
  668224:	48 83 7c 24 10 00    	cmp    QWORD PTR [rsp+0x10],0x0
  66822a:	74 0b                	je     668237 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x567>
  66822c:	48 8b 7c 24 08       	mov    rdi,QWORD PTR [rsp+0x8]
  668231:	ff 15 61 f1 09 00    	call   QWORD PTR [rip+0x9f161]        # 707398 <free@GLIBC_2.2.5>
  668237:	40 84 ed             	test   bpl,bpl
  66823a:	0f 84 a1 00 00 00    	je     6682e1 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x611>
  668240:	48 8b 84 24 98 00 00 	mov    rax,QWORD PTR [rsp+0x98]
  668247:	00 
  668248:	48 ba 00 00 00 00 00 	movabs rdx,0x8000000000000000
  66824f:	00 00 80 
  668252:	48 31 c2             	xor    rdx,rax
  668255:	48 85 c0             	test   rax,rax
  668258:	b9 01 00 00 00       	mov    ecx,0x1
  66825d:	48 0f 48 ca          	cmovs  rcx,rdx
  668261:	48 85 c9             	test   rcx,rcx
  668264:	74 7b                	je     6682e1 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x611>
  668266:	48 83 f9 01          	cmp    rcx,0x1
  66826a:	75 14                	jne    668280 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x5b0>
  66826c:	48 85 c0             	test   rax,rax
  66826f:	74 70                	je     6682e1 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x611>
  668271:	48 8b 84 24 a0 00 00 	mov    rax,QWORD PTR [rsp+0xa0]
  668278:	00 
  668279:	48 89 44 24 08       	mov    QWORD PTR [rsp+0x8],rax
  66827e:	eb 56                	jmp    6682d6 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x606>
  668280:	48 8b 84 24 a0 00 00 	mov    rax,QWORD PTR [rsp+0xa0]
  668287:	00 
  668288:	48 89 44 24 08       	mov    QWORD PTR [rsp+0x8],rax
  66828d:	4c 8b b4 24 a8 00 00 	mov    r14,QWORD PTR [rsp+0xa8]
  668294:	00 
  668295:	49 8b 06             	mov    rax,QWORD PTR [r14]
  668298:	48 85 c0             	test   rax,rax
  66829b:	74 07                	je     6682a4 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x5d4>
  66829d:	48 8b 7c 24 08       	mov    rdi,QWORD PTR [rsp+0x8]
  6682a2:	ff d0                	call   rax
  6682a4:	49 83 7e 08 00       	cmp    QWORD PTR [r14+0x8],0x0
  6682a9:	75 2b                	jne    6682d6 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x606>
  6682ab:	eb 34                	jmp    6682e1 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x611>
  6682ad:	49 83 7e 08 00       	cmp    QWORD PTR [r14+0x8],0x0
  6682b2:	74 0b                	je     6682bf <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x5ef>
  6682b4:	48 8b 7c 24 08       	mov    rdi,QWORD PTR [rsp+0x8]
  6682b9:	ff 15 d9 f0 09 00    	call   QWORD PTR [rip+0x9f0d9]        # 707398 <free@GLIBC_2.2.5>
  6682bf:	ff 15 7b f3 09 00    	call   QWORD PTR [rip+0x9f37b]        # 707640 <_DYNAMIC+0x4b0>
  6682c5:	ff 15 75 f3 09 00    	call   QWORD PTR [rip+0x9f375]        # 707640 <_DYNAMIC+0x4b0>
  6682cb:	48 89 c3             	mov    rbx,rax
  6682ce:	48 83 7c 24 10 00    	cmp    QWORD PTR [rsp+0x10],0x0
  6682d4:	74 0b                	je     6682e1 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_+0x611>
  6682d6:	48 8b 7c 24 08       	mov    rdi,QWORD PTR [rsp+0x8]
  6682db:	ff 15 b7 f0 09 00    	call   QWORD PTR [rip+0x9f0b7]        # 707398 <free@GLIBC_2.2.5>
  6682e1:	48 89 df             	mov    rdi,rbx
  6682e4:	e8 47 87 09 00       	call   700a30 <_Unwind_Resume@plt>
  6682e9:	cc                   	int3   
  6682ea:	cc                   	int3   
  6682eb:	cc                   	int3   
  6682ec:	cc                   	int3   
  6682ed:	cc                   	int3   
  6682ee:	cc                   	int3   
  6682ef:	cc                   	int3   

