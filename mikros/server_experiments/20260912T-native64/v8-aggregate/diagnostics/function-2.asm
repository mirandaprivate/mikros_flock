0000000000660eb0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_>:
  660eb0:	55                   	push   rbp
  660eb1:	41 57                	push   r15
  660eb3:	41 56                	push   r14
  660eb5:	41 55                	push   r13
  660eb7:	41 54                	push   r12
  660eb9:	53                   	push   rbx
  660eba:	48 81 ec 08 02 00 00 	sub    rsp,0x208
  660ec1:	4c 8b 94 24 40 02 00 	mov    r10,QWORD PTR [rsp+0x240]
  660ec8:	00 
  660ec9:	48 89 b4 24 88 00 00 	mov    QWORD PTR [rsp+0x88],rsi
  660ed0:	00 
  660ed1:	48 89 8c 24 90 00 00 	mov    QWORD PTR [rsp+0x90],rcx
  660ed8:	00 
  660ed9:	4c 89 84 24 98 00 00 	mov    QWORD PTR [rsp+0x98],r8
  660ee0:	00 
  660ee1:	48 d1 ee             	shr    rsi,1
  660ee4:	4c 39 c6             	cmp    rsi,r8
  660ee7:	0f 83 a0 00 00 00    	jae    660f8d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0xdd>
  660eed:	49 89 fe             	mov    r14,rdi
  660ef0:	49 8b 01             	mov    rax,QWORD PTR [r9]
  660ef3:	48 89 44 24 28       	mov    QWORD PTR [rsp+0x28],rax
  660ef8:	49 8b 59 08          	mov    rbx,QWORD PTR [r9+0x8]
  660efc:	4d 8b 79 10          	mov    r15,QWORD PTR [r9+0x10]
  660f00:	49 8b 41 18          	mov    rax,QWORD PTR [r9+0x18]
  660f04:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
  660f09:	4d 8b 61 20          	mov    r12,QWORD PTR [r9+0x20]
  660f0d:	4d 8b 69 28          	mov    r13,QWORD PTR [r9+0x28]
  660f11:	49 8b 41 30          	mov    rax,QWORD PTR [r9+0x30]
  660f15:	48 89 44 24 18       	mov    QWORD PTR [rsp+0x18],rax
  660f1a:	49 8b 69 38          	mov    rbp,QWORD PTR [r9+0x38]
  660f1e:	49 8b 41 40          	mov    rax,QWORD PTR [r9+0x40]
  660f22:	48 89 44 24 30       	mov    QWORD PTR [rsp+0x30],rax
  660f27:	49 8b 42 10          	mov    rax,QWORD PTR [r10+0x10]
  660f2b:	48 89 44 24 10       	mov    QWORD PTR [rsp+0x10],rax
  660f30:	bf f0 0f 00 00       	mov    edi,0xff0
  660f35:	be 01 00 00 00       	mov    esi,0x1
  660f3a:	ff 15 70 64 0a 00    	call   QWORD PTR [rip+0xa6470]        # 7073b0 <calloc@GLIBC_2.2.5>
  660f40:	48 89 44 24 08       	mov    QWORD PTR [rsp+0x8],rax
  660f45:	48 85 c0             	test   rax,rax
  660f48:	0f 84 8e 05 00 00    	je     6614dc <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x62c>
  660f4e:	4d 85 ff             	test   r15,r15
  660f51:	0f 84 34 05 00 00    	je     66148b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x5db>
  660f57:	4d 89 f8             	mov    r8,r15
  660f5a:	4d 89 ef             	mov    r15,r13
  660f5d:	4d 85 ed             	test   r13,r13
  660f60:	0f 84 25 05 00 00    	je     66148b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x5db>
  660f66:	48 89 df             	mov    rdi,rbx
  660f69:	4d 89 e1             	mov    r9,r12
  660f6c:	49 89 eb             	mov    r11,rbp
  660f6f:	48 85 db             	test   rbx,rbx
  660f72:	74 53                	je     660fc7 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x117>
  660f74:	48 89 f8             	mov    rax,rdi
  660f77:	4c 09 c0             	or     rax,r8
  660f7a:	48 c1 e8 20          	shr    rax,0x20
  660f7e:	74 52                	je     660fd2 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x122>
  660f80:	48 89 f8             	mov    rax,rdi
  660f83:	31 d2                	xor    edx,edx
  660f85:	49 f7 f0             	div    r8
  660f88:	48 89 c6             	mov    rsi,rax
  660f8b:	eb 4e                	jmp    660fdb <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x12b>
  660f8d:	84 d2                	test   dl,dl
  660f8f:	74 1f                	je     660fb0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x100>
  660f91:	48 c7 c0 a8 ff ff ff 	mov    rax,0xffffffffffffffa8
  660f98:	64 48 8b 00          	mov    rax,QWORD PTR fs:[rax]
  660f9c:	48 85 c0             	test   rax,rax
  660f9f:	0f 84 7f 02 00 00    	je     661224 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x374>
  660fa5:	48 05 10 01 00 00    	add    rax,0x110
  660fab:	e9 98 02 00 00       	jmp    661248 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x398>
  660fb0:	48 85 c9             	test   rcx,rcx
  660fb3:	0f 84 34 ff ff ff    	je     660eed <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3d>
  660fb9:	4d 89 d6             	mov    r14,r10
  660fbc:	48 89 fd             	mov    rbp,rdi
  660fbf:	48 d1 e9             	shr    rcx,1
  660fc2:	e9 9b 02 00 00       	jmp    661262 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3b2>
  660fc7:	31 f6                	xor    esi,esi
  660fc9:	4d 85 c9             	test   r9,r9
  660fcc:	75 1a                	jne    660fe8 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x138>
  660fce:	31 c9                	xor    ecx,ecx
  660fd0:	eb 48                	jmp    66101a <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x16a>
  660fd2:	89 f8                	mov    eax,edi
  660fd4:	31 d2                	xor    edx,edx
  660fd6:	41 f7 f0             	div    r8d
  660fd9:	89 c6                	mov    esi,eax
  660fdb:	48 83 fa 01          	cmp    rdx,0x1
  660fdf:	48 83 de ff          	sbb    rsi,0xffffffffffffffff
  660fe3:	4d 85 c9             	test   r9,r9
  660fe6:	74 e6                	je     660fce <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x11e>
  660fe8:	4c 89 c8             	mov    rax,r9
  660feb:	4c 09 f8             	or     rax,r15
  660fee:	48 c1 e8 20          	shr    rax,0x20
  660ff2:	74 0d                	je     661001 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x151>
  660ff4:	4c 89 c8             	mov    rax,r9
  660ff7:	31 d2                	xor    edx,edx
  660ff9:	49 f7 f7             	div    r15
  660ffc:	48 89 c1             	mov    rcx,rax
  660fff:	eb 0a                	jmp    66100b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x15b>
  661001:	44 89 c8             	mov    eax,r9d
  661004:	31 d2                	xor    edx,edx
  661006:	41 f7 f7             	div    r15d
  661009:	89 c1                	mov    ecx,eax
  66100b:	48 83 fa 01          	cmp    rdx,0x1
  66100f:	48 83 d9 ff          	sbb    rcx,0xffffffffffffffff
  661013:	48 39 f1             	cmp    rcx,rsi
  661016:	48 0f 43 ce          	cmovae rcx,rsi
  66101a:	48 8b 74 24 30       	mov    rsi,QWORD PTR [rsp+0x30]
  66101f:	48 85 f6             	test   rsi,rsi
  661022:	0f 84 63 04 00 00    	je     66148b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x5db>
  661028:	4c 89 74 24 68       	mov    QWORD PTR [rsp+0x68],r14
  66102d:	ba ff 00 00 00       	mov    edx,0xff
  661032:	4d 85 db             	test   r11,r11
  661035:	74 19                	je     661050 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1a0>
  661037:	4c 89 d8             	mov    rax,r11
  66103a:	48 09 f0             	or     rax,rsi
  66103d:	48 c1 e8 20          	shr    rax,0x20
  661041:	74 17                	je     66105a <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1aa>
  661043:	4c 89 d8             	mov    rax,r11
  661046:	31 d2                	xor    edx,edx
  661048:	48 f7 f6             	div    rsi
  66104b:	48 89 c5             	mov    rbp,rax
  66104e:	eb 13                	jmp    661063 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1b3>
  661050:	b9 ff 00 00 00       	mov    ecx,0xff
  661055:	e9 b0 01 00 00       	jmp    66120a <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x35a>
  66105a:	44 89 d8             	mov    eax,r11d
  66105d:	31 d2                	xor    edx,edx
  66105f:	f7 f6                	div    esi
  661061:	89 c5                	mov    ebp,eax
  661063:	48 83 fa 01          	cmp    rdx,0x1
  661067:	48 83 dd ff          	sbb    rbp,0xffffffffffffffff
  66106b:	48 39 cd             	cmp    rbp,rcx
  66106e:	48 0f 43 e9          	cmovae rbp,rcx
  661072:	48 85 ed             	test   rbp,rbp
  661075:	0f 84 85 01 00 00    	je     661200 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x350>
  66107b:	48 8b 44 24 10       	mov    rax,QWORD PTR [rsp+0x10]
  661080:	48 8b 00             	mov    rax,QWORD PTR [rax]
  661083:	48 89 84 24 80 00 00 	mov    QWORD PTR [rsp+0x80],rax
  66108a:	00 
  66108b:	48 89 f0             	mov    rax,rsi
  66108e:	48 c1 e0 04          	shl    rax,0x4
  661092:	48 89 44 24 78       	mov    QWORD PTR [rsp+0x78],rax
  661097:	b9 ff 00 00 00       	mov    ecx,0xff
  66109c:	ba ff 00 00 00       	mov    edx,0xff
  6610a1:	4c 8b 64 24 28       	mov    r12,QWORD PTR [rsp+0x28]
  6610a6:	48 8b 5c 24 20       	mov    rbx,QWORD PTR [rsp+0x20]
  6610ab:	4c 8b 6c 24 18       	mov    r13,QWORD PTR [rsp+0x18]
  6610b0:	4c 89 44 24 70       	mov    QWORD PTR [rsp+0x70],r8
  6610b5:	66 66 2e 0f 1f 84 00 	data16 cs nop WORD PTR [rax+rax*1+0x0]
  6610bc:	00 00 00 00 
  6610c0:	4c 89 4c 24 10       	mov    QWORD PTR [rsp+0x10],r9
  6610c5:	48 89 7c 24 18       	mov    QWORD PTR [rsp+0x18],rdi
  6610ca:	48 89 6c 24 28       	mov    QWORD PTR [rsp+0x28],rbp
  6610cf:	49 39 f8             	cmp    r8,rdi
  6610d2:	49 0f 42 f8          	cmovb  rdi,r8
  6610d6:	4d 39 cf             	cmp    r15,r9
  6610d9:	48 89 94 24 a0 00 00 	mov    QWORD PTR [rsp+0xa0],rdx
  6610e0:	00 
  6610e1:	4c 8b 74 24 08       	mov    r14,QWORD PTR [rsp+0x8]
  6610e6:	4c 89 b4 24 a8 00 00 	mov    QWORD PTR [rsp+0xa8],r14
  6610ed:	00 
  6610ee:	48 89 4c 24 20       	mov    QWORD PTR [rsp+0x20],rcx
  6610f3:	48 89 8c 24 b0 00 00 	mov    QWORD PTR [rsp+0xb0],rcx
  6610fa:	00 
  6610fb:	4c 89 a4 24 b8 00 00 	mov    QWORD PTR [rsp+0xb8],r12
  661102:	00 
  661103:	48 89 bc 24 c0 00 00 	mov    QWORD PTR [rsp+0xc0],rdi
  66110a:	00 
  66110b:	4d 0f 42 cf          	cmovb  r9,r15
  66110f:	4c 39 de             	cmp    rsi,r11
  661112:	48 89 9c 24 c8 00 00 	mov    QWORD PTR [rsp+0xc8],rbx
  661119:	00 
  66111a:	4c 89 8c 24 d0 00 00 	mov    QWORD PTR [rsp+0xd0],r9
  661121:	00 
  661122:	4c 89 d8             	mov    rax,r11
  661125:	48 0f 42 c6          	cmovb  rax,rsi
  661129:	4c 89 ac 24 d8 00 00 	mov    QWORD PTR [rsp+0xd8],r13
  661130:	00 
  661131:	48 89 84 24 e0 00 00 	mov    QWORD PTR [rsp+0xe0],rax
  661138:	00 
  661139:	48 8b 84 24 80 00 00 	mov    rax,QWORD PTR [rsp+0x80]
  661140:	00 
  661141:	0f b6 30             	movzx  esi,BYTE PTR [rax]
  661144:	48 8d bc 24 f0 00 00 	lea    rdi,[rsp+0xf0]
  66114b:	00 
  66114c:	48 89 d5             	mov    rbp,rdx
  66114f:	48 8d 94 24 b8 00 00 	lea    rdx,[rsp+0xb8]
  661156:	00 
  661157:	4d 89 de             	mov    r14,r11
  66115a:	e8 b1 93 00 00       	call   66a510 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_>
  66115f:	48 8b 84 24 00 01 00 	mov    rax,QWORD PTR [rsp+0x100]
  661166:	00 
  661167:	c5 f8 10 84 24 f0 00 	vmovups xmm0,XMMWORD PTR [rsp+0xf0]
  66116e:	00 00 
  661170:	48 8d 54 24 50       	lea    rdx,[rsp+0x50]
  661175:	48 89 42 10          	mov    QWORD PTR [rdx+0x10],rax
  661179:	c5 f8 11 02          	vmovups XMMWORD PTR [rdx],xmm0
  66117d:	48 89 6c 24 38       	mov    QWORD PTR [rsp+0x38],rbp
  661182:	48 8b 44 24 08       	mov    rax,QWORD PTR [rsp+0x8]
  661187:	48 89 44 24 40       	mov    QWORD PTR [rsp+0x40],rax
  66118c:	48 8b 44 24 20       	mov    rax,QWORD PTR [rsp+0x20]
  661191:	48 89 44 24 48       	mov    QWORD PTR [rsp+0x48],rax
  661196:	48 8d bc 24 08 01 00 	lea    rdi,[rsp+0x108]
  66119d:	00 
  66119e:	48 8d 74 24 38       	lea    rsi,[rsp+0x38]
  6611a3:	e8 58 f1 00 00       	call   670300 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0B5_>
  6611a8:	48 8b 94 24 08 01 00 	mov    rdx,QWORD PTR [rsp+0x108]
  6611af:	00 
  6611b0:	48 8b 84 24 10 01 00 	mov    rax,QWORD PTR [rsp+0x110]
  6611b7:	00 
  6611b8:	48 89 44 24 08       	mov    QWORD PTR [rsp+0x8],rax
  6611bd:	48 8b 8c 24 18 01 00 	mov    rcx,QWORD PTR [rsp+0x118]
  6611c4:	00 
  6611c5:	4c 03 6c 24 78       	add    r13,QWORD PTR [rsp+0x78]
  6611ca:	48 8b 74 24 30       	mov    rsi,QWORD PTR [rsp+0x30]
  6611cf:	49 29 f6             	sub    r14,rsi
  6611d2:	4c 01 fb             	add    rbx,r15
  6611d5:	4c 8b 4c 24 10       	mov    r9,QWORD PTR [rsp+0x10]
  6611da:	4d 29 f9             	sub    r9,r15
  6611dd:	4c 8b 44 24 70       	mov    r8,QWORD PTR [rsp+0x70]
  6611e2:	4d 01 c4             	add    r12,r8
  6611e5:	48 8b 7c 24 18       	mov    rdi,QWORD PTR [rsp+0x18]
  6611ea:	4c 29 c7             	sub    rdi,r8
  6611ed:	48 8b 6c 24 28       	mov    rbp,QWORD PTR [rsp+0x28]
  6611f2:	48 ff cd             	dec    rbp
  6611f5:	4d 89 f3             	mov    r11,r14
  6611f8:	0f 85 c2 fe ff ff    	jne    6610c0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x210>
  6611fe:	eb 0a                	jmp    66120a <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x35a>
  661200:	b9 ff 00 00 00       	mov    ecx,0xff
  661205:	ba ff 00 00 00       	mov    edx,0xff
  66120a:	48 8b 44 24 68       	mov    rax,QWORD PTR [rsp+0x68]
  66120f:	48 89 10             	mov    QWORD PTR [rax],rdx
  661212:	48 8b 54 24 08       	mov    rdx,QWORD PTR [rsp+0x8]
  661217:	48 89 50 08          	mov    QWORD PTR [rax+0x8],rdx
  66121b:	48 89 48 10          	mov    QWORD PTR [rax+0x10],rcx
  66121f:	e9 55 02 00 00       	jmp    661479 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x5c9>
  661224:	48 89 f3             	mov    rbx,rsi
  661227:	4d 89 cc             	mov    r12,r9
  66122a:	49 89 cf             	mov    r15,rcx
  66122d:	49 89 fe             	mov    r14,rdi
  661230:	4d 89 d5             	mov    r13,r10
  661233:	ff 15 1f 65 0a 00    	call   QWORD PTR [rip+0xa651f]        # 707758 <_DYNAMIC+0x5c8>
  661239:	4d 89 ea             	mov    r10,r13
  66123c:	4c 89 f7             	mov    rdi,r14
  66123f:	4c 89 f9             	mov    rcx,r15
  661242:	4d 89 e1             	mov    r9,r12
  661245:	48 89 de             	mov    rsi,rbx
  661248:	4d 89 d6             	mov    r14,r10
  66124b:	48 89 fd             	mov    rbp,rdi
  66124e:	48 8b 00             	mov    rax,QWORD PTR [rax]
  661251:	48 8b 80 08 02 00 00 	mov    rax,QWORD PTR [rax+0x208]
  661258:	48 d1 e9             	shr    rcx,1
  66125b:	48 39 c1             	cmp    rcx,rax
  66125e:	48 0f 46 c8          	cmovbe rcx,rax
  661262:	48 89 8c 24 90 00 00 	mov    QWORD PTR [rsp+0x90],rcx
  661269:	00 
  66126a:	48 89 b4 24 f0 00 00 	mov    QWORD PTR [rsp+0xf0],rsi
  661271:	00 
  661272:	49 8b 01             	mov    rax,QWORD PTR [r9]
  661275:	49 8b 59 08          	mov    rbx,QWORD PTR [r9+0x8]
  661279:	49 8b 49 10          	mov    rcx,QWORD PTR [r9+0x10]
  66127d:	49 8b 51 18          	mov    rdx,QWORD PTR [r9+0x18]
  661281:	4d 8b 59 20          	mov    r11,QWORD PTR [r9+0x20]
  661285:	49 8b 79 28          	mov    rdi,QWORD PTR [r9+0x28]
  661289:	4d 8b 41 30          	mov    r8,QWORD PTR [r9+0x30]
  66128d:	49 89 ca             	mov    r10,rcx
  661290:	4c 0f af d6          	imul   r10,rsi
  661294:	4c 39 d3             	cmp    rbx,r10
  661297:	4c 0f 42 d3          	cmovb  r10,rbx
  66129b:	4e 8d 3c 10          	lea    r15,[rax+r10*1]
  66129f:	4c 29 d3             	sub    rbx,r10
  6612a2:	4c 8d a4 24 88 00 00 	lea    r12,[rsp+0x88]
  6612a9:	00 
  6612aa:	4c 89 a4 24 20 01 00 	mov    QWORD PTR [rsp+0x120],r12
  6612b1:	00 
  6612b2:	4c 8d a4 24 f0 00 00 	lea    r12,[rsp+0xf0]
  6612b9:	00 
  6612ba:	4c 89 a4 24 28 01 00 	mov    QWORD PTR [rsp+0x128],r12
  6612c1:	00 
  6612c2:	4c 8d ac 24 90 00 00 	lea    r13,[rsp+0x90]
  6612c9:	00 
  6612ca:	4c 89 ac 24 30 01 00 	mov    QWORD PTR [rsp+0x130],r13
  6612d1:	00 
  6612d2:	4c 89 bc 24 38 01 00 	mov    QWORD PTR [rsp+0x138],r15
  6612d9:	00 
  6612da:	49 89 ff             	mov    r15,rdi
  6612dd:	4c 0f af fe          	imul   r15,rsi
  6612e1:	4d 39 fb             	cmp    r11,r15
  6612e4:	4d 0f 42 fb          	cmovb  r15,r11
  6612e8:	48 89 9c 24 40 01 00 	mov    QWORD PTR [rsp+0x140],rbx
  6612ef:	00 
  6612f0:	4a 8d 1c 3a          	lea    rbx,[rdx+r15*1]
  6612f4:	48 89 8c 24 48 01 00 	mov    QWORD PTR [rsp+0x148],rcx
  6612fb:	00 
  6612fc:	48 89 9c 24 50 01 00 	mov    QWORD PTR [rsp+0x150],rbx
  661303:	00 
  661304:	49 8b 59 38          	mov    rbx,QWORD PTR [r9+0x38]
  661308:	4d 8b 49 40          	mov    r9,QWORD PTR [r9+0x40]
  66130c:	4d 29 fb             	sub    r11,r15
  66130f:	49 0f af f1          	imul   rsi,r9
  661313:	48 39 f3             	cmp    rbx,rsi
  661316:	48 0f 42 f3          	cmovb  rsi,rbx
  66131a:	4c 89 9c 24 58 01 00 	mov    QWORD PTR [rsp+0x158],r11
  661321:	00 
  661322:	49 89 f3             	mov    r11,rsi
  661325:	49 c1 e3 04          	shl    r11,0x4
  661329:	4d 01 c3             	add    r11,r8
  66132c:	48 29 f3             	sub    rbx,rsi
  66132f:	48 89 bc 24 60 01 00 	mov    QWORD PTR [rsp+0x160],rdi
  661336:	00 
  661337:	4c 89 9c 24 68 01 00 	mov    QWORD PTR [rsp+0x168],r11
  66133e:	00 
  66133f:	4d 8b 1e             	mov    r11,QWORD PTR [r14]
  661342:	48 89 9c 24 70 01 00 	mov    QWORD PTR [rsp+0x170],rbx
  661349:	00 
  66134a:	49 8b 5e 08          	mov    rbx,QWORD PTR [r14+0x8]
  66134e:	4d 8b 76 10          	mov    r14,QWORD PTR [r14+0x10]
  661352:	4c 89 8c 24 78 01 00 	mov    QWORD PTR [rsp+0x178],r9
  661359:	00 
  66135a:	4c 89 9c 24 80 01 00 	mov    QWORD PTR [rsp+0x180],r11
  661361:	00 
  661362:	48 89 9c 24 88 01 00 	mov    QWORD PTR [rsp+0x188],rbx
  661369:	00 
  66136a:	4c 89 b4 24 90 01 00 	mov    QWORD PTR [rsp+0x190],r14
  661371:	00 
  661372:	4c 89 a4 24 98 01 00 	mov    QWORD PTR [rsp+0x198],r12
  661379:	00 
  66137a:	4c 89 ac 24 a0 01 00 	mov    QWORD PTR [rsp+0x1a0],r13
  661381:	00 
  661382:	48 89 84 24 a8 01 00 	mov    QWORD PTR [rsp+0x1a8],rax
  661389:	00 
  66138a:	4c 89 94 24 b0 01 00 	mov    QWORD PTR [rsp+0x1b0],r10
  661391:	00 
  661392:	48 89 8c 24 b8 01 00 	mov    QWORD PTR [rsp+0x1b8],rcx
  661399:	00 
  66139a:	48 89 94 24 c0 01 00 	mov    QWORD PTR [rsp+0x1c0],rdx
  6613a1:	00 
  6613a2:	4c 89 bc 24 c8 01 00 	mov    QWORD PTR [rsp+0x1c8],r15
  6613a9:	00 
  6613aa:	48 89 bc 24 d0 01 00 	mov    QWORD PTR [rsp+0x1d0],rdi
  6613b1:	00 
  6613b2:	4c 89 84 24 d8 01 00 	mov    QWORD PTR [rsp+0x1d8],r8
  6613b9:	00 
  6613ba:	48 89 b4 24 e0 01 00 	mov    QWORD PTR [rsp+0x1e0],rsi
  6613c1:	00 
  6613c2:	4c 89 8c 24 e8 01 00 	mov    QWORD PTR [rsp+0x1e8],r9
  6613c9:	00 
  6613ca:	4c 89 9c 24 f0 01 00 	mov    QWORD PTR [rsp+0x1f0],r11
  6613d1:	00 
  6613d2:	48 89 9c 24 f8 01 00 	mov    QWORD PTR [rsp+0x1f8],rbx
  6613d9:	00 
  6613da:	4c 89 b4 24 00 02 00 	mov    QWORD PTR [rsp+0x200],r14
  6613e1:	00 
  6613e2:	48 c7 c3 a8 ff ff ff 	mov    rbx,0xffffffffffffffa8
  6613e9:	64 48 8b 13          	mov    rdx,QWORD PTR fs:[rbx]
  6613ed:	48 85 d2             	test   rdx,rdx
  6613f0:	75 23                	jne    661415 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x565>
  6613f2:	ff 15 60 63 0a 00    	call   QWORD PTR [rip+0xa6360]        # 707758 <_DYNAMIC+0x5c8>
  6613f8:	48 8b 30             	mov    rsi,QWORD PTR [rax]
  6613fb:	64 48 8b 13          	mov    rdx,QWORD PTR fs:[rbx]
  6613ff:	48 85 d2             	test   rdx,rdx
  661402:	0f 84 9e 00 00 00    	je     6614a6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x5f6>
  661408:	48 39 b2 10 01 00 00 	cmp    QWORD PTR [rdx+0x110],rsi
  66140f:	0f 85 ac 00 00 00    	jne    6614c1 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x611>
  661415:	48 8d 7c 24 38       	lea    rdi,[rsp+0x38]
  66141a:	48 8d b4 24 20 01 00 	lea    rsi,[rsp+0x120]
  661421:	00 
  661422:	31 c9                	xor    ecx,ecx
  661424:	e8 a7 68 00 00       	call   667cd0 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_>
  661429:	48 8b 44 24 48       	mov    rax,QWORD PTR [rsp+0x48]
  66142e:	48 89 84 24 b0 00 00 	mov    QWORD PTR [rsp+0xb0],rax
  661435:	00 
  661436:	c5 f8 10 44 24 38    	vmovups xmm0,XMMWORD PTR [rsp+0x38]
  66143c:	c5 f8 29 84 24 a0 00 	vmovaps XMMWORD PTR [rsp+0xa0],xmm0
  661443:	00 00 
  661445:	48 8d 94 24 b8 00 00 	lea    rdx,[rsp+0xb8]
  66144c:	00 
  66144d:	c5 f8 10 44 24 50    	vmovups xmm0,XMMWORD PTR [rsp+0x50]
  661453:	c5 f8 11 84 24 b8 00 	vmovups XMMWORD PTR [rsp+0xb8],xmm0
  66145a:	00 00 
  66145c:	48 8b 44 24 60       	mov    rax,QWORD PTR [rsp+0x60]
  661461:	48 89 84 24 c8 00 00 	mov    QWORD PTR [rsp+0xc8],rax
  661468:	00 
  661469:	48 8d b4 24 a0 00 00 	lea    rsi,[rsp+0xa0]
  661470:	00 
  661471:	48 89 ef             	mov    rdi,rbp
  661474:	e8 87 ee 00 00       	call   670300 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0B5_>
  661479:	48 81 c4 08 02 00 00 	add    rsp,0x208
  661480:	5b                   	pop    rbx
  661481:	41 5c                	pop    r12
  661483:	41 5d                	pop    r13
  661485:	41 5e                	pop    r14
  661487:	41 5f                	pop    r15
  661489:	5d                   	pop    rbp
  66148a:	c3                   	ret    
  66148b:	48 8d 3d 06 15 9b ff 	lea    rdi,[rip+0xffffffffff9b1506]        # 12998 <anon.0e5291c933a25680de2bf032d375ada2.80.llvm.8267282299410697290+0x1>
  661492:	48 8d 15 ff 32 0a 00 	lea    rdx,[rip+0xa32ff]        # 704798 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x918>
  661499:	be 37 00 00 00       	mov    esi,0x37
  66149e:	ff 15 a4 5f 0a 00    	call   QWORD PTR [rip+0xa5fa4]        # 707448 <_DYNAMIC+0x2b8>
  6614a4:	0f 0b                	ud2    
  6614a6:	48 83 ee 80          	sub    rsi,0xffffffffffffff80
  6614aa:	48 8d 7c 24 38       	lea    rdi,[rsp+0x38]
  6614af:	48 8d 94 24 20 01 00 	lea    rdx,[rsp+0x120]
  6614b6:	00 
  6614b7:	e8 14 cd ff ff       	call   65e1d0 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_3zip11ZipProducerIB2V_INtNtNtB1P_5slice6chunks14ChunksProducerhEB3p_EIB3q_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5X_s_0ENCB5X_0EE0NCB1G_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7f_E0TB7f_B7f_EEB61_>
  6614bc:	e9 68 ff ff ff       	jmp    661429 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x579>
  6614c1:	48 83 ee 80          	sub    rsi,0xffffffffffffff80
  6614c5:	48 8d 7c 24 38       	lea    rdi,[rsp+0x38]
  6614ca:	48 8d 8c 24 20 01 00 	lea    rcx,[rsp+0x120]
  6614d1:	00 
  6614d2:	e8 69 db ff ff       	call   65f040 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_3zip11ZipProducerIB2W_INtNtNtB1Q_5slice6chunks14ChunksProducerhEB3q_EIB3r_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5Y_s_0ENCB5Y_0EE0NCB1H_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7g_E0TB7g_B7g_EEB62_>
  6614d7:	e9 4d ff ff ff       	jmp    661429 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x579>
  6614dc:	bf 01 00 00 00       	mov    edi,0x1
  6614e1:	be f0 0f 00 00       	mov    esi,0xff0
  6614e6:	ff 15 e4 5e 0a 00    	call   QWORD PTR [rip+0xa5ee4]        # 7073d0 <_DYNAMIC+0x240>
  6614ec:	48 89 c3             	mov    rbx,rax
  6614ef:	48 85 ed             	test   rbp,rbp
  6614f2:	48 8b 7c 24 08       	mov    rdi,QWORD PTR [rsp+0x8]
  6614f7:	75 0a                	jne    661503 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x653>
  6614f9:	eb 0e                	jmp    661509 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x659>
  6614fb:	48 89 c3             	mov    rbx,rax
  6614fe:	48 8b 7c 24 08       	mov    rdi,QWORD PTR [rsp+0x8]
  661503:	ff 15 8f 5e 0a 00    	call   QWORD PTR [rip+0xa5e8f]        # 707398 <free@GLIBC_2.2.5>
  661509:	48 89 df             	mov    rdi,rbx
  66150c:	e8 1f f5 09 00       	call   700a30 <_Unwind_Resume@plt>
  661511:	cc                   	int3   
  661512:	cc                   	int3   
  661513:	cc                   	int3   
  661514:	cc                   	int3   
  661515:	cc                   	int3   
  661516:	cc                   	int3   
  661517:	cc                   	int3   
  661518:	cc                   	int3   
  661519:	cc                   	int3   
  66151a:	cc                   	int3   
  66151b:	cc                   	int3   
  66151c:	cc                   	int3   
  66151d:	cc                   	int3   
  66151e:	cc                   	int3   
  66151f:	cc                   	int3   

