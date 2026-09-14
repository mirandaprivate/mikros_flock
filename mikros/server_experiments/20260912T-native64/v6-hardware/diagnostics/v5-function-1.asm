000000000064ccc0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_>:
  64ccc0:	push   rbp
  64ccc1:	push   r15
  64ccc3:	push   r14
  64ccc5:	push   r13
  64ccc7:	push   r12
  64ccc9:	push   rbx
  64ccca:	sub    rsp,0x1000
  64ccd1:	mov    QWORD PTR [rsp],0x0
  64ccd9:	sub    rsp,0x7e8
  64cce0:	mov    r14,rdi
  64cce3:	mov    rbx,QWORD PTR [rsp+0x1820]
  64cceb:	mov    QWORD PTR [rsp+0x168],rsi
  64ccf3:	mov    QWORD PTR [rsp+0x170],rcx
  64ccfb:	mov    QWORD PTR [rsp+0x178],r8
  64cd03:	shr    rsi,1
  64cd06:	cmp    rsi,r8
  64cd09:	jae    64cdc4 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x104>
  64cd0f:	mov    rax,QWORD PTR [r9]
  64cd12:	mov    QWORD PTR [rsp+0x90],rax
  64cd1a:	mov    r15,QWORD PTR [r9+0x8]
  64cd1e:	mov    r12,QWORD PTR [r9+0x10]
  64cd22:	mov    rax,QWORD PTR [r9+0x18]
  64cd26:	mov    QWORD PTR [rsp+0x88],rax
  64cd2e:	mov    rax,QWORD PTR [r9+0x20]
  64cd32:	mov    QWORD PTR [rsp+0x40],rax
  64cd37:	mov    rax,QWORD PTR [r9+0x28]
  64cd3b:	mov    QWORD PTR [rsp+0x10],rax
  64cd40:	mov    rax,QWORD PTR [r9+0x30]
  64cd44:	mov    QWORD PTR [rsp+0xd8],rax
  64cd4c:	mov    rax,QWORD PTR [r9+0x38]
  64cd50:	mov    QWORD PTR [rsp+0xa0],rax
  64cd58:	mov    rax,QWORD PTR [r9+0x40]
  64cd5c:	mov    QWORD PTR [rsp+0x38],rax
  64cd61:	mov    rbx,QWORD PTR [rbx+0x10]
  64cd65:	mov    edi,0xff0
  64cd6a:	mov    esi,0x1
  64cd6f:	call   QWORD PTR [rip+0x9a47b]        # 6e71f0 <calloc@GLIBC_2.2.5>
  64cd75:	test   rax,rax
  64cd78:	je     64f081 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x23c1>
  64cd7e:	mov    rdi,rax
  64cd81:	test   r12,r12
  64cd84:	mov    rax,QWORD PTR [rsp+0x40]
  64cd89:	je     64ef5c <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x229c>
  64cd8f:	cmp    QWORD PTR [rsp+0x10],0x0
  64cd95:	je     64ef5c <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x229c>
  64cd9b:	test   r15,r15
  64cd9e:	mov    QWORD PTR [rsp+0x98],r12
  64cda6:	je     64cdfe <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x13e>
  64cda8:	mov    rcx,r12
  64cdab:	mov    rax,r15
  64cdae:	or     rax,r12
  64cdb1:	shr    rax,0x20
  64cdb5:	je     64ce09 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x149>
  64cdb7:	mov    rax,r15
  64cdba:	xor    edx,edx
  64cdbc:	div    rcx
  64cdbf:	mov    rsi,rax
  64cdc2:	jmp    64ce12 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x152>
  64cdc4:	test   dl,dl
  64cdc6:	je     64cdea <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x12a>
  64cdc8:	mov    rbp,r14
  64cdcb:	mov    rax,0xffffffffffffffa8
  64cdd2:	mov    rax,QWORD PTR fs:[rax]
  64cdd6:	test   rax,rax
  64cdd9:	je     64ec1b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1f5b>
  64cddf:	add    rax,0x110
  64cde5:	jmp    64ec33 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1f73>
  64cdea:	test   rcx,rcx
  64cded:	je     64cd0f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x4f>
  64cdf3:	mov    rbp,r14
  64cdf6:	shr    rcx,1
  64cdf9:	jmp    64ec47 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1f87>
  64cdfe:	xor    esi,esi
  64ce00:	test   rax,rax
  64ce03:	jne    64ce24 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x164>
  64ce05:	xor    ecx,ecx
  64ce07:	jmp    64ce55 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x195>
  64ce09:	mov    eax,r15d
  64ce0c:	xor    edx,edx
  64ce0e:	div    ecx
  64ce10:	mov    esi,eax
  64ce12:	cmp    rdx,0x1
  64ce16:	sbb    rsi,0xffffffffffffffff
  64ce1a:	mov    rax,QWORD PTR [rsp+0x40]
  64ce1f:	test   rax,rax
  64ce22:	je     64ce05 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x145>
  64ce24:	mov    rcx,rax
  64ce27:	mov    r8,QWORD PTR [rsp+0x10]
  64ce2c:	or     rcx,r8
  64ce2f:	shr    rcx,0x20
  64ce33:	je     64ce3f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x17f>
  64ce35:	xor    edx,edx
  64ce37:	div    r8
  64ce3a:	mov    rcx,rax
  64ce3d:	jmp    64ce46 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x186>
  64ce3f:	xor    edx,edx
  64ce41:	div    r8d
  64ce44:	mov    ecx,eax
  64ce46:	cmp    rdx,0x1
  64ce4a:	sbb    rcx,0xffffffffffffffff
  64ce4e:	cmp    rcx,rsi
  64ce51:	cmovae rcx,rsi
  64ce55:	cmp    QWORD PTR [rsp+0x38],0x0
  64ce5b:	je     64ef5c <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x229c>
  64ce61:	mov    QWORD PTR [rsp+0xc0],r14
  64ce69:	mov    esi,0xff
  64ce6e:	mov    rax,QWORD PTR [rsp+0xa0]
  64ce76:	test   rax,rax
  64ce79:	je     64ebf4 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1f34>
  64ce7f:	mov    rdx,rax
  64ce82:	mov    r8,QWORD PTR [rsp+0x38]
  64ce87:	or     rdx,r8
  64ce8a:	shr    rdx,0x20
  64ce8e:	je     64ce9a <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1da>
  64ce90:	xor    edx,edx
  64ce92:	div    r8
  64ce95:	mov    r14,rax
  64ce98:	jmp    64cea2 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1e2>
  64ce9a:	xor    edx,edx
  64ce9c:	div    r8d
  64ce9f:	mov    r14d,eax
  64cea2:	cmp    rdx,0x1
  64cea6:	sbb    r14,0xffffffffffffffff
  64ceaa:	cmp    r14,rcx
  64cead:	cmovae r14,rcx
  64ceb1:	test   r14,r14
  64ceb4:	mov    rax,QWORD PTR [rsp+0x98]
  64cebc:	je     64ebf4 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1f34>
  64cec2:	mov    rcx,QWORD PTR [rbx]
  64cec5:	mov    QWORD PTR [rsp+0xd0],rcx
  64cecd:	mov    ecx,0xff
  64ced2:	mov    QWORD PTR [rsp+0x18],rcx
  64ced7:	xor    r12d,r12d
  64ceda:	lea    rcx,[rip+0x97037]        # 6e3f18 <anon.e5d9da81ce3e97b5bac39b4a40b5610e.12.llvm.10381458960231317189+0xd8>
  64cee1:	mov    QWORD PTR [rsp+0x80],rcx
  64cee9:	mov    rcx,QWORD PTR [rsp+0x88]
  64cef1:	mov    QWORD PTR [rsp+0xb0],rcx
  64cef9:	mov    rcx,QWORD PTR [rsp+0x90]
  64cf01:	mov    QWORD PTR [rsp+0xa8],rcx
  64cf09:	mov    rbx,QWORD PTR [rsp+0x10]
  64cf0e:	mov    QWORD PTR [rsp+0xc8],r15
  64cf16:	mov    QWORD PTR [rsp+0x30],r14
  64cf1b:	jmp    64cfdc <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x31c>
  64cf20:	mov    rax,QWORD PTR [rsp+0x20]
  64cf25:	mov    rcx,QWORD PTR [rsp+0x28]
  64cf2a:	mov    rbx,QWORD PTR [rsp+0x10]
  64cf2f:	mov    r14,QWORD PTR [rsp+0x30]
  64cf34:	mov    r12,QWORD PTR [rsp+0x48]
  64cf39:	inc    r12
  64cf3c:	mov    QWORD PTR [rsp+0x7e0],rcx
  64cf44:	mov    QWORD PTR [rsp+0x7e8],rax
  64cf4c:	mov    rax,QWORD PTR [rsp+0x18]
  64cf51:	mov    QWORD PTR [rsp+0x7f0],rax
  64cf59:	mov    QWORD PTR [rsp+0x7f8],0xff
  64cf65:	mov    QWORD PTR [rsp+0x800],r15
  64cf6d:	mov    QWORD PTR [rsp+0x808],0xff
  64cf79:	lea    rdi,[rsp+0x180]
  64cf81:	lea    rsi,[rsp+0x7e0]
  64cf89:	lea    rdx,[rsp+0x7f8]
  64cf91:	call   656c90 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0B5_>
  64cf96:	mov    rsi,QWORD PTR [rsp+0x180]
  64cf9e:	mov    rdi,QWORD PTR [rsp+0x188]
  64cfa6:	mov    rax,QWORD PTR [rsp+0x190]
  64cfae:	mov    QWORD PTR [rsp+0x18],rax
  64cfb3:	mov    rax,QWORD PTR [rsp+0x98]
  64cfbb:	add    QWORD PTR [rsp+0xa8],rax
  64cfc3:	add    QWORD PTR [rsp+0xb0],rbx
  64cfcb:	cmp    r12,r14
  64cfce:	mov    r15,QWORD PTR [rsp+0xc8]
  64cfd6:	je     64ebfe <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1f3e>
  64cfdc:	mov    QWORD PTR [rsp+0x28],rsi
  64cfe1:	mov    QWORD PTR [rsp+0x20],rdi
  64cfe6:	mov    rcx,r12
  64cfe9:	imul   rcx,rax
  64cfed:	mov    r14,r15
  64cff0:	sub    r14,rcx
  64cff3:	cmp    rax,r14
  64cff6:	cmovb  r14,rax
  64cffa:	add    rcx,QWORD PTR [rsp+0x90]
  64d002:	mov    QWORD PTR [rsp+0x68],rcx
  64d007:	mov    rax,r12
  64d00a:	imul   rax,rbx
  64d00e:	mov    r13,QWORD PTR [rsp+0x40]
  64d013:	sub    r13,rax
  64d016:	cmp    rbx,r13
  64d019:	cmovb  r13,rbx
  64d01d:	add    rax,QWORD PTR [rsp+0x88]
  64d025:	mov    QWORD PTR [rsp+0x58],rax
  64d02a:	mov    rcx,r12
  64d02d:	mov    rax,QWORD PTR [rsp+0x38]
  64d032:	imul   rcx,rax
  64d036:	mov    r15,QWORD PTR [rsp+0xa0]
  64d03e:	sub    r15,rcx
  64d041:	cmp    rax,r15
  64d044:	cmovb  r15,rax
  64d048:	shl    rcx,0x4
  64d04c:	add    rcx,QWORD PTR [rsp+0xd8]
  64d054:	mov    QWORD PTR [rsp+0x60],rcx
  64d059:	mov    rax,QWORD PTR [rsp+0xd0]
  64d061:	movzx  eax,BYTE PTR [rax]
  64d064:	cmp    eax,0x2
  64d067:	mov    QWORD PTR [rsp+0x48],r12
  64d06c:	je     64d660 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x9a0>
  64d072:	test   eax,eax
  64d074:	jne    64e1c0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1500>
  64d07a:	mov    eax,DWORD PTR [rip+0x9c1d8]        # 6e9258 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.10381458960231317189+0x130>
  64d080:	test   eax,eax
  64d082:	jne    64ebb8 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1ef8>
  64d088:	mov    eax,DWORD PTR [rip+0x9d2f2]        # 6ea380 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.10381458960231317189+0x1120>
  64d08e:	test   eax,eax
  64d090:	jne    64ebcc <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1f0c>
  64d096:	mov    edi,0xff0
  64d09b:	mov    esi,0x1
  64d0a0:	call   QWORD PTR [rip+0x9a14a]        # 6e71f0 <calloc@GLIBC_2.2.5>
  64d0a6:	test   rax,rax
  64d0a9:	je     64f032 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2372>
  64d0af:	mov    rbx,rax
  64d0b2:	shr    r14,0x4
  64d0b6:	shr    r13,0x4
  64d0ba:	cmp    r13,r14
  64d0bd:	cmovae r13,r14
  64d0c1:	cmp    r15,r13
  64d0c4:	cmovb  r13,r15
  64d0c8:	test   r13,r13
  64d0cb:	je     64d445 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x785>
  64d0d1:	mov    r12,r13
  64d0d4:	xor    r14d,r14d
  64d0d7:	nop    WORD PTR [rax+rax*1+0x0]
  64d0e0:	mov    r15,r14
  64d0e3:	shl    r15,0x4
  64d0e7:	mov    rax,QWORD PTR [rsp+0x68]
  64d0ec:	lea    rsi,[rax+r15*1]
  64d0f0:	lea    rdi,[rsp+0x4e0]
  64d0f8:	vzeroupper 
  64d0fb:	call   6585d0 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate6domainNtB2_6Domain8eval_row>
  64d100:	mov    rax,QWORD PTR [rsp+0x58]
  64d105:	lea    rsi,[rax+r15*1]
  64d109:	lea    rdi,[rsp+0x180]
  64d111:	call   6585d0 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate6domainNtB2_6Domain8eval_row>
  64d116:	inc    r14
  64d119:	vmovups ymm0,YMMWORD PTR [rsp+0x5bf]
  64d122:	lea    rcx,[rsp+0x800]
  64d12a:	vmovups YMMWORD PTR [rcx+0xdf],ymm0
  64d132:	vmovups ymm0,YMMWORD PTR [rsp+0x5a0]
  64d13b:	vmovups YMMWORD PTR [rcx+0xc0],ymm0
  64d143:	vmovups ymm0,YMMWORD PTR [rsp+0x580]
  64d14c:	vmovups YMMWORD PTR [rcx+0xa0],ymm0
  64d154:	vmovups ymm0,YMMWORD PTR [rsp+0x560]
  64d15d:	vmovups YMMWORD PTR [rcx+0x80],ymm0
  64d165:	vmovups ymm0,YMMWORD PTR [rsp+0x4e0]
  64d16e:	vmovups ymm1,YMMWORD PTR [rsp+0x500]
  64d177:	vmovups ymm2,YMMWORD PTR [rsp+0x520]
  64d180:	vmovups ymm3,YMMWORD PTR [rsp+0x540]
  64d189:	vmovups YMMWORD PTR [rcx+0x60],ymm3
  64d18e:	vmovups YMMWORD PTR [rcx+0x40],ymm2
  64d193:	vmovups YMMWORD PTR [rcx+0x20],ymm1
  64d198:	vmovups YMMWORD PTR [rcx],ymm0
  64d19c:	lea    rax,[rsp+0x6e1]
  64d1a4:	vxorps xmm4,xmm4,xmm4
  64d1a8:	vmovups XMMWORD PTR [rax],xmm4
  64d1ac:	vmovups ymm0,YMMWORD PTR [rsp+0x25f]
  64d1b5:	lea    rdx,[rsp+0x920]
  64d1bd:	vmovups YMMWORD PTR [rdx+0xdf],ymm0
  64d1c5:	vmovups ymm0,YMMWORD PTR [rsp+0x240]
  64d1ce:	vmovups YMMWORD PTR [rdx+0xc0],ymm0
  64d1d6:	vmovups ymm0,YMMWORD PTR [rsp+0x220]
  64d1df:	vmovups YMMWORD PTR [rdx+0xa0],ymm0
  64d1e7:	vmovups ymm0,YMMWORD PTR [rsp+0x200]
  64d1f0:	vmovups YMMWORD PTR [rdx+0x80],ymm0
  64d1f8:	vmovups ymm0,YMMWORD PTR [rsp+0x180]
  64d201:	vmovups ymm1,YMMWORD PTR [rsp+0x1a0]
  64d20a:	vmovups ymm2,YMMWORD PTR [rsp+0x1c0]
  64d213:	vmovups ymm3,YMMWORD PTR [rsp+0x1e0]
  64d21c:	vmovups YMMWORD PTR [rdx+0x60],ymm3
  64d221:	vmovups YMMWORD PTR [rdx+0x40],ymm2
  64d226:	vmovups YMMWORD PTR [rdx+0x20],ymm1
  64d22b:	vmovups YMMWORD PTR [rdx],ymm0
  64d22f:	add    r15,QWORD PTR [rsp+0x60]
  64d234:	movzx  eax,BYTE PTR [rsp+0x6f0]
  64d23c:	vmovups xmm0,XMMWORD PTR [rsp+0x6e0]
  64d245:	vmovups XMMWORD PTR [rcx+0xff],xmm0
  64d24d:	mov    BYTE PTR [rcx+0x10f],al
  64d253:	lea    rax,[rsp+0x661]
  64d25b:	vmovups XMMWORD PTR [rax],xmm4
  64d25f:	mov    QWORD PTR [rsp+0x910],0x0
  64d26b:	mov    QWORD PTR [rsp+0x918],0xff
  64d277:	movzx  eax,BYTE PTR [rsp+0x670]
  64d27f:	mov    BYTE PTR [rcx+0x22f],al
  64d285:	vmovups xmm0,XMMWORD PTR [rsp+0x660]
  64d28e:	vmovups XMMWORD PTR [rcx+0x21f],xmm0
  64d296:	mov    rax,QWORD PTR [r15]
  64d299:	mov    rcx,QWORD PTR [r15+0x8]
  64d29d:	vmovq  xmm0,rax
  64d2a2:	vmovq  xmm1,rcx
  64d2a7:	xor    rcx,rax
  64d2aa:	vmovq  xmm2,rcx
  64d2af:	mov    rax,rdx
  64d2b2:	xor    ecx,ecx
  64d2b4:	mov    r10,QWORD PTR [rip+0x9a345]        # 6e7600 <_DYNAMIC+0x630>
  64d2bb:	mov    r11d,0x87
  64d2c1:	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  64d2d0:	movzx  edx,BYTE PTR [rax-0x120]
  64d2d7:	shl    edx,0x4
  64d2da:	mov    rsi,QWORD PTR [r10+rdx*1]
  64d2de:	mov    rdx,QWORD PTR [r10+rdx*1+0x8]
  64d2e3:	vmovq  xmm3,rsi
  64d2e8:	vpclmullqlqdq xmm3,xmm0,xmm3
  64d2ee:	vmovq  xmm4,rdx
  64d2f3:	vpclmullqlqdq xmm4,xmm1,xmm4
  64d2f9:	xor    rdx,rsi
  64d2fc:	vmovq  xmm5,rdx
  64d301:	vpclmullqlqdq xmm5,xmm2,xmm5
  64d307:	vpternlogq xmm5,xmm3,xmm4,0x96
  64d30e:	vpshufd xmm6,xmm4,0xee
  64d313:	vmovq  xmm7,r11
  64d318:	vpclmullqlqdq xmm6,xmm6,xmm7
  64d31e:	vpshufd xmm8,xmm5,0xee
  64d323:	vpxor  xmm4,xmm8,xmm4
  64d327:	movzx  edx,BYTE PTR [rax]
  64d32a:	vpclmullqlqdq xmm4,xmm4,xmm7
  64d330:	vpextrq rsi,xmm6,0x1
  64d336:	lea    rdi,[rsi+rsi*1]
  64d33a:	lea    r8,[rsi*4+0x0]
  64d342:	mov    r9,rsi
  64d345:	xor    r8,rdi
  64d348:	shl    r9,0x7
  64d34c:	xor    r9,r8
  64d34f:	vpxor  xmm3,xmm3,xmm4
  64d353:	vmovq  rdi,xmm3
  64d358:	vpshufd xmm3,xmm3,0xee
  64d35d:	xor    rdi,r9
  64d360:	vpternlogq xmm3,xmm5,xmm6,0x96
  64d367:	vmovq  r8,xmm3
  64d36c:	shl    edx,0x4
  64d36f:	mov    r9,QWORD PTR [r10+rdx*1]
  64d373:	mov    rdx,QWORD PTR [r10+rdx*1+0x8]
  64d378:	xor    rdi,rsi
  64d37b:	vmovq  xmm4,rdi
  64d380:	vmovq  xmm5,r9
  64d385:	vmovq  xmm6,rdx
  64d38a:	vpclmullqlqdq xmm3,xmm3,xmm6
  64d390:	xor    r8,rdi
  64d393:	vpclmullqlqdq xmm4,xmm4,xmm5
  64d399:	xor    rdx,r9
  64d39c:	vmovq  xmm5,r8
  64d3a1:	vmovq  xmm6,rdx
  64d3a6:	vpclmullqlqdq xmm5,xmm5,xmm6
  64d3ac:	vpshufd xmm6,xmm3,0xee
  64d3b1:	vpternlogq xmm5,xmm4,xmm3,0x96
  64d3b8:	vpclmullqlqdq xmm6,xmm6,xmm7
  64d3be:	vpshufd xmm8,xmm5,0xee
  64d3c3:	vpxor  xmm3,xmm8,xmm3
  64d3c7:	vpclmullqlqdq xmm3,xmm3,xmm7
  64d3cd:	vpextrq rdx,xmm6,0x1
  64d3d3:	lea    rsi,[rdx+rdx*1]
  64d3d7:	shl    rdx,0x2
  64d3db:	xor    rdx,rsi
  64d3de:	vpxor  xmm7,xmm4,xmm3
  64d3e2:	vmovq  rsi,xmm7
  64d3e7:	xor    rsi,rdx
  64d3ea:	vpshufd xmm4,xmm4,0xee
  64d3ef:	vpsllq xmm7,xmm6,0x7
  64d3f4:	vpxor  xmm4,xmm6,xmm4
  64d3f8:	vpbroadcastq xmm5,xmm5
  64d3fd:	vpinsrq xmm5,xmm5,rsi,0x0
  64d403:	vpalignr xmm4,xmm4,xmm7,0x8
  64d409:	vmovdqa xmm7,XMMWORD PTR [rbx+rcx*1]
  64d40e:	vpblendd xmm3,xmm7,xmm3,0xc
  64d414:	vpunpckhqdq xmm6,xmm6,xmm7
  64d418:	vpxor  xmm4,xmm4,xmm5
  64d41c:	vpternlogq xmm6,xmm4,xmm3,0x96
  64d423:	vmovdqa XMMWORD PTR [rbx+rcx*1],xmm6
  64d428:	add    rcx,0x10
  64d42c:	inc    rax
  64d42f:	cmp    rcx,0xff0
  64d436:	jne    64d2d0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x610>
  64d43c:	cmp    r14,r12
  64d43f:	jne    64d0e0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x420>
  64d445:	mov    edi,0xff0
  64d44a:	vzeroupper 
  64d44d:	call   QWORD PTR [rip+0x99d75]        # 6e71c8 <malloc@GLIBC_2.2.5>
  64d453:	test   rax,rax
  64d456:	je     64f04b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x238b>
  64d45c:	mov    r15,rax
  64d45f:	mov    rdi,QWORD PTR [rip+0x9bcda]        # 6e9140 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.10381458960231317189+0x18>
  64d466:	cmp    rdi,0x10
  64d46a:	jb     64f039 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2379>
  64d470:	mov    rax,QWORD PTR [rip+0x9bcc1]        # 6e9138 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.10381458960231317189+0x10>
  64d477:	lea    rbp,[rax+0xc000]
  64d47e:	lea    rcx,[rax+0xd000]
  64d485:	lea    rdx,[rax+0xe000]
  64d48c:	lea    rsi,[rax+0xf000]
  64d493:	xor    edi,edi
  64d495:	data16 cs nop WORD PTR [rax+rax*1+0x0]
  64d4a0:	mov    r8,QWORD PTR [rbx+rdi*1]
  64d4a4:	mov    r11,r8
  64d4a7:	mov    r10,r8
  64d4aa:	mov    r14,r8
  64d4ad:	mov    r12,r8
  64d4b0:	mov    r9d,r8d
  64d4b3:	shr    r9d,0x4
  64d4b7:	and    r9d,0xff0
  64d4be:	lea    r13,[rax+0x1000]
  64d4c5:	vmovdqa xmm0,XMMWORD PTR [r13+r9*1+0x0]
  64d4cc:	movzx  r9d,r8b
  64d4d0:	shl    r9d,0x4
  64d4d4:	vpxor  xmm0,xmm0,XMMWORD PTR [rax+r9*1]
  64d4da:	mov    r9d,r8d
  64d4dd:	shr    r9d,0xc
  64d4e1:	and    r9d,0xff0
  64d4e8:	shr    r8d,0x14
  64d4ec:	and    r8d,0xfffffff0
  64d4f0:	lea    r13,[rax+0x3000]
  64d4f7:	vmovdqa xmm1,XMMWORD PTR [r13+r8*1+0x0]
  64d4fe:	lea    r8,[rax+0x2000]
  64d505:	vpternlogq xmm1,xmm0,XMMWORD PTR [r8+r9*1],0x96
  64d50d:	mov    r9,QWORD PTR [rbx+rdi*1+0x8]
  64d512:	mov    r8,r9
  64d515:	shr    r14,0x28
  64d519:	movzx  r14d,r14b
  64d51d:	shl    r14d,0x4
  64d521:	lea    r13,[rax+0x5000]
  64d528:	vmovdqa xmm0,XMMWORD PTR [r13+r14*1+0x0]
  64d52f:	mov    r14,r9
  64d532:	shr    r12,0x20
  64d536:	movzx  r12d,r12b
  64d53a:	shl    r12d,0x4
  64d53e:	lea    r13,[rax+0x4000]
  64d545:	vpternlogq xmm0,xmm1,XMMWORD PTR [r13+r12*1+0x0],0x96
  64d54e:	mov    r12,r9
  64d551:	shr    r11,0x38
  64d555:	shl    r11d,0x4
  64d559:	lea    r13,[rax+0x7000]
  64d560:	vmovdqa xmm1,XMMWORD PTR [r13+r11*1+0x0]
  64d567:	mov    r11,r9
  64d56a:	shr    r10,0x30
  64d56e:	movzx  r10d,r10b
  64d572:	shl    r10d,0x4
  64d576:	lea    r13,[rax+0x6000]
  64d57d:	vpternlogq xmm1,xmm0,XMMWORD PTR [r13+r10*1+0x0],0x96
  64d586:	mov    r10d,r9d
  64d589:	shr    r10d,0x4
  64d58d:	and    r10d,0xff0
  64d594:	lea    r13,[rax+0x9000]
  64d59b:	vmovdqa xmm0,XMMWORD PTR [r13+r10*1+0x0]
  64d5a2:	movzx  r10d,r9b
  64d5a6:	shl    r10d,0x4
  64d5aa:	lea    r13,[rax+0x8000]
  64d5b1:	vpternlogq xmm0,xmm1,XMMWORD PTR [r13+r10*1+0x0],0x96
  64d5ba:	mov    r10d,r9d
  64d5bd:	shr    r9d,0x14
  64d5c1:	and    r9d,0xfffffff0
  64d5c5:	lea    r13,[rax+0xb000]
  64d5cc:	vmovdqa xmm1,XMMWORD PTR [r13+r9*1+0x0]
  64d5d3:	shr    r10d,0xc
  64d5d7:	and    r10d,0xff0
  64d5de:	lea    r9,[rax+0xa000]
  64d5e5:	vpternlogq xmm1,xmm0,XMMWORD PTR [r9+r10*1],0x96
  64d5ed:	shr    r12,0x28
  64d5f1:	movzx  r9d,r12b
  64d5f5:	shl    r9d,0x4
  64d5f9:	vmovdqa xmm0,XMMWORD PTR [rcx+r9*1]
  64d5ff:	shr    r11,0x20
  64d603:	movzx  r9d,r11b
  64d607:	shl    r9d,0x4
  64d60b:	vpternlogq xmm0,xmm1,XMMWORD PTR [rbp+r9*1+0x0],0x96
  64d614:	shr    r8,0x38
  64d618:	shl    r8d,0x4
  64d61c:	vmovdqa xmm1,XMMWORD PTR [rsi+r8*1]
  64d622:	shr    r14,0x30
  64d626:	movzx  r8d,r14b
  64d62a:	shl    r8d,0x4
  64d62e:	vpternlogq xmm1,xmm0,XMMWORD PTR [rdx+r8*1],0x96
  64d636:	vmovdqu XMMWORD PTR [r15+rdi*1],xmm1
  64d63c:	add    rdi,0x10
  64d640:	cmp    rdi,0xff0
  64d647:	jne    64d4a0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x7e0>
  64d64d:	mov    rdi,rbx
  64d650:	call   QWORD PTR [rip+0x99b82]        # 6e71d8 <free@GLIBC_2.2.5>
  64d656:	jmp    64cf20 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x260>
  64d65b:	nop    DWORD PTR [rax+rax*1+0x0]
  64d660:	mov    eax,DWORD PTR [rip+0x9bbf2]        # 6e9258 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.10381458960231317189+0x130>
  64d666:	test   eax,eax
  64d668:	jne    64ebc2 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1f02>
  64d66e:	mov    eax,DWORD PTR [rip+0x9cd0c]        # 6ea380 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.10381458960231317189+0x1120>
  64d674:	test   eax,eax
  64d676:	jne    64ebd6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1f16>
  64d67c:	shr    r14,0x4
  64d680:	shr    r13,0x4
  64d684:	cmp    r13,r14
  64d687:	cmovae r13,r14
  64d68b:	cmp    r15,r13
  64d68e:	cmovb  r13,r15
  64d692:	mov    edx,0x1000
  64d697:	lea    rdi,[rsp+0x7e0]
  64d69f:	xor    esi,esi
  64d6a1:	call   QWORD PTR [rip+0x99b51]        # 6e71f8 <memset@GLIBC_2.2.5>
  64d6a7:	mov    QWORD PTR [rsp+0x50],r13
  64d6ac:	test   r13,r13
  64d6af:	je     64e078 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x13b8>
  64d6b5:	mov    r15,QWORD PTR [rsp+0xb0]
  64d6bd:	mov    rbp,QWORD PTR [rsp+0xa8]
  64d6c5:	xor    ebx,ebx
  64d6c7:	nop    WORD PTR [rax+rax*1+0x0]
  64d6d0:	mov    QWORD PTR [rsp+0x8],rbx
  64d6d5:	shl    rbx,0x4
  64d6d9:	mov    rax,QWORD PTR [rsp+0x68]
  64d6de:	lea    rsi,[rax+rbx*1]
  64d6e2:	mov    rax,QWORD PTR [rsp+0x60]
  64d6e7:	mov    r14,QWORD PTR [rax+rbx*1]
  64d6eb:	mov    r13,QWORD PTR [rax+rbx*1+0x8]
  64d6f0:	vpxor  xmm0,xmm0,xmm0
  64d6f4:	vmovdqu YMMWORD PTR [rsp+0x1e0],ymm0
  64d6fd:	vmovdqu YMMWORD PTR [rsp+0x1c0],ymm0
  64d706:	vmovdqu YMMWORD PTR [rsp+0x1a0],ymm0
  64d70f:	vmovdqu YMMWORD PTR [rsp+0x180],ymm0
  64d718:	mov    rax,QWORD PTR [rip+0x9cb79]        # 6ea298 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.10381458960231317189+0x1038>
  64d71f:	cmp    rax,0x80
  64d725:	je     64d9c0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0xd00>
  64d72b:	cmp    rax,0x40
  64d72f:	je     64efb9 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x22f9>
  64d735:	cmp    rax,0xf
  64d739:	jbe    64d9f0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0xd30>
  64d73f:	mov    QWORD PTR [rsp+0x6e0],0x10
  64d74b:	cmp    QWORD PTR [rip+0x9cb4d],0x10        # 6ea2a0 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.10381458960231317189+0x1040>
  64d753:	jne    64ee9e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x21de>
  64d759:	mov    QWORD PTR [rsp+0x4e0],0x80
  64d765:	cmp    rax,0x80
  64d76b:	jne    64eeb6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x21f6>
  64d771:	mov    rax,QWORD PTR [rip+0x9caf0]        # 6ea268 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.10381458960231317189+0x1008>
  64d778:	add    rax,QWORD PTR [rip+0x9cb29]        # 6ea2a8 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.10381458960231317189+0x1048>
  64d77f:	mov    r12,rsi
  64d782:	movzx  ecx,BYTE PTR [rsi]
  64d785:	shl    ecx,0x7
  64d788:	vmovups ymm0,YMMWORD PTR [rcx+rax*1]
  64d78d:	vmovups ymm1,YMMWORD PTR [rcx+rax*1+0x20]
  64d793:	vmovups ymm2,YMMWORD PTR [rcx+rax*1+0x40]
  64d799:	vmovups ymm3,YMMWORD PTR [rcx+rax*1+0x60]
  64d79f:	vmovups YMMWORD PTR [rsp+0x180],ymm0
  64d7a8:	vmovups YMMWORD PTR [rsp+0x1a0],ymm1
  64d7b1:	vmovups YMMWORD PTR [rsp+0x1c0],ymm2
  64d7ba:	vmovups YMMWORD PTR [rsp+0x1e0],ymm3
  64d7c3:	vmovdqa xmm0,XMMWORD PTR [rsp+0x180]
  64d7cc:	vmovdqa xmm1,XMMWORD PTR [rsp+0x190]
  64d7d5:	vmovdqa xmm2,XMMWORD PTR [rsp+0x1a0]
  64d7de:	vmovdqa xmm3,XMMWORD PTR [rsp+0x1b0]
  64d7e7:	vmovdqa xmm4,XMMWORD PTR [rsp+0x1c0]
  64d7f0:	vmovdqa xmm5,XMMWORD PTR [rsp+0x1d0]
  64d7f9:	vmovdqa xmm6,XMMWORD PTR [rsp+0x1e0]
  64d802:	vmovdqa xmm7,XMMWORD PTR [rsp+0x1f0]
  64d80b:	mov    ecx,0x1
  64d810:	jmp    64d8cb <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0xc0b>
  64d815:	data16 cs nop WORD PTR [rax+rax*1+0x0]
  64d820:	vpxor  xmm0,xmm0,XMMWORD PTR [rsi+rdx*1]
  64d825:	vmovdqa XMMWORD PTR [rsp+0x180],xmm0
  64d82e:	mov    rdi,rdx
  64d831:	xor    rdi,0x10
  64d835:	vpxor  xmm1,xmm1,XMMWORD PTR [rsi+rdi*1]
  64d83a:	vmovdqa XMMWORD PTR [rsp+0x190],xmm1
  64d843:	mov    rdi,rdx
  64d846:	xor    rdi,0x20
  64d84a:	vpxor  xmm2,xmm2,XMMWORD PTR [rsi+rdi*1]
  64d84f:	vmovdqa XMMWORD PTR [rsp+0x1a0],xmm2
  64d858:	mov    rdi,rdx
  64d85b:	xor    rdi,0x30
  64d85f:	vpxor  xmm3,xmm3,XMMWORD PTR [rsi+rdi*1]
  64d864:	vmovdqa XMMWORD PTR [rsp+0x1b0],xmm3
  64d86d:	mov    rdi,rdx
  64d870:	xor    rdi,0x40
  64d874:	vpxor  xmm4,xmm4,XMMWORD PTR [rsi+rdi*1]
  64d879:	vmovdqa XMMWORD PTR [rsp+0x1c0],xmm4
  64d882:	mov    rdi,rdx
  64d885:	xor    rdi,0x50
  64d889:	vpxor  xmm5,xmm5,XMMWORD PTR [rsi+rdi*1]
  64d88e:	vmovdqa XMMWORD PTR [rsp+0x1d0],xmm5
  64d897:	mov    rdi,rdx
  64d89a:	xor    rdi,0x60
  64d89e:	vpxor  xmm6,xmm6,XMMWORD PTR [rsi+rdi*1]
  64d8a3:	vmovdqa XMMWORD PTR [rsp+0x1e0],xmm6
  64d8ac:	xor    rdx,0x70
  64d8b0:	vpxor  xmm7,xmm7,XMMWORD PTR [rsi+rdx*1]
  64d8b5:	vmovdqa XMMWORD PTR [rsp+0x1f0],xmm7
  64d8be:	inc    rcx
  64d8c1:	cmp    rcx,0x10
  64d8c5:	je     64da16 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0xd56>
  64d8cb:	mov    rdx,rcx
  64d8ce:	shr    rdx,1
  64d8d1:	movzx  esi,BYTE PTR [rbp+rcx*1+0x0]
  64d8d6:	shl    esi,0x7
  64d8d9:	add    rsi,rax
  64d8dc:	shl    rdx,0x4
  64d8e0:	test   cl,0x1
  64d8e3:	je     64d820 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0xb60>
  64d8e9:	vpshufd xmm8,XMMWORD PTR [rsi+rdx*1],0x4e
  64d8ef:	vpxor  xmm0,xmm8,xmm0
  64d8f3:	vmovdqa XMMWORD PTR [rsp+0x180],xmm0
  64d8fc:	mov    rdi,rdx
  64d8ff:	xor    rdi,0x10
  64d903:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  64d909:	vpxor  xmm1,xmm8,xmm1
  64d90d:	vmovdqa XMMWORD PTR [rsp+0x190],xmm1
  64d916:	mov    rdi,rdx
  64d919:	xor    rdi,0x20
  64d91d:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  64d923:	vpxor  xmm2,xmm8,xmm2
  64d927:	vmovdqa XMMWORD PTR [rsp+0x1a0],xmm2
  64d930:	mov    rdi,rdx
  64d933:	xor    rdi,0x30
  64d937:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  64d93d:	vpxor  xmm3,xmm8,xmm3
  64d941:	vmovdqa XMMWORD PTR [rsp+0x1b0],xmm3
  64d94a:	mov    rdi,rdx
  64d94d:	xor    rdi,0x40
  64d951:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  64d957:	vpxor  xmm4,xmm8,xmm4
  64d95b:	vmovdqa XMMWORD PTR [rsp+0x1c0],xmm4
  64d964:	mov    rdi,rdx
  64d967:	xor    rdi,0x50
  64d96b:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  64d971:	vpxor  xmm5,xmm8,xmm5
  64d975:	vmovdqa XMMWORD PTR [rsp+0x1d0],xmm5
  64d97e:	mov    rdi,rdx
  64d981:	xor    rdi,0x60
  64d985:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  64d98b:	vpxor  xmm6,xmm8,xmm6
  64d98f:	vmovdqa XMMWORD PTR [rsp+0x1e0],xmm6
  64d998:	xor    rdx,0x70
  64d99c:	vpshufd xmm8,XMMWORD PTR [rsi+rdx*1],0x4e
  64d9a2:	vpxor  xmm7,xmm8,xmm7
  64d9a6:	vmovdqa XMMWORD PTR [rsp+0x1f0],xmm7
  64d9af:	inc    rcx
  64d9b2:	cmp    rcx,0x10
  64d9b6:	jne    64d8cb <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0xc0b>
  64d9bc:	jmp    64da16 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0xd56>
  64d9be:	xchg   ax,ax
  64d9c0:	mov    edx,0x10
  64d9c5:	mov    r8d,0x80
  64d9cb:	lea    rdi,[rip+0x9c88e]        # 6ea260 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.10381458960231317189+0x1000>
  64d9d2:	mov    r12,rsi
  64d9d5:	lea    rcx,[rsp+0x180]
  64d9dd:	vzeroupper 
  64d9e0:	call   QWORD PTR [rip+0x99c4a]        # 6e7630 <_DYNAMIC+0x660>
  64d9e6:	jmp    64da16 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0xd56>
  64d9e8:	nop    DWORD PTR [rax+rax*1+0x0]
  64d9f0:	mov    edx,0x10
  64d9f5:	mov    r8d,0x80
  64d9fb:	lea    rdi,[rip+0x9c85e]        # 6ea260 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.10381458960231317189+0x1000>
  64da02:	mov    r12,rsi
  64da05:	lea    rcx,[rsp+0x180]
  64da0d:	vzeroupper 
  64da10:	call   QWORD PTR [rip+0x99c12]        # 6e7628 <_DYNAMIC+0x658>
  64da16:	add    rbx,QWORD PTR [rsp+0x58]
  64da1b:	vmovups zmm0,ZMMWORD PTR [rsp+0x180]
  64da23:	vmovups ZMMWORD PTR [rsp+0x620],zmm0
  64da2e:	vmovdqu64 zmm0,ZMMWORD PTR [rsp+0x1c0]
  64da36:	vmovdqu64 ZMMWORD PTR [rsp+0x5e0],zmm0
  64da41:	vpxor  xmm0,xmm0,xmm0
  64da45:	vmovdqu YMMWORD PTR [rsp+0x1e0],ymm0
  64da4e:	vmovdqu YMMWORD PTR [rsp+0x1c0],ymm0
  64da57:	vmovdqu YMMWORD PTR [rsp+0x1a0],ymm0
  64da60:	vmovdqu YMMWORD PTR [rsp+0x180],ymm0
  64da69:	mov    rax,QWORD PTR [rip+0x9c828]        # 6ea298 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.10381458960231317189+0x1038>
  64da70:	cmp    rax,0x80
  64da76:	je     64dd10 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1050>
  64da7c:	cmp    rax,0x40
  64da80:	je     64efb9 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x22f9>
  64da86:	cmp    rax,0xf
  64da8a:	jbe    64dd40 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1080>
  64da90:	mov    QWORD PTR [rsp+0x6e0],0x10
  64da9c:	cmp    QWORD PTR [rip+0x9c7fc],0x10        # 6ea2a0 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.10381458960231317189+0x1040>
  64daa4:	jne    64ee9e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x21de>
  64daaa:	mov    QWORD PTR [rsp+0x4e0],0x80
  64dab6:	cmp    rax,0x80
  64dabc:	jne    64eeb6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x21f6>
  64dac2:	mov    rax,QWORD PTR [rip+0x9c79f]        # 6ea268 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.10381458960231317189+0x1008>
  64dac9:	add    rax,QWORD PTR [rip+0x9c7d8]        # 6ea2a8 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.10381458960231317189+0x1048>
  64dad0:	movzx  ecx,BYTE PTR [rbx]
  64dad3:	shl    ecx,0x7
  64dad6:	vmovups ymm0,YMMWORD PTR [rcx+rax*1]
  64dadb:	vmovups ymm1,YMMWORD PTR [rcx+rax*1+0x20]
  64dae1:	vmovups ymm2,YMMWORD PTR [rcx+rax*1+0x40]
  64dae7:	vmovups ymm3,YMMWORD PTR [rcx+rax*1+0x60]
  64daed:	vmovups YMMWORD PTR [rsp+0x180],ymm0
  64daf6:	vmovups YMMWORD PTR [rsp+0x1a0],ymm1
  64daff:	vmovups YMMWORD PTR [rsp+0x1c0],ymm2
  64db08:	vmovups YMMWORD PTR [rsp+0x1e0],ymm3
  64db11:	vmovdqa xmm0,XMMWORD PTR [rsp+0x180]
  64db1a:	vmovdqa xmm1,XMMWORD PTR [rsp+0x190]
  64db23:	vmovdqa xmm2,XMMWORD PTR [rsp+0x1a0]
  64db2c:	vmovdqa xmm3,XMMWORD PTR [rsp+0x1b0]
  64db35:	vmovdqa xmm4,XMMWORD PTR [rsp+0x1c0]
  64db3e:	vmovdqa xmm5,XMMWORD PTR [rsp+0x1d0]
  64db47:	vmovdqa xmm6,XMMWORD PTR [rsp+0x1e0]
  64db50:	vmovdqa xmm7,XMMWORD PTR [rsp+0x1f0]
  64db59:	mov    ecx,0x1
  64db5e:	jmp    64dc1b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0xf5b>
  64db63:	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  64db70:	vpxor  xmm0,xmm0,XMMWORD PTR [rsi+rdx*1]
  64db75:	vmovdqa XMMWORD PTR [rsp+0x180],xmm0
  64db7e:	mov    rdi,rdx
  64db81:	xor    rdi,0x10
  64db85:	vpxor  xmm1,xmm1,XMMWORD PTR [rsi+rdi*1]
  64db8a:	vmovdqa XMMWORD PTR [rsp+0x190],xmm1
  64db93:	mov    rdi,rdx
  64db96:	xor    rdi,0x20
  64db9a:	vpxor  xmm2,xmm2,XMMWORD PTR [rsi+rdi*1]
  64db9f:	vmovdqa XMMWORD PTR [rsp+0x1a0],xmm2
  64dba8:	mov    rdi,rdx
  64dbab:	xor    rdi,0x30
  64dbaf:	vpxor  xmm3,xmm3,XMMWORD PTR [rsi+rdi*1]
  64dbb4:	vmovdqa XMMWORD PTR [rsp+0x1b0],xmm3
  64dbbd:	mov    rdi,rdx
  64dbc0:	xor    rdi,0x40
  64dbc4:	vpxor  xmm4,xmm4,XMMWORD PTR [rsi+rdi*1]
  64dbc9:	vmovdqa XMMWORD PTR [rsp+0x1c0],xmm4
  64dbd2:	mov    rdi,rdx
  64dbd5:	xor    rdi,0x50
  64dbd9:	vpxor  xmm5,xmm5,XMMWORD PTR [rsi+rdi*1]
  64dbde:	vmovdqa XMMWORD PTR [rsp+0x1d0],xmm5
  64dbe7:	mov    rdi,rdx
  64dbea:	xor    rdi,0x60
  64dbee:	vpxor  xmm6,xmm6,XMMWORD PTR [rsi+rdi*1]
  64dbf3:	vmovdqa XMMWORD PTR [rsp+0x1e0],xmm6
  64dbfc:	xor    rdx,0x70
  64dc00:	vpxor  xmm7,xmm7,XMMWORD PTR [rsi+rdx*1]
  64dc05:	vmovdqa XMMWORD PTR [rsp+0x1f0],xmm7
  64dc0e:	inc    rcx
  64dc11:	cmp    rcx,0x10
  64dc15:	je     64dd66 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x10a6>
  64dc1b:	mov    rdx,rcx
  64dc1e:	shr    rdx,1
  64dc21:	movzx  esi,BYTE PTR [r15+rcx*1]
  64dc26:	shl    esi,0x7
  64dc29:	add    rsi,rax
  64dc2c:	shl    rdx,0x4
  64dc30:	test   cl,0x1
  64dc33:	je     64db70 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0xeb0>
  64dc39:	vpshufd xmm8,XMMWORD PTR [rsi+rdx*1],0x4e
  64dc3f:	vpxor  xmm0,xmm8,xmm0
  64dc43:	vmovdqa XMMWORD PTR [rsp+0x180],xmm0
  64dc4c:	mov    rdi,rdx
  64dc4f:	xor    rdi,0x10
  64dc53:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  64dc59:	vpxor  xmm1,xmm8,xmm1
  64dc5d:	vmovdqa XMMWORD PTR [rsp+0x190],xmm1
  64dc66:	mov    rdi,rdx
  64dc69:	xor    rdi,0x20
  64dc6d:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  64dc73:	vpxor  xmm2,xmm8,xmm2
  64dc77:	vmovdqa XMMWORD PTR [rsp+0x1a0],xmm2
  64dc80:	mov    rdi,rdx
  64dc83:	xor    rdi,0x30
  64dc87:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  64dc8d:	vpxor  xmm3,xmm8,xmm3
  64dc91:	vmovdqa XMMWORD PTR [rsp+0x1b0],xmm3
  64dc9a:	mov    rdi,rdx
  64dc9d:	xor    rdi,0x40
  64dca1:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  64dca7:	vpxor  xmm4,xmm8,xmm4
  64dcab:	vmovdqa XMMWORD PTR [rsp+0x1c0],xmm4
  64dcb4:	mov    rdi,rdx
  64dcb7:	xor    rdi,0x50
  64dcbb:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  64dcc1:	vpxor  xmm5,xmm8,xmm5
  64dcc5:	vmovdqa XMMWORD PTR [rsp+0x1d0],xmm5
  64dcce:	mov    rdi,rdx
  64dcd1:	xor    rdi,0x60
  64dcd5:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  64dcdb:	vpxor  xmm6,xmm8,xmm6
  64dcdf:	vmovdqa XMMWORD PTR [rsp+0x1e0],xmm6
  64dce8:	xor    rdx,0x70
  64dcec:	vpshufd xmm8,XMMWORD PTR [rsi+rdx*1],0x4e
  64dcf2:	vpxor  xmm7,xmm8,xmm7
  64dcf6:	vmovdqa XMMWORD PTR [rsp+0x1f0],xmm7
  64dcff:	inc    rcx
  64dd02:	cmp    rcx,0x10
  64dd06:	jne    64dc1b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0xf5b>
  64dd0c:	jmp    64dd66 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x10a6>
  64dd0e:	xchg   ax,ax
  64dd10:	mov    edx,0x10
  64dd15:	mov    r8d,0x80
  64dd1b:	lea    rdi,[rip+0x9c53e]        # 6ea260 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.10381458960231317189+0x1000>
  64dd22:	mov    rsi,rbx
  64dd25:	lea    rcx,[rsp+0x180]
  64dd2d:	vzeroupper 
  64dd30:	call   QWORD PTR [rip+0x998fa]        # 6e7630 <_DYNAMIC+0x660>
  64dd36:	jmp    64dd66 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x10a6>
  64dd38:	nop    DWORD PTR [rax+rax*1+0x0]
  64dd40:	mov    edx,0x10
  64dd45:	mov    r8d,0x80
  64dd4b:	lea    rdi,[rip+0x9c50e]        # 6ea260 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.10381458960231317189+0x1000>
  64dd52:	mov    rsi,rbx
  64dd55:	lea    rcx,[rsp+0x180]
  64dd5d:	vzeroupper 
  64dd60:	call   QWORD PTR [rip+0x998c2]        # 6e7628 <_DYNAMIC+0x658>
  64dd66:	mov    rdi,QWORD PTR [rip+0x9b3d3]        # 6e9140 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.10381458960231317189+0x18>
  64dd6d:	cmp    rdi,0xf
  64dd71:	jbe    64efee <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x232e>
  64dd77:	mov    rax,QWORD PTR [rip+0x9b3ba]        # 6e9138 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.10381458960231317189+0x10>
  64dd7e:	mov    rdx,r14
  64dd81:	mov    rcx,r14
  64dd84:	mov    rsi,r14
  64dd87:	mov    rdi,r14
  64dd8a:	mov    r8d,r14d
  64dd8d:	shr    r8d,0x4
  64dd91:	and    r8d,0xff0
  64dd98:	vmovdqa xmm0,XMMWORD PTR [rax+r8*1+0x1000]
  64dda2:	movzx  r8d,r14b
  64dda6:	shl    r8d,0x4
  64ddaa:	vpxor  xmm0,xmm0,XMMWORD PTR [rax+r8*1]
  64ddb0:	mov    r8d,r14d
  64ddb3:	shr    r14d,0x14
  64ddb7:	and    r14d,0xfffffff0
  64ddbb:	vmovdqa xmm1,XMMWORD PTR [rax+r14*1+0x3000]
  64ddc5:	shr    rdx,0x38
  64ddc9:	shr    rsi,0x28
  64ddcd:	shr    r8d,0xc
  64ddd1:	and    r8d,0xff0
  64ddd8:	vpternlogq xmm1,xmm0,XMMWORD PTR [rax+r8*1+0x2000],0x96
  64dde4:	shr    rdi,0x20
  64dde8:	movzx  edi,dil
  64ddec:	movzx  esi,sil
  64ddf0:	shl    esi,0x4
  64ddf3:	vmovdqa xmm0,XMMWORD PTR [rax+rsi*1+0x5000]
  64ddfc:	shl    edi,0x4
  64ddff:	vpternlogq xmm0,xmm1,XMMWORD PTR [rax+rdi*1+0x4000],0x96
  64de0b:	shl    edx,0x4
  64de0e:	vmovdqa xmm1,XMMWORD PTR [rax+rdx*1+0x7000]
  64de17:	mov    rsi,r13
  64de1a:	mov    rdx,r13
  64de1d:	mov    r8,r13
  64de20:	mov    rdi,r13
  64de23:	shr    rcx,0x30
  64de27:	movzx  ecx,cl
  64de2a:	shl    ecx,0x4
  64de2d:	vpternlogq xmm1,xmm0,XMMWORD PTR [rax+rcx*1+0x6000],0x96
  64de39:	mov    ecx,r13d
  64de3c:	shr    ecx,0x4
  64de3f:	and    ecx,0xff0
  64de45:	vmovdqa xmm0,XMMWORD PTR [rax+rcx*1+0x9000]
  64de4e:	movzx  ecx,r13b
  64de52:	shl    ecx,0x4
  64de55:	vpternlogq xmm0,xmm1,XMMWORD PTR [rax+rcx*1+0x8000],0x96
  64de61:	mov    ecx,r13d
  64de64:	shr    r13d,0x14
  64de68:	and    r13d,0xfffffff0
  64de6c:	vmovdqa xmm1,XMMWORD PTR [rax+r13*1+0xb000]
  64de76:	shr    ecx,0xc
  64de79:	and    ecx,0xff0
  64de7f:	vpternlogq xmm1,xmm0,XMMWORD PTR [rax+rcx*1+0xa000],0x96
  64de8b:	shr    r8,0x28
  64de8f:	movzx  ecx,r8b
  64de93:	shl    ecx,0x4
  64de96:	vmovdqa xmm0,XMMWORD PTR [rax+rcx*1+0xd000]
  64de9f:	shr    rdi,0x20
  64dea3:	movzx  ecx,dil
  64dea7:	shl    ecx,0x4
  64deaa:	vpternlogq xmm0,xmm1,XMMWORD PTR [rax+rcx*1+0xc000],0x96
  64deb6:	shr    rsi,0x38
  64deba:	shl    esi,0x4
  64debd:	vmovdqa xmm2,XMMWORD PTR [rax+rsi*1+0xf000]
  64dec6:	shr    rdx,0x30
  64deca:	movzx  ecx,dl
  64decd:	shl    ecx,0x4
  64ded0:	vpternlogq xmm2,xmm0,XMMWORD PTR [rax+rcx*1+0xe000],0x96
  64dedc:	mov    rax,QWORD PTR [rbx]
  64dedf:	mov    rcx,QWORD PTR [rbx+0x8]
  64dee3:	and    rax,QWORD PTR [r12]
  64dee7:	and    rcx,QWORD PTR [r12+0x8]
  64deec:	mov    rbx,QWORD PTR [rsp+0x8]
  64def1:	inc    rbx
  64def4:	vmovdqa64 zmm0,ZMMWORD PTR [rip+0xffffffffff9c3002]        # 10f00 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xf2>
  64defe:	vpandq zmm1,zmm0,ZMMWORD PTR [rsp+0x1c0]
  64df06:	vpandq zmm3,zmm0,ZMMWORD PTR [rsp+0x5e0]
  64df11:	vmovdqu64 zmm0,ZMMWORD PTR [rsp+0x620]
  64df1c:	vgf2p8mulb zmm0,zmm0,ZMMWORD PTR [rsp+0x180]
  64df24:	vgf2p8mulb zmm1,zmm3,zmm1
  64df2a:	lea    rdx,[rsp+0x1b0]
  64df32:	vpxor  xmm3,xmm3,xmm3
  64df36:	vmovdqu XMMWORD PTR [rdx],xmm3
  64df3a:	vmovdqu XMMWORD PTR [rsp+0x1a0],xmm2
  64df43:	kmovq  k1,rax
  64df48:	kmovq  k2,rcx
  64df4d:	lea    rax,[rsp+0x1a1]
  64df55:	xor    ecx,ecx
  64df57:	nop    WORD PTR [rax+rax*1+0x0]
  64df60:	vpbroadcastb zmm2,BYTE PTR [rax-0x1]
  64df67:	vmovdqu8 zmm3{k1}{z},zmm2
  64df6d:	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x7e0]
  64df78:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x7e0],zmm3
  64df83:	vmovdqu8 zmm3{k2}{z},zmm2
  64df89:	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x820]
  64df94:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x820],zmm3
  64df9f:	vgf2p8mulb zmm3,zmm0,zmm2
  64dfa5:	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x860]
  64dfb0:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x860],zmm3
  64dfbb:	vgf2p8mulb zmm2,zmm1,zmm2
  64dfc1:	vpxord zmm2,zmm2,ZMMWORD PTR [rsp+rcx*1+0x8a0]
  64dfcc:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x8a0],zmm2
  64dfd7:	vpbroadcastb zmm2,BYTE PTR [rax]
  64dfdd:	vmovdqu8 zmm3{k1}{z},zmm2
  64dfe3:	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x8e0]
  64dfee:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x8e0],zmm3
  64dff9:	vmovdqu8 zmm3{k2}{z},zmm2
  64dfff:	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x920]
  64e00a:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x920],zmm3
  64e015:	vgf2p8mulb zmm3,zmm0,zmm2
  64e01b:	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x960]
  64e026:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x960],zmm3
  64e031:	vgf2p8mulb zmm2,zmm1,zmm2
  64e037:	vpxord zmm2,zmm2,ZMMWORD PTR [rsp+rcx*1+0x9a0]
  64e042:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x9a0],zmm2
  64e04d:	add    rcx,0x200
  64e054:	add    rax,0x2
  64e058:	cmp    rcx,0x1000
  64e05f:	jne    64df60 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x12a0>
  64e065:	add    rbp,0x10
  64e069:	add    r15,0x10
  64e06d:	cmp    rbx,QWORD PTR [rsp+0x50]
  64e072:	jne    64d6d0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0xa10>
  64e078:	mov    edi,0xff0
  64e07d:	vzeroupper 
  64e080:	call   QWORD PTR [rip+0x99142]        # 6e71c8 <malloc@GLIBC_2.2.5>
  64e086:	test   rax,rax
  64e089:	je     64f05d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x239d>
  64e08f:	mov    r15,rax
  64e092:	mov    eax,0xf
  64e097:	lea    rcx,[rsp+0x16e0]
  64e09f:	nop
  64e0a0:	movzx  edx,BYTE PTR [rcx-0xf00]
  64e0a7:	mov    BYTE PTR [rsp+0x620],dl
  64e0ae:	movzx  esi,BYTE PTR [rcx-0xe00]
  64e0b5:	movzx  edi,BYTE PTR [rcx-0xd00]
  64e0bc:	movzx  r8d,BYTE PTR [rcx-0xc00]
  64e0c4:	movzx  r9d,BYTE PTR [rcx-0xb00]
  64e0cc:	movzx  r10d,BYTE PTR [rcx-0xa00]
  64e0d4:	movzx  r11d,BYTE PTR [rcx-0x900]
  64e0dc:	movzx  ebx,BYTE PTR [rcx-0x800]
  64e0e3:	movzx  ebp,BYTE PTR [rcx-0x700]
  64e0ea:	movzx  r14d,BYTE PTR [rcx-0x600]
  64e0f2:	movzx  r12d,BYTE PTR [rcx-0x500]
  64e0fa:	movzx  r13d,BYTE PTR [rcx-0x400]
  64e102:	movzx  edx,BYTE PTR [rcx-0x300]
  64e109:	mov    BYTE PTR [rsp+0x8],dl
  64e10d:	movzx  edx,BYTE PTR [rcx-0x200]
  64e114:	mov    BYTE PTR [rsp+0x5e0],dl
  64e11b:	movzx  edx,BYTE PTR [rcx-0x100]
  64e122:	mov    BYTE PTR [rsp+0x78],dl
  64e126:	movzx  edx,BYTE PTR [rcx]
  64e129:	mov    BYTE PTR [rsp+0x70],dl
  64e12d:	movzx  edx,BYTE PTR [rsp+0x620]
  64e135:	mov    BYTE PTR [r15+rax*1-0xf],dl
  64e13a:	mov    BYTE PTR [r15+rax*1-0xe],sil
  64e13f:	mov    BYTE PTR [r15+rax*1-0xd],dil
  64e144:	mov    BYTE PTR [r15+rax*1-0xc],r8b
  64e149:	mov    BYTE PTR [r15+rax*1-0xb],r9b
  64e14e:	mov    BYTE PTR [r15+rax*1-0xa],r10b
  64e153:	mov    BYTE PTR [r15+rax*1-0x9],r11b
  64e158:	mov    BYTE PTR [r15+rax*1-0x8],bl
  64e15d:	mov    BYTE PTR [r15+rax*1-0x7],bpl
  64e162:	mov    BYTE PTR [r15+rax*1-0x6],r14b
  64e167:	mov    BYTE PTR [r15+rax*1-0x5],r12b
  64e16c:	mov    BYTE PTR [r15+rax*1-0x4],r13b
  64e171:	movzx  edx,BYTE PTR [rsp+0x8]
  64e176:	mov    BYTE PTR [r15+rax*1-0x3],dl
  64e17b:	movzx  edx,BYTE PTR [rsp+0x5e0]
  64e183:	mov    BYTE PTR [r15+rax*1-0x2],dl
  64e188:	movzx  edx,BYTE PTR [rsp+0x78]
  64e18d:	mov    BYTE PTR [r15+rax*1-0x1],dl
  64e192:	movzx  edx,BYTE PTR [rsp+0x70]
  64e197:	mov    BYTE PTR [r15+rax*1],dl
  64e19b:	add    rax,0x10
  64e19f:	inc    rcx
  64e1a2:	cmp    rax,0xfff
  64e1a8:	jne    64e0a0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x13e0>
  64e1ae:	jmp    64cf20 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x260>
  64e1b3:	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  64e1c0:	mov    eax,DWORD PTR [rip+0x9b092]        # 6e9258 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.10381458960231317189+0x130>
  64e1c6:	test   eax,eax
  64e1c8:	jne    64ebe0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1f20>
  64e1ce:	mov    eax,DWORD PTR [rip+0x9c1ac]        # 6ea380 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.10381458960231317189+0x1120>
  64e1d4:	test   eax,eax
  64e1d6:	jne    64ebea <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1f2a>
  64e1dc:	mov    edi,0xff0
  64e1e1:	mov    esi,0x1
  64e1e6:	call   QWORD PTR [rip+0x99004]        # 6e71f0 <calloc@GLIBC_2.2.5>
  64e1ec:	test   rax,rax
  64e1ef:	je     64f032 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2372>
  64e1f5:	mov    rbp,rax
  64e1f8:	mov    edx,0x1000
  64e1fd:	lea    rdi,[rsp+0x7e0]
  64e205:	xor    esi,esi
  64e207:	call   QWORD PTR [rip+0x98feb]        # 6e71f8 <memset@GLIBC_2.2.5>
  64e20d:	shr    r14,0x4
  64e211:	shr    r13,0x4
  64e215:	cmp    r13,r14
  64e218:	cmovae r13,r14
  64e21c:	cmp    r15,r13
  64e21f:	cmovb  r13,r15
  64e223:	mov    QWORD PTR [rsp+0x50],r13
  64e228:	test   r13,r13
  64e22b:	mov    QWORD PTR [rsp],rbp
  64e22f:	je     64eb6b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1eab>
  64e235:	xor    ebx,ebx
  64e237:	nop    WORD PTR [rax+rax*1+0x0]
  64e240:	mov    QWORD PTR [rsp+0xb8],rbx
  64e248:	shl    rbx,0x4
  64e24c:	mov    rax,QWORD PTR [rsp+0x68]
  64e251:	lea    rsi,[rax+rbx*1]
  64e255:	mov    rax,QWORD PTR [rsp+0x60]
  64e25a:	mov    r15,QWORD PTR [rax+rbx*1]
  64e25e:	mov    r14,QWORD PTR [rax+rbx*1+0x8]
  64e263:	lea    rdi,[rsp+0x6e0]
  64e26b:	vzeroupper 
  64e26e:	call   6585d0 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate6domainNtB2_6Domain8eval_row>
  64e273:	add    rbx,QWORD PTR [rsp+0x58]
  64e278:	lea    rdi,[rsp+0x4e0]
  64e280:	mov    rsi,rbx
  64e283:	call   6585d0 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate6domainNtB2_6Domain8eval_row>
  64e288:	mov    rax,QWORD PTR [rip+0x9aeb1]        # 6e9140 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.10381458960231317189+0x18>
  64e28f:	cmp    rax,0x10
  64e293:	jb     64f003 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2343>
  64e299:	inc    QWORD PTR [rsp+0xb8]
  64e2a1:	mov    rax,r14
  64e2a4:	shr    rax,0x38
  64e2a8:	mov    rcx,r14
  64e2ab:	shr    rcx,0x30
  64e2af:	mov    rsi,r14
  64e2b2:	shr    rsi,0x28
  64e2b6:	mov    rdi,r14
  64e2b9:	mov    r8,r15
  64e2bc:	shr    r8,0x38
  64e2c0:	mov    r9,r15
  64e2c3:	shr    r9,0x30
  64e2c7:	mov    r10,r15
  64e2ca:	shr    r10,0x28
  64e2ce:	mov    r11,r15
  64e2d1:	mov    rdx,QWORD PTR [rip+0x9ae60]        # 6e9138 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.10381458960231317189+0x10>
  64e2d8:	movzx  ebx,r15b
  64e2dc:	shl    ebx,0x4
  64e2df:	mov    r12d,r15d
  64e2e2:	shr    r12d,0x4
  64e2e6:	and    r12d,0xff0
  64e2ed:	vmovdqa xmm0,XMMWORD PTR [rdx+r12*1+0x1000]
  64e2f7:	vpxor  xmm0,xmm0,XMMWORD PTR [rdx+rbx*1]
  64e2fc:	mov    ebx,r15d
  64e2ff:	shr    ebx,0xc
  64e302:	and    ebx,0xff0
  64e308:	shr    r15d,0x14
  64e30c:	and    r15d,0xfffffff0
  64e310:	vmovdqa xmm1,XMMWORD PTR [rdx+r15*1+0x3000]
  64e31a:	vpternlogq xmm1,xmm0,XMMWORD PTR [rdx+rbx*1+0x2000],0x96
  64e326:	shr    r11,0x20
  64e32a:	movzx  r11d,r11b
  64e32e:	movzx  r10d,r10b
  64e332:	shl    r10d,0x4
  64e336:	vmovdqa xmm0,XMMWORD PTR [rdx+r10*1+0x5000]
  64e340:	shl    r11d,0x4
  64e344:	vpternlogq xmm0,xmm1,XMMWORD PTR [rdx+r11*1+0x4000],0x96
  64e350:	movzx  r9d,r9b
  64e354:	shl    r9d,0x4
  64e358:	shl    r8d,0x4
  64e35c:	vmovdqa xmm1,XMMWORD PTR [rdx+r8*1+0x7000]
  64e366:	vpternlogq xmm1,xmm0,XMMWORD PTR [rdx+r9*1+0x6000],0x96
  64e372:	movzx  r8d,r14b
  64e376:	shl    r8d,0x4
  64e37a:	mov    r9d,r14d
  64e37d:	shr    r9d,0x4
  64e381:	and    r9d,0xff0
  64e388:	vmovdqa xmm0,XMMWORD PTR [rdx+r9*1+0x9000]
  64e392:	vpternlogq xmm0,xmm1,XMMWORD PTR [rdx+r8*1+0x8000],0x96
  64e39e:	shr    rdi,0x20
  64e3a2:	mov    r8d,r14d
  64e3a5:	shr    r8d,0xc
  64e3a9:	shr    r14d,0x14
  64e3ad:	and    r14d,0xfffffff0
  64e3b1:	vmovdqa xmm1,XMMWORD PTR [rdx+r14*1+0xb000]
  64e3bb:	and    r8d,0xff0
  64e3c2:	vpternlogq xmm1,xmm0,XMMWORD PTR [rdx+r8*1+0xa000],0x96
  64e3ce:	movzx  edi,dil
  64e3d2:	shl    edi,0x4
  64e3d5:	movzx  esi,sil
  64e3d9:	shl    esi,0x4
  64e3dc:	vmovdqa xmm0,XMMWORD PTR [rdx+rsi*1+0xd000]
  64e3e5:	vpternlogq xmm0,xmm1,XMMWORD PTR [rdx+rdi*1+0xc000],0x96
  64e3f1:	movzx  ecx,cl
  64e3f4:	shl    ecx,0x4
  64e3f7:	shl    eax,0x4
  64e3fa:	vmovdqa xmm1,XMMWORD PTR [rdx+rax*1+0xf000]
  64e403:	vpternlogq xmm1,xmm0,XMMWORD PTR [rdx+rcx*1+0xe000],0x96
  64e40f:	vpextrq rcx,xmm1,0x1
  64e415:	vmovq  rdx,xmm1
  64e41a:	mov    eax,edx
  64e41c:	shl    eax,0x8
  64e41f:	movzx  eax,ax
  64e422:	mov    QWORD PTR [rsp+0x160],rax
  64e42a:	mov    eax,edx
  64e42c:	and    eax,0xff00
  64e431:	mov    QWORD PTR [rsp+0x158],rax
  64e439:	mov    eax,edx
  64e43b:	shr    eax,0x8
  64e43e:	and    eax,0xff00
  64e443:	mov    QWORD PTR [rsp+0x150],rax
  64e44b:	mov    eax,edx
  64e44d:	shr    eax,0x10
  64e450:	and    eax,0xff00
  64e455:	mov    QWORD PTR [rsp+0x148],rax
  64e45d:	mov    rax,rdx
  64e460:	shr    rax,0x18
  64e464:	and    eax,0xff00
  64e469:	mov    QWORD PTR [rsp+0x140],rax
  64e471:	mov    rax,rdx
  64e474:	shr    rax,0x20
  64e478:	and    eax,0xff00
  64e47d:	mov    QWORD PTR [rsp+0x138],rax
  64e485:	mov    rax,rdx
  64e488:	shr    rax,0x28
  64e48c:	and    eax,0xff00
  64e491:	mov    QWORD PTR [rsp+0x130],rax
  64e499:	shr    rdx,0x30
  64e49d:	and    edx,0xff00
  64e4a3:	mov    QWORD PTR [rsp+0x128],rdx
  64e4ab:	mov    eax,ecx
  64e4ad:	shl    eax,0x8
  64e4b0:	movzx  eax,ax
  64e4b3:	mov    QWORD PTR [rsp+0x120],rax
  64e4bb:	mov    eax,ecx
  64e4bd:	and    eax,0xff00
  64e4c2:	mov    QWORD PTR [rsp+0x118],rax
  64e4ca:	mov    eax,ecx
  64e4cc:	shr    eax,0x8
  64e4cf:	and    eax,0xff00
  64e4d4:	mov    QWORD PTR [rsp+0x110],rax
  64e4dc:	mov    eax,ecx
  64e4de:	shr    eax,0x10
  64e4e1:	and    eax,0xff00
  64e4e6:	mov    QWORD PTR [rsp+0x108],rax
  64e4ee:	mov    rax,rcx
  64e4f1:	shr    rax,0x18
  64e4f5:	and    eax,0xff00
  64e4fa:	mov    QWORD PTR [rsp+0x100],rax
  64e502:	mov    rax,rcx
  64e505:	shr    rax,0x20
  64e509:	and    eax,0xff00
  64e50e:	mov    QWORD PTR [rsp+0xf8],rax
  64e516:	mov    rax,rcx
  64e519:	shr    rax,0x28
  64e51d:	and    eax,0xff00
  64e522:	mov    QWORD PTR [rsp+0xf0],rax
  64e52a:	shr    rcx,0x30
  64e52e:	and    ecx,0xff00
  64e534:	mov    QWORD PTR [rsp+0xe8],rcx
  64e53c:	mov    rax,QWORD PTR [rip+0x9abed]        # 6e9130 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.10381458960231317189+0x8>
  64e543:	mov    QWORD PTR [rsp+0x8],rax
  64e548:	mov    r11,QWORD PTR [rip+0x9abd9]        # 6e9128 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.10381458960231317189>
  64e54f:	lea    r12,[rsp+0x180]
  64e557:	xor    ecx,ecx
  64e559:	mov    QWORD PTR [rsp+0xe0],r11
  64e561:	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  64e570:	mov    al,0x1
  64e572:	shl    al,cl
  64e574:	movzx  r10d,al
  64e578:	mov    rax,QWORD PTR [rsp+0x160]
  64e580:	or     rax,r10
  64e583:	mov    rsi,QWORD PTR [rsp+0x8]
  64e588:	cmp    rax,rsi
  64e58b:	jae    64f01b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x235b>
  64e591:	mov    r14,QWORD PTR [rsp+0x158]
  64e599:	or     r14,r10
  64e59c:	cmp    r14,rsi
  64e59f:	mov    rbp,QWORD PTR [rsp]
  64e5a3:	jae    64ef24 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2264>
  64e5a9:	mov    rdi,QWORD PTR [rsp+0x150]
  64e5b1:	or     rdi,r10
  64e5b4:	cmp    rdi,rsi
  64e5b7:	jae    64ef4c <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x228c>
  64e5bd:	mov    r13,QWORD PTR [rsp+0x148]
  64e5c5:	or     r13,r10
  64e5c8:	cmp    r13,rsi
  64e5cb:	jae    64eef3 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2233>
  64e5d1:	mov    r15,QWORD PTR [rsp+0x140]
  64e5d9:	or     r15,r10
  64e5dc:	cmp    r15,rsi
  64e5df:	jae    64ef54 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2294>
  64e5e5:	mov    rbx,QWORD PTR [rsp+0x138]
  64e5ed:	or     rbx,r10
  64e5f0:	cmp    rbx,rsi
  64e5f3:	jae    64ef34 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2274>
  64e5f9:	mov    r8,QWORD PTR [rsp+0x130]
  64e601:	or     r8,r10
  64e604:	cmp    r8,rsi
  64e607:	jae    64eefb <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x223b>
  64e60d:	mov    r9,QWORD PTR [rsp+0x128]
  64e615:	or     r9,r10
  64e618:	cmp    r9,rsi
  64e61b:	jae    64ef1c <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x225c>
  64e621:	mov    rdx,QWORD PTR [rsp+0x120]
  64e629:	or     rdx,r10
  64e62c:	cmp    rdx,rsi
  64e62f:	jae    64ef2c <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x226c>
  64e635:	mov    QWORD PTR [rsp+0x620],r13
  64e63d:	mov    r13,QWORD PTR [rsp+0x118]
  64e645:	or     r13,r10
  64e648:	cmp    r13,rsi
  64e64b:	jae    64eef3 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2233>
  64e651:	mov    r11,QWORD PTR [rsp+0x110]
  64e659:	or     r11,r10
  64e65c:	cmp    r11,rsi
  64e65f:	jae    64ef3c <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x227c>
  64e665:	mov    QWORD PTR [rsp+0x78],rdi
  64e66a:	mov    QWORD PTR [rsp+0x5e0],rcx
  64e672:	mov    rdi,r12
  64e675:	mov    r12,QWORD PTR [rsp+0x108]
  64e67d:	or     r12,r10
  64e680:	cmp    r12,rsi
  64e683:	jae    64ef44 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2284>
  64e689:	mov    rbp,QWORD PTR [rsp+0x100]
  64e691:	or     rbp,r10
  64e694:	cmp    rbp,rsi
  64e697:	jae    64ef03 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2243>
  64e69d:	mov    rcx,QWORD PTR [rsp+0xf8]
  64e6a5:	or     rcx,r10
  64e6a8:	cmp    rcx,rsi
  64e6ab:	jae    64f018 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2358>
  64e6b1:	mov    QWORD PTR [rsp+0x70],rax
  64e6b6:	mov    rax,QWORD PTR [rsp+0xf0]
  64e6be:	or     rax,r10
  64e6c1:	cmp    rax,rsi
  64e6c4:	jae    64f01b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x235b>
  64e6ca:	or     r10,QWORD PTR [rsp+0xe8]
  64e6d2:	cmp    r10,rsi
  64e6d5:	jae    64ef0b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x224b>
  64e6db:	mov    rsi,QWORD PTR [rsp+0xe0]
  64e6e3:	movzx  r13d,BYTE PTR [rsi+r13*1]
  64e6e8:	shl    r13d,0x8
  64e6ec:	movzx  edx,BYTE PTR [rsi+rdx*1]
  64e6f0:	or     rdx,r13
  64e6f3:	movzx  r11d,BYTE PTR [rsi+r11*1]
  64e6f8:	shl    r11d,0x10
  64e6fc:	or     rdx,r11
  64e6ff:	movzx  r11d,BYTE PTR [rsi+r12*1]
  64e704:	shl    r11d,0x18
  64e708:	or     rdx,r11
  64e70b:	movzx  r11d,BYTE PTR [rsi+rbp*1]
  64e710:	shl    r11,0x20
  64e714:	or     rdx,r11
  64e717:	movzx  ecx,BYTE PTR [rsi+rcx*1]
  64e71b:	shl    rcx,0x28
  64e71f:	or     rdx,rcx
  64e722:	movzx  eax,BYTE PTR [rsi+rax*1]
  64e726:	shl    rax,0x30
  64e72a:	or     rdx,rax
  64e72d:	movzx  eax,BYTE PTR [rsi+r10*1]
  64e732:	shl    rax,0x38
  64e736:	or     rdx,rax
  64e739:	movzx  eax,BYTE PTR [rsi+r14*1]
  64e73e:	shl    eax,0x8
  64e741:	mov    rcx,QWORD PTR [rsp+0x70]
  64e746:	movzx  ecx,BYTE PTR [rsi+rcx*1]
  64e74a:	or     rcx,rax
  64e74d:	mov    rax,QWORD PTR [rsp+0x78]
  64e752:	movzx  eax,BYTE PTR [rsi+rax*1]
  64e756:	shl    eax,0x10
  64e759:	or     rcx,rax
  64e75c:	mov    rax,QWORD PTR [rsp+0x620]
  64e764:	movzx  eax,BYTE PTR [rsi+rax*1]
  64e768:	shl    eax,0x18
  64e76b:	or     rcx,rax
  64e76e:	movzx  eax,BYTE PTR [rsi+r15*1]
  64e773:	shl    rax,0x20
  64e777:	or     rcx,rax
  64e77a:	movzx  eax,BYTE PTR [rsi+rbx*1]
  64e77e:	shl    rax,0x28
  64e782:	or     rcx,rax
  64e785:	movzx  eax,BYTE PTR [rsi+r8*1]
  64e78a:	shl    rax,0x30
  64e78e:	or     rcx,rax
  64e791:	movzx  eax,BYTE PTR [rsi+r9*1]
  64e796:	shl    rax,0x38
  64e79a:	or     rcx,rax
  64e79d:	mov    QWORD PTR [rdi+0x8],rdx
  64e7a1:	mov    QWORD PTR [rdi],rcx
  64e7a4:	mov    rcx,QWORD PTR [rsp+0x5e0]
  64e7ac:	inc    rcx
  64e7af:	add    rdi,0x10
  64e7b3:	cmp    rcx,0x8
  64e7b7:	mov    r12,rdi
  64e7ba:	jne    64e570 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x18b0>
  64e7c0:	mov    r11,rsi
  64e7c3:	vmovups ymm0,YMMWORD PTR [rsp+0x180]
  64e7cc:	vmovups ymm1,YMMWORD PTR [rsp+0x1a0]
  64e7d5:	vmovups ymm2,YMMWORD PTR [rsp+0x1c0]
  64e7de:	vmovdqu ymm3,YMMWORD PTR [rsp+0x1e0]
  64e7e7:	vmovdqu YMMWORD PTR [rsp+0x6c0],ymm3
  64e7f0:	vmovups YMMWORD PTR [rsp+0x6a0],ymm2
  64e7f9:	vmovups YMMWORD PTR [rsp+0x680],ymm1
  64e802:	vmovups YMMWORD PTR [rsp+0x660],ymm0
  64e80b:	lea    rcx,[rsp+0x810]
  64e813:	xor    edx,edx
  64e815:	mov    r12,QWORD PTR [rsp+0x48]
  64e81a:	movabs r9,0x8000000000000000
  64e824:	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  64e830:	lea    rsi,[rdx+0x1]
  64e834:	lea    rax,[rdx+0x2]
  64e838:	lea    rdi,[rdx+r9*1]
  64e83c:	and    rdi,rsi
  64e83f:	shl    rdi,0x4
  64e843:	tzcnt  r8,rsi
  64e848:	shl    r8d,0x4
  64e84c:	vmovaps xmm0,XMMWORD PTR [rsp+r8*1+0x660]
  64e856:	vxorps xmm0,xmm0,XMMWORD PTR [rsp+rdi*1+0x7e0]
  64e85f:	vmovaps XMMWORD PTR [rcx-0x20],xmm0
  64e864:	and    rsi,rax
  64e867:	shl    rsi,0x4
  64e86b:	vmovq  xmm0,QWORD PTR [rsp+rsi*1+0x7e8]
  64e874:	add    rdx,0x3
  64e878:	vmovq  xmm1,QWORD PTR [rsp+rsi*1+0x7e0]
  64e881:	vpunpcklqdq xmm0,xmm1,xmm0
  64e885:	tzcnt  rsi,rax
  64e88a:	shl    esi,0x4
  64e88d:	vmovq  xmm1,QWORD PTR [rsp+rsi*1+0x668]
  64e896:	vmovq  xmm2,QWORD PTR [rsp+rsi*1+0x660]
  64e89f:	vpunpcklqdq xmm1,xmm2,xmm1
  64e8a3:	vpxor  xmm0,xmm1,xmm0
  64e8a7:	vpextrq QWORD PTR [rcx-0x8],xmm0,0x1
  64e8ae:	vmovq  QWORD PTR [rcx-0x10],xmm0
  64e8b3:	and    rax,rdx
  64e8b6:	shl    rax,0x4
  64e8ba:	vmovq  xmm0,QWORD PTR [rsp+rax*1+0x7e8]
  64e8c3:	vmovq  xmm1,QWORD PTR [rsp+rax*1+0x7e0]
  64e8cc:	vpunpcklqdq xmm0,xmm1,xmm0
  64e8d0:	tzcnt  rax,rdx
  64e8d5:	shl    eax,0x4
  64e8d8:	vmovq  xmm1,QWORD PTR [rsp+rax*1+0x668]
  64e8e1:	vmovq  xmm2,QWORD PTR [rsp+rax*1+0x660]
  64e8ea:	vpunpcklqdq xmm1,xmm2,xmm1
  64e8ee:	vpxor  xmm0,xmm1,xmm0
  64e8f2:	vmovdqa XMMWORD PTR [rcx],xmm0
  64e8f6:	add    rcx,0x30
  64e8fa:	cmp    rdx,0xff
  64e901:	jne    64e830 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1b70>
  64e907:	vmovups ymm0,YMMWORD PTR [rsp+0x7bf]
  64e910:	lea    rcx,[rsp+0x1b0]
  64e918:	vmovups YMMWORD PTR [rcx+0xcf],ymm0
  64e920:	vmovups ymm0,YMMWORD PTR [rsp+0x7a0]
  64e929:	vmovups YMMWORD PTR [rcx+0xb0],ymm0
  64e931:	vmovups ymm0,YMMWORD PTR [rsp+0x780]
  64e93a:	vmovups YMMWORD PTR [rcx+0x90],ymm0
  64e942:	vmovups ymm0,YMMWORD PTR [rsp+0x760]
  64e94b:	vmovups YMMWORD PTR [rcx+0x70],ymm0
  64e950:	vmovups ymm0,YMMWORD PTR [rsp+0x6e0]
  64e959:	vmovups ymm1,YMMWORD PTR [rsp+0x700]
  64e962:	vmovups ymm2,YMMWORD PTR [rsp+0x720]
  64e96b:	vmovups ymm3,YMMWORD PTR [rsp+0x740]
  64e974:	vmovups YMMWORD PTR [rcx+0x50],ymm3
  64e979:	vmovups YMMWORD PTR [rcx+0x30],ymm2
  64e97e:	vmovups YMMWORD PTR [rcx+0x10],ymm1
  64e983:	vmovups YMMWORD PTR [rcx-0x10],ymm0
  64e988:	lea    rax,[rsp+0x3e8]
  64e990:	vpxor  xmm4,xmm4,xmm4
  64e994:	vmovdqu XMMWORD PTR [rax],xmm4
  64e998:	vmovups ymm0,YMMWORD PTR [rsp+0x5bf]
  64e9a1:	vmovups YMMWORD PTR [rcx+0x1ef],ymm0
  64e9a9:	vmovups ymm0,YMMWORD PTR [rsp+0x5a0]
  64e9b2:	vmovups YMMWORD PTR [rcx+0x1d0],ymm0
  64e9ba:	vmovups ymm0,YMMWORD PTR [rsp+0x580]
  64e9c3:	vmovups YMMWORD PTR [rcx+0x1b0],ymm0
  64e9cb:	vmovups ymm0,YMMWORD PTR [rsp+0x560]
  64e9d4:	vmovups YMMWORD PTR [rcx+0x190],ymm0
  64e9dc:	vmovups ymm0,YMMWORD PTR [rsp+0x4e0]
  64e9e5:	vmovdqu ymm1,YMMWORD PTR [rsp+0x500]
  64e9ee:	vmovdqu ymm2,YMMWORD PTR [rsp+0x520]
  64e9f7:	vmovdqu ymm3,YMMWORD PTR [rsp+0x540]
  64ea00:	vmovdqu YMMWORD PTR [rcx+0x170],ymm3
  64ea08:	vmovdqu YMMWORD PTR [rcx+0x150],ymm2
  64ea10:	vmovdqu YMMWORD PTR [rcx+0x130],ymm1
  64ea18:	vmovups YMMWORD PTR [rcx+0x110],ymm0
  64ea20:	movzx  eax,BYTE PTR [rsp+0x3f7]
  64ea28:	vmovups xmm0,XMMWORD PTR [rsp+0x3e7]
  64ea31:	vmovups XMMWORD PTR [rcx+0xef],xmm0
  64ea39:	mov    BYTE PTR [rcx+0xff],al
  64ea3f:	lea    rax,[rsp+0x3d7]
  64ea47:	vmovdqu XMMWORD PTR [rax],xmm4
  64ea4b:	mov    QWORD PTR [rsp+0x2b0],0x0
  64ea57:	mov    QWORD PTR [rsp+0x2b8],0xff
  64ea63:	movzx  eax,BYTE PTR [rsp+0x3e6]
  64ea6b:	mov    BYTE PTR [rcx+0x21f],al
  64ea71:	vmovdqu xmm0,XMMWORD PTR [rsp+0x3d6]
  64ea7a:	vmovdqu XMMWORD PTR [rcx+0x20f],xmm0
  64ea82:	lea    rcx,[rsp+0x2c2]
  64ea8a:	xor    edx,edx
  64ea8c:	mov    rbp,QWORD PTR [rsp]
  64ea90:	movzx  edi,BYTE PTR [rcx-0x122]
  64ea97:	movzx  eax,BYTE PTR [rcx-0x2]
  64ea9b:	shl    edi,0x8
  64ea9e:	or     rdi,rax
  64eaa1:	mov    rsi,QWORD PTR [rsp+0x8]
  64eaa6:	cmp    rdi,rsi
  64eaa9:	jae    64eede <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x221e>
  64eaaf:	movzx  eax,BYTE PTR [r11+rdi*1]
  64eab4:	shl    eax,0x4
  64eab7:	vmovdqa xmm0,XMMWORD PTR [rbp+rdx*1+0x0]
  64eabd:	vpxor  xmm0,xmm0,XMMWORD PTR [rsp+rax*1+0x7e0]
  64eac6:	vmovdqa XMMWORD PTR [rbp+rdx*1+0x0],xmm0
  64eacc:	movzx  edi,BYTE PTR [rcx-0x121]
  64ead3:	movzx  eax,BYTE PTR [rcx-0x1]
  64ead7:	shl    edi,0x8
  64eada:	or     rdi,rax
  64eadd:	mov    rsi,QWORD PTR [rsp+0x8]
  64eae2:	cmp    rdi,rsi
  64eae5:	jae    64eede <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x221e>
  64eaeb:	movzx  eax,BYTE PTR [r11+rdi*1]
  64eaf0:	shl    eax,0x4
  64eaf3:	vmovdqa xmm0,XMMWORD PTR [rbp+rdx*1+0x10]
  64eaf9:	vpxor  xmm0,xmm0,XMMWORD PTR [rsp+rax*1+0x7e0]
  64eb02:	vmovdqa XMMWORD PTR [rbp+rdx*1+0x10],xmm0
  64eb08:	movzx  edi,BYTE PTR [rcx-0x120]
  64eb0f:	movzx  eax,BYTE PTR [rcx]
  64eb12:	shl    edi,0x8
  64eb15:	or     rdi,rax
  64eb18:	mov    rsi,QWORD PTR [rsp+0x8]
  64eb1d:	cmp    rdi,rsi
  64eb20:	jae    64eede <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x221e>
  64eb26:	movzx  eax,BYTE PTR [r11+rdi*1]
  64eb2b:	shl    eax,0x4
  64eb2e:	vmovdqa xmm0,XMMWORD PTR [rbp+rdx*1+0x20]
  64eb34:	vpxor  xmm0,xmm0,XMMWORD PTR [rsp+rax*1+0x7e0]
  64eb3d:	vmovdqa XMMWORD PTR [rbp+rdx*1+0x20],xmm0
  64eb43:	add    rdx,0x30
  64eb47:	add    rcx,0x3
  64eb4b:	cmp    rdx,0xff0
  64eb52:	jne    64ea90 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1dd0>
  64eb58:	mov    rbx,QWORD PTR [rsp+0xb8]
  64eb60:	cmp    rbx,QWORD PTR [rsp+0x50]
  64eb65:	jne    64e240 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1580>
  64eb6b:	mov    edi,0xff0
  64eb70:	vzeroupper 
  64eb73:	call   QWORD PTR [rip+0x9864f]        # 6e71c8 <malloc@GLIBC_2.2.5>
  64eb79:	test   rax,rax
  64eb7c:	je     64f06f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x23af>
  64eb82:	mov    r15,rax
  64eb85:	mov    edx,0xff0
  64eb8a:	mov    rdi,rax
  64eb8d:	mov    rsi,rbp
  64eb90:	call   QWORD PTR [rip+0x98652]        # 6e71e8 <memcpy@GLIBC_2.14>
  64eb96:	mov    rdi,rbp
  64eb99:	call   QWORD PTR [rip+0x98639]        # 6e71d8 <free@GLIBC_2.2.5>
  64eb9f:	mov    rax,QWORD PTR [rsp+0x20]
  64eba4:	mov    rcx,QWORD PTR [rsp+0x28]
  64eba9:	mov    rbx,QWORD PTR [rsp+0x10]
  64ebae:	mov    r14,QWORD PTR [rsp+0x30]
  64ebb3:	jmp    64cf39 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x279>
  64ebb8:	call   64a2b4 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.10381458960231317189>
  64ebbd:	jmp    64d088 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3c8>
  64ebc2:	call   64a2b4 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.10381458960231317189>
  64ebc7:	jmp    64d66e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x9ae>
  64ebcc:	call   64a306 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate6domain6DomainE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.10381458960231317189>
  64ebd1:	jmp    64d096 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3d6>
  64ebd6:	call   64a306 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate6domain6DomainE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.10381458960231317189>
  64ebdb:	jmp    64d67c <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x9bc>
  64ebe0:	call   64a2b4 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.10381458960231317189>
  64ebe5:	jmp    64e1ce <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x150e>
  64ebea:	call   64a306 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate6domain6DomainE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.10381458960231317189>
  64ebef:	jmp    64e1dc <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x151c>
  64ebf4:	mov    eax,0xff
  64ebf9:	mov    QWORD PTR [rsp+0x18],rax
  64ebfe:	mov    rax,QWORD PTR [rsp+0xc0]
  64ec06:	mov    QWORD PTR [rax],rsi
  64ec09:	mov    QWORD PTR [rax+0x8],rdi
  64ec0d:	mov    rcx,QWORD PTR [rsp+0x18]
  64ec12:	mov    QWORD PTR [rax+0x10],rcx
  64ec16:	jmp    64ee8c <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x21cc>
  64ec1b:	mov    r14,rsi
  64ec1e:	mov    r12,r9
  64ec21:	mov    r15,rcx
  64ec24:	call   QWORD PTR [rip+0x9896e]        # 6e7598 <_DYNAMIC+0x5c8>
  64ec2a:	mov    rcx,r15
  64ec2d:	mov    r9,r12
  64ec30:	mov    rsi,r14
  64ec33:	mov    rax,QWORD PTR [rax]
  64ec36:	mov    rax,QWORD PTR [rax+0x208]
  64ec3d:	shr    rcx,1
  64ec40:	cmp    rcx,rax
  64ec43:	cmovbe rcx,rax
  64ec47:	mov    QWORD PTR [rsp+0x170],rcx
  64ec4f:	mov    QWORD PTR [rsp+0x4e0],rsi
  64ec57:	mov    rax,QWORD PTR [r9]
  64ec5a:	mov    r14,QWORD PTR [r9+0x8]
  64ec5e:	mov    rcx,QWORD PTR [r9+0x10]
  64ec62:	mov    rdx,QWORD PTR [r9+0x18]
  64ec66:	mov    r11,QWORD PTR [r9+0x20]
  64ec6a:	mov    rdi,QWORD PTR [r9+0x28]
  64ec6e:	mov    r8,QWORD PTR [r9+0x30]
  64ec72:	mov    r10,rcx
  64ec75:	imul   r10,rsi
  64ec79:	cmp    r14,r10
  64ec7c:	cmovb  r10,r14
  64ec80:	lea    r15,[rax+r10*1]
  64ec84:	sub    r14,r10
  64ec87:	lea    r12,[rsp+0x168]
  64ec8f:	mov    QWORD PTR [rsp+0x3f8],r12
  64ec97:	lea    r12,[rsp+0x4e0]
  64ec9f:	mov    QWORD PTR [rsp+0x400],r12
  64eca7:	lea    r13,[rsp+0x170]
  64ecaf:	mov    QWORD PTR [rsp+0x408],r13
  64ecb7:	mov    QWORD PTR [rsp+0x410],r15
  64ecbf:	mov    r15,rdi
  64ecc2:	imul   r15,rsi
  64ecc6:	cmp    r11,r15
  64ecc9:	cmovb  r15,r11
  64eccd:	mov    QWORD PTR [rsp+0x418],r14
  64ecd5:	lea    r14,[rdx+r15*1]
  64ecd9:	mov    QWORD PTR [rsp+0x420],rcx
  64ece1:	mov    QWORD PTR [rsp+0x428],r14
  64ece9:	mov    r14,QWORD PTR [r9+0x38]
  64eced:	mov    r9,QWORD PTR [r9+0x40]
  64ecf1:	sub    r11,r15
  64ecf4:	imul   rsi,r9
  64ecf8:	cmp    r14,rsi
  64ecfb:	cmovb  rsi,r14
  64ecff:	mov    QWORD PTR [rsp+0x430],r11
  64ed07:	mov    r11,rsi
  64ed0a:	shl    r11,0x4
  64ed0e:	add    r11,r8
  64ed11:	sub    r14,rsi
  64ed14:	mov    QWORD PTR [rsp+0x438],rdi
  64ed1c:	mov    QWORD PTR [rsp+0x440],r11
  64ed24:	mov    r11,QWORD PTR [rbx]
  64ed27:	mov    QWORD PTR [rsp+0x448],r14
  64ed2f:	mov    r14,QWORD PTR [rbx+0x8]
  64ed33:	mov    rbx,QWORD PTR [rbx+0x10]
  64ed37:	mov    QWORD PTR [rsp+0x450],r9
  64ed3f:	mov    QWORD PTR [rsp+0x458],r11
  64ed47:	mov    QWORD PTR [rsp+0x460],r14
  64ed4f:	mov    QWORD PTR [rsp+0x468],rbx
  64ed57:	mov    QWORD PTR [rsp+0x470],r12
  64ed5f:	mov    QWORD PTR [rsp+0x478],r13
  64ed67:	mov    QWORD PTR [rsp+0x480],rax
  64ed6f:	mov    QWORD PTR [rsp+0x488],r10
  64ed77:	mov    QWORD PTR [rsp+0x490],rcx
  64ed7f:	mov    QWORD PTR [rsp+0x498],rdx
  64ed87:	mov    QWORD PTR [rsp+0x4a0],r15
  64ed8f:	mov    QWORD PTR [rsp+0x4a8],rdi
  64ed97:	mov    QWORD PTR [rsp+0x4b0],r8
  64ed9f:	mov    QWORD PTR [rsp+0x4b8],rsi
  64eda7:	mov    QWORD PTR [rsp+0x4c0],r9
  64edaf:	mov    QWORD PTR [rsp+0x4c8],r11
  64edb7:	mov    QWORD PTR [rsp+0x4d0],r14
  64edbf:	mov    QWORD PTR [rsp+0x4d8],rbx
  64edc7:	mov    rbx,0xffffffffffffffa8
  64edce:	mov    rdx,QWORD PTR fs:[rbx]
  64edd2:	test   rdx,rdx
  64edd5:	je     64edf3 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2133>
  64edd7:	lea    rdi,[rsp+0x180]
  64eddf:	lea    rsi,[rsp+0x3f8]
  64ede7:	xor    ecx,ecx
  64ede9:	call   6549d0 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_>
  64edee:	mov    rdi,rbp
  64edf1:	jmp    64ee33 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2173>
  64edf3:	call   QWORD PTR [rip+0x9879f]        # 6e7598 <_DYNAMIC+0x5c8>
  64edf9:	mov    rsi,QWORD PTR [rax]
  64edfc:	mov    rdx,QWORD PTR fs:[rbx]
  64ee00:	test   rdx,rdx
  64ee03:	je     64ef7d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x22bd>
  64ee09:	mov    rbx,rbp
  64ee0c:	cmp    QWORD PTR [rdx+0x110],rsi
  64ee13:	jne    64ef9b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x22db>
  64ee19:	lea    rdi,[rsp+0x180]
  64ee21:	lea    rsi,[rsp+0x3f8]
  64ee29:	xor    ecx,ecx
  64ee2b:	call   6549d0 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_>
  64ee30:	mov    rdi,rbx
  64ee33:	mov    rax,QWORD PTR [rsp+0x190]
  64ee3b:	mov    QWORD PTR [rsp+0x7f0],rax
  64ee43:	vmovups xmm0,XMMWORD PTR [rsp+0x180]
  64ee4c:	vmovaps XMMWORD PTR [rsp+0x7e0],xmm0
  64ee55:	lea    rdx,[rsp+0x7f8]
  64ee5d:	vmovdqu xmm0,XMMWORD PTR [rsp+0x198]
  64ee66:	vmovdqu XMMWORD PTR [rsp+0x7f8],xmm0
  64ee6f:	mov    rax,QWORD PTR [rsp+0x1a8]
  64ee77:	mov    QWORD PTR [rsp+0x808],rax
  64ee7f:	lea    rsi,[rsp+0x7e0]
  64ee87:	call   656c90 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0B5_>
  64ee8c:	add    rsp,0x17e8
  64ee93:	pop    rbx
  64ee94:	pop    r12
  64ee96:	pop    r13
  64ee98:	pop    r14
  64ee9a:	pop    r15
  64ee9c:	pop    rbp
  64ee9d:	ret    
  64ee9e:	lea    rdx,[rip+0x9b3fb]        # 6ea2a0 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.10381458960231317189+0x1040>
  64eea5:	lea    rsi,[rsp+0x6e0]
  64eead:	lea    r9,[rip+0x93ea4]        # 6e2d58 <anon.01aa214378fce5a476fd64886db4a5c0.1330.llvm.3043956165524440405>
  64eeb4:	jmp    64eecc <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x220c>
  64eeb6:	lea    rsi,[rsp+0x4e0]
  64eebe:	lea    r9,[rip+0x93eab]        # 6e2d70 <anon.01aa214378fce5a476fd64886db4a5c0.1331.llvm.3043956165524440405>
  64eec5:	lea    rdx,[rip+0x9b3cc]        # 6ea298 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.10381458960231317189+0x1038>
  64eecc:	xor    edi,edi
  64eece:	xor    ecx,ecx
  64eed0:	vzeroupper 
  64eed3:	call   QWORD PTR [rip+0x984bf]        # 6e7398 <_DYNAMIC+0x3c8>
  64eed9:	jmp    64f07f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x23bf>
  64eede:	lea    rdx,[rip+0x95033]        # 6e3f18 <anon.e5d9da81ce3e97b5bac39b4a40b5610e.12.llvm.10381458960231317189+0xd8>
  64eee5:	vzeroupper 
  64eee8:	call   QWORD PTR [rip+0x98332]        # 6e7220 <_DYNAMIC+0x250>
  64eeee:	jmp    64f07f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x23bf>
  64eef3:	mov    rax,r13
  64eef6:	jmp    64f01f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x235f>
  64eefb:	mov    rax,r8
  64eefe:	jmp    64f01f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x235f>
  64ef03:	mov    rax,rbp
  64ef06:	jmp    64f01b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x235b>
  64ef0b:	mov    rax,r10
  64ef0e:	mov    rbp,QWORD PTR [rsp]
  64ef12:	mov    rsi,QWORD PTR [rsp+0x8]
  64ef17:	jmp    64f01f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x235f>
  64ef1c:	mov    rax,r9
  64ef1f:	jmp    64f01f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x235f>
  64ef24:	mov    rax,r14
  64ef27:	jmp    64f01f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x235f>
  64ef2c:	mov    rax,rdx
  64ef2f:	jmp    64f01f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x235f>
  64ef34:	mov    rax,rbx
  64ef37:	jmp    64f01f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x235f>
  64ef3c:	mov    rax,r11
  64ef3f:	jmp    64f01f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x235f>
  64ef44:	mov    rax,r12
  64ef47:	jmp    64f01f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x235f>
  64ef4c:	mov    rax,rdi
  64ef4f:	jmp    64f01f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x235f>
  64ef54:	mov    rax,r15
  64ef57:	jmp    64f01f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x235f>
  64ef5c:	mov    rbx,rdi
  64ef5f:	lea    rdi,[rip+0xffffffffff9c3482]        # 123e8 <anon.e5d9da81ce3e97b5bac39b4a40b5610e.67.llvm.10381458960231317189+0x1>
  64ef66:	lea    rdx,[rip+0x9569b]        # 6e4608 <anon.e5d9da81ce3e97b5bac39b4a40b5610e.12.llvm.10381458960231317189+0x7c8>
  64ef6d:	mov    esi,0x37
  64ef72:	call   QWORD PTR [rip+0x98310]        # 6e7288 <_DYNAMIC+0x2b8>
  64ef78:	jmp    64f07f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x23bf>
  64ef7d:	sub    rsi,0xffffffffffffff80
  64ef81:	lea    rdi,[rsp+0x180]
  64ef89:	lea    rdx,[rsp+0x3f8]
  64ef91:	call   64a410 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_3zip11ZipProducerIB2V_INtNtNtB1P_5slice6chunks14ChunksProducerhEB3p_EIB3q_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5X_s_0ENCB5X_0EE0NCB1G_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7f_E0TB7f_B7f_EEB61_>
  64ef96:	jmp    64edee <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x212e>
  64ef9b:	sub    rsi,0xffffffffffffff80
  64ef9f:	lea    rdi,[rsp+0x180]
  64efa7:	lea    rcx,[rsp+0x3f8]
  64efaf:	call   64b090 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_3zip11ZipProducerIB2W_INtNtNtB1Q_5slice6chunks14ChunksProducerhEB3q_EIB3r_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5Y_s_0ENCB5Y_0EE0NCB1H_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7g_E0TB7g_B7g_EEB62_>
  64efb4:	jmp    64ee30 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2170>
  64efb9:	mov    QWORD PTR [rsp+0x6e0],0x10
  64efc5:	cmp    QWORD PTR [rip+0x9b2d3],0x10        # 6ea2a0 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.10381458960231317189+0x1040>
  64efcd:	jne    64f091 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x23d1>
  64efd3:	lea    rsi,[rsp+0x4e0]
  64efdb:	mov    QWORD PTR [rsi],0x80
  64efe2:	lea    r9,[rip+0x93dcf]        # 6e2db8 <anon.01aa214378fce5a476fd64886db4a5c0.1334.llvm.3043956165524440405>
  64efe9:	jmp    64eec5 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2205>
  64efee:	lea    rdx,[rip+0x95403]        # 6e43f8 <anon.e5d9da81ce3e97b5bac39b4a40b5610e.12.llvm.10381458960231317189+0x5b8>
  64eff5:	mov    rsi,rdi
  64eff8:	vzeroupper 
  64effb:	call   QWORD PTR [rip+0x9821f]        # 6e7220 <_DYNAMIC+0x250>
  64f001:	jmp    64f07f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x23bf>
  64f003:	mov    rsi,rax
  64f006:	mov    rcx,rax
  64f009:	lea    rax,[rip+0x953e8]        # 6e43f8 <anon.e5d9da81ce3e97b5bac39b4a40b5610e.12.llvm.10381458960231317189+0x5b8>
  64f010:	mov    QWORD PTR [rsp+0x80],rax
  64f018:	mov    rax,rcx
  64f01b:	mov    rbp,QWORD PTR [rsp]
  64f01f:	mov    rdi,rax
  64f022:	mov    rdx,QWORD PTR [rsp+0x80]
  64f02a:	call   QWORD PTR [rip+0x981f0]        # 6e7220 <_DYNAMIC+0x250>
  64f030:	jmp    64f07f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x23bf>
  64f032:	mov    edi,0x10
  64f037:	jmp    64f062 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x23a2>
  64f039:	lea    rdx,[rip+0x953b8]        # 6e43f8 <anon.e5d9da81ce3e97b5bac39b4a40b5610e.12.llvm.10381458960231317189+0x5b8>
  64f040:	mov    rsi,rdi
  64f043:	call   QWORD PTR [rip+0x981d7]        # 6e7220 <_DYNAMIC+0x250>
  64f049:	jmp    64f07f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x23bf>
  64f04b:	mov    edi,0x1
  64f050:	mov    esi,0xff0
  64f055:	call   QWORD PTR [rip+0x981b5]        # 6e7210 <_DYNAMIC+0x240>
  64f05b:	jmp    64f07f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x23bf>
  64f05d:	mov    edi,0x1
  64f062:	mov    esi,0xff0
  64f067:	call   QWORD PTR [rip+0x981a3]        # 6e7210 <_DYNAMIC+0x240>
  64f06d:	jmp    64f07f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x23bf>
  64f06f:	mov    edi,0x1
  64f074:	mov    esi,0xff0
  64f079:	call   QWORD PTR [rip+0x98191]        # 6e7210 <_DYNAMIC+0x240>
  64f07f:	ud2    
  64f081:	mov    edi,0x1
  64f086:	mov    esi,0xff0
  64f08b:	call   QWORD PTR [rip+0x9817f]        # 6e7210 <_DYNAMIC+0x240>
  64f091:	lea    r9,[rip+0x93d08]        # 6e2da0 <anon.01aa214378fce5a476fd64886db4a5c0.1333.llvm.3043956165524440405>
  64f098:	lea    rdx,[rip+0x9b201]        # 6ea2a0 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.10381458960231317189+0x1040>
  64f09f:	lea    rsi,[rsp+0x6e0]
  64f0a7:	jmp    64eecc <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x220c>
  64f0ac:	jmp    64f0fb <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x243b>
  64f0ae:	mov    r14,rax
  64f0b1:	mov    rbp,QWORD PTR [rsp]
  64f0b5:	jmp    64f0f0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2430>
  64f0b7:	jmp    64f0d5 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2415>
  64f0b9:	mov    r14,rax
  64f0bc:	mov    rdi,rbx
  64f0bf:	mov    rbx,QWORD PTR [rip+0x98112]        # 6e71d8 <free@GLIBC_2.2.5>
  64f0c6:	call   rbx
  64f0c8:	mov    rdi,r15
  64f0cb:	call   rbx
  64f0cd:	jmp    64f0fe <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x243e>
  64f0cf:	jmp    64f0fb <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x243b>
  64f0d1:	jmp    64f0fb <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x243b>
  64f0d3:	jmp    64f0ed <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x242d>
  64f0d5:	mov    r14,rax
  64f0d8:	mov    rdi,rbx
  64f0db:	call   QWORD PTR [rip+0x980f7]        # 6e71d8 <free@GLIBC_2.2.5>
  64f0e1:	jmp    64f0fe <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x243e>
  64f0e3:	mov    r14,rax
  64f0e6:	mov    rdi,rbx
  64f0e9:	jmp    64f10b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x244b>
  64f0eb:	jmp    64f0ed <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x242d>
  64f0ed:	mov    r14,rax
  64f0f0:	mov    rdi,rbp
  64f0f3:	call   QWORD PTR [rip+0x980df]        # 6e71d8 <free@GLIBC_2.2.5>
  64f0f9:	jmp    64f0fe <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x243e>
  64f0fb:	mov    r14,rax
  64f0fe:	cmp    QWORD PTR [rsp+0x28],0x0
  64f104:	je     64f111 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2451>
  64f106:	mov    rdi,QWORD PTR [rsp+0x20]
  64f10b:	call   QWORD PTR [rip+0x980c7]        # 6e71d8 <free@GLIBC_2.2.5>
  64f111:	mov    rdi,r14
  64f114:	call   6e09f0 <_Unwind_Resume@plt>
  64f119:	int3   
  64f11a:	int3   
  64f11b:	int3   
  64f11c:	int3   
  64f11d:	int3   
  64f11e:	int3   
  64f11f:	int3   

