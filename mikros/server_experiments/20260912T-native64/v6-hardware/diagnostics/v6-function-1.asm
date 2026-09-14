0000000000651740 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_>:
  651740:	push   rbp
  651741:	mov    rbp,rsp
  651744:	push   r15
  651746:	push   r14
  651748:	push   r13
  65174a:	push   r12
  65174c:	push   rbx
  65174d:	and    rsp,0xffffffffffffffc0
  651751:	sub    rsp,0xfc0
  651758:	mov    QWORD PTR [rsp],0x0
  651760:	sub    rsp,0x1000
  651767:	mov    QWORD PTR [rsp],0x0
  65176f:	sub    rsp,0x1000
  651776:	mov    QWORD PTR [rsp],0x0
  65177e:	sub    rsp,0x1000
  651785:	mov    QWORD PTR [rsp],0x0
  65178d:	sub    rsp,0x440
  651794:	mov    rbx,QWORD PTR [rbp+0x10]
  651798:	mov    QWORD PTR [rsp+0x200],rsi
  6517a0:	mov    QWORD PTR [rsp+0x208],rcx
  6517a8:	mov    QWORD PTR [rsp+0x210],r8
  6517b0:	shr    rsi,1
  6517b3:	cmp    rsi,r8
  6517b6:	mov    QWORD PTR [rsp+0x118],rdi
  6517be:	jae    651890 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x150>
  6517c4:	mov    rax,QWORD PTR [r9]
  6517c7:	mov    QWORD PTR [rsp+0x198],rax
  6517cf:	mov    rax,QWORD PTR [r9+0x8]
  6517d3:	mov    QWORD PTR [rsp+0x80],rax
  6517db:	mov    r15,QWORD PTR [r9+0x10]
  6517df:	mov    rax,QWORD PTR [r9+0x18]
  6517e3:	mov    QWORD PTR [rsp+0x190],rax
  6517eb:	mov    rax,QWORD PTR [r9+0x20]
  6517ef:	mov    QWORD PTR [rsp+0xa0],rax
  6517f7:	mov    rax,QWORD PTR [r9+0x28]
  6517fb:	mov    QWORD PTR [rsp+0x88],rax
  651803:	mov    rax,QWORD PTR [r9+0x30]
  651807:	mov    QWORD PTR [rsp+0x188],rax
  65180f:	mov    rax,QWORD PTR [r9+0x38]
  651813:	mov    QWORD PTR [rsp+0x98],rax
  65181b:	mov    rax,QWORD PTR [r9+0x40]
  65181f:	mov    QWORD PTR [rsp+0x130],rax
  651827:	mov    rbx,QWORD PTR [rbx+0x10]
  65182b:	mov    edi,0xff0
  651830:	mov    esi,0x1
  651835:	call   QWORD PTR [rip+0x9cde5]        # 6ee620 <calloc@GLIBC_2.2.5>
  65183b:	mov    QWORD PTR [rsp+0x68],rax
  651840:	test   rax,rax
  651843:	je     6551da <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3a9a>
  651849:	mov    QWORD PTR [rsp+0x78],r15
  65184e:	test   r15,r15
  651851:	mov    rax,QWORD PTR [rsp+0x88]
  651859:	je     6550c6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3986>
  65185f:	test   rax,rax
  651862:	je     6550c6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3986>
  651868:	mov    rax,QWORD PTR [rsp+0x80]
  651870:	test   rax,rax
  651873:	je     6518c4 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x184>
  651875:	mov    rcx,rax
  651878:	mov    rsi,QWORD PTR [rsp+0x78]
  65187d:	or     rcx,rsi
  651880:	shr    rcx,0x20
  651884:	je     6518d7 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x197>
  651886:	xor    edx,edx
  651888:	div    rsi
  65188b:	mov    rsi,rax
  65188e:	jmp    6518dd <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x19d>
  651890:	test   dl,dl
  651892:	je     6518b3 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x173>
  651894:	mov    rax,0xffffffffffffffa8
  65189b:	mov    rax,QWORD PTR fs:[rax]
  65189f:	test   rax,rax
  6518a2:	je     654d88 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3648>
  6518a8:	add    rax,0x110
  6518ae:	jmp    654da0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3660>
  6518b3:	test   rcx,rcx
  6518b6:	je     6517c4 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x84>
  6518bc:	shr    rcx,1
  6518bf:	jmp    654db4 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3674>
  6518c4:	xor    esi,esi
  6518c6:	mov    rax,QWORD PTR [rsp+0xa0]
  6518ce:	test   rax,rax
  6518d1:	jne    6518f2 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1b2>
  6518d3:	xor    ecx,ecx
  6518d5:	jmp    651925 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1e5>
  6518d7:	xor    edx,edx
  6518d9:	div    esi
  6518db:	mov    esi,eax
  6518dd:	cmp    rdx,0x1
  6518e1:	sbb    rsi,0xffffffffffffffff
  6518e5:	mov    rax,QWORD PTR [rsp+0xa0]
  6518ed:	test   rax,rax
  6518f0:	je     6518d3 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x193>
  6518f2:	mov    rcx,rax
  6518f5:	mov    rdi,QWORD PTR [rsp+0x88]
  6518fd:	or     rcx,rdi
  651900:	shr    rcx,0x20
  651904:	je     651910 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1d0>
  651906:	xor    edx,edx
  651908:	div    rdi
  65190b:	mov    rcx,rax
  65190e:	jmp    651916 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1d6>
  651910:	xor    edx,edx
  651912:	div    edi
  651914:	mov    ecx,eax
  651916:	cmp    rdx,0x1
  65191a:	sbb    rcx,0xffffffffffffffff
  65191e:	cmp    rcx,rsi
  651921:	cmovae rcx,rsi
  651925:	cmp    QWORD PTR [rsp+0x130],0x0
  65192e:	je     6550c6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3986>
  651934:	mov    r10d,0xff
  65193a:	mov    rax,QWORD PTR [rsp+0x98]
  651942:	test   rax,rax
  651945:	je     654d5c <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x361c>
  65194b:	mov    rdx,rax
  65194e:	mov    r8,QWORD PTR [rsp+0x130]
  651956:	or     rdx,r8
  651959:	shr    rdx,0x20
  65195d:	je     651969 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x229>
  65195f:	xor    edx,edx
  651961:	div    r8
  651964:	mov    r9,rax
  651967:	jmp    651971 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x231>
  651969:	xor    edx,edx
  65196b:	div    r8d
  65196e:	mov    r9d,eax
  651971:	mov    rsi,QWORD PTR [rsp+0x80]
  651979:	mov    rdi,QWORD PTR [rsp+0x78]
  65197e:	cmp    rdx,0x1
  651982:	sbb    r9,0xffffffffffffffff
  651986:	cmp    r9,rcx
  651989:	cmovae r9,rcx
  65198d:	test   r9,r9
  651990:	je     654d5c <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x361c>
  651996:	mov    rax,QWORD PTR [rbx]
  651999:	mov    QWORD PTR [rsp+0x1e0],rax
  6519a1:	movabs r13,0x7fffffffffffffc0
  6519ab:	mov    rcx,QWORD PTR [rsp+0x188]
  6519b3:	lea    rax,[rcx+0x8]
  6519b7:	mov    QWORD PTR [rsp+0x180],rax
  6519bf:	shl    r8,0x4
  6519c3:	mov    QWORD PTR [rsp+0x1d0],r8
  6519cb:	mov    eax,0x87
  6519d0:	vmovq  xmm0,rax
  6519d5:	vmovdqa XMMWORD PTR [rsp+0x3b0],xmm0
  6519de:	mov    eax,0xff
  6519e3:	mov    QWORD PTR [rsp+0x60],rax
  6519e8:	xor    ebx,ebx
  6519ea:	lea    rax,[rip+0x99927]        # 6eb318 <anon.e5d9da81ce3e97b5bac39b4a40b5610e.12.llvm.17036336450247840325+0xd8>
  6519f1:	mov    QWORD PTR [rsp+0x170],rax
  6519f9:	mov    rax,QWORD PTR [rsp+0x190]
  651a01:	mov    QWORD PTR [rsp+0x128],rax
  651a09:	mov    rax,QWORD PTR [rsp+0x198]
  651a11:	mov    QWORD PTR [rsp+0x120],rax
  651a19:	mov    QWORD PTR [rsp+0x178],rcx
  651a21:	mov    rax,QWORD PTR [rsp+0x88]
  651a29:	mov    QWORD PTR [rsp+0x1d8],r9
  651a31:	jmp    651b2c <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3ec>
  651a36:	cs nop WORD PTR [rax+rax*1+0x0]
  651a40:	movabs r13,0x7fffffffffffffc0
  651a4a:	mov    rbx,QWORD PTR [rsp+0x1e8]
  651a52:	inc    rbx
  651a55:	mov    QWORD PTR [rsp+0x23d8],0xff
  651a61:	mov    QWORD PTR [rsp+0x23e0],r14
  651a69:	mov    QWORD PTR [rsp+0x23e8],0xff
  651a75:	mov    rax,QWORD PTR [rsp+0x60]
  651a7a:	mov    QWORD PTR [rsp+0x23c0],rax
  651a82:	mov    rax,QWORD PTR [rsp+0x68]
  651a87:	mov    QWORD PTR [rsp+0x23c8],rax
  651a8f:	mov    rax,QWORD PTR [rsp+0x1f0]
  651a97:	mov    QWORD PTR [rsp+0x23d0],rax
  651a9f:	lea    rdi,[rsp+0xfc0]
  651aa7:	lea    rsi,[rsp+0x23c0]
  651aaf:	lea    rdx,[rsp+0x23d8]
  651ab7:	call   65d880 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0B5_>
  651abc:	mov    rax,QWORD PTR [rsp+0xfc0]
  651ac4:	mov    QWORD PTR [rsp+0x60],rax
  651ac9:	mov    rax,QWORD PTR [rsp+0xfc8]
  651ad1:	mov    QWORD PTR [rsp+0x68],rax
  651ad6:	mov    r10,QWORD PTR [rsp+0xfd0]
  651ade:	mov    rax,QWORD PTR [rsp+0x1d0]
  651ae6:	add    QWORD PTR [rsp+0x180],rax
  651aee:	add    QWORD PTR [rsp+0x178],rax
  651af6:	mov    rdi,QWORD PTR [rsp+0x78]
  651afb:	add    QWORD PTR [rsp+0x120],rdi
  651b03:	mov    rax,QWORD PTR [rsp+0x88]
  651b0b:	add    QWORD PTR [rsp+0x128],rax
  651b13:	mov    r9,QWORD PTR [rsp+0x1d8]
  651b1b:	cmp    rbx,r9
  651b1e:	mov    rsi,QWORD PTR [rsp+0x80]
  651b26:	je     654d66 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3626>
  651b2c:	mov    QWORD PTR [rsp+0x1f0],r10
  651b34:	mov    rcx,rbx
  651b37:	imul   rcx,rdi
  651b3b:	sub    rsi,rcx
  651b3e:	cmp    rdi,rsi
  651b41:	cmovb  rsi,rdi
  651b45:	mov    QWORD PTR [rsp+0x40],rsi
  651b4a:	add    rcx,QWORD PTR [rsp+0x198]
  651b52:	mov    QWORD PTR [rsp+0xb8],rcx
  651b5a:	mov    r14,rbx
  651b5d:	imul   r14,rax
  651b61:	mov    r12,QWORD PTR [rsp+0xa0]
  651b69:	sub    r12,r14
  651b6c:	cmp    rax,r12
  651b6f:	cmovb  r12,rax
  651b73:	mov    rax,QWORD PTR [rsp+0x190]
  651b7b:	add    rax,r14
  651b7e:	mov    QWORD PTR [rsp+0xb0],rax
  651b86:	mov    QWORD PTR [rsp+0x1e8],rbx
  651b8e:	mov    rax,QWORD PTR [rsp+0x130]
  651b96:	imul   rbx,rax
  651b9a:	mov    r15,QWORD PTR [rsp+0x98]
  651ba2:	sub    r15,rbx
  651ba5:	cmp    rax,r15
  651ba8:	cmovb  r15,rax
  651bac:	mov    rax,rbx
  651baf:	shl    rax,0x4
  651bb3:	add    rax,QWORD PTR [rsp+0x188]
  651bbb:	mov    QWORD PTR [rsp+0xa8],rax
  651bc3:	mov    rax,QWORD PTR [rsp+0x1e0]
  651bcb:	movzx  eax,BYTE PTR [rax]
  651bce:	lea    rcx,[rip+0xffffffffff9c086b]        # 12440 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1532>
  651bd5:	movsxd rax,DWORD PTR [rcx+rax*4]
  651bd9:	add    rax,rcx
  651bdc:	jmp    rax
  651bde:	mov    eax,DWORD PTR [rip+0x9f2a4]        # 6f0e88 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.17036336450247840325+0x930>
  651be4:	test   eax,eax
  651be6:	jne    654d0c <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x35cc>
  651bec:	mov    eax,DWORD PTR [rip+0xa03be]        # 6f1fb0 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.17036336450247840325+0x1120>
  651bf2:	test   eax,eax
  651bf4:	jne    654d20 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x35e0>
  651bfa:	mov    edi,0xff0
  651bff:	mov    esi,0x1
  651c04:	call   QWORD PTR [rip+0x9ca16]        # 6ee620 <calloc@GLIBC_2.2.5>
  651c0a:	test   rax,rax
  651c0d:	je     65518b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3a4b>
  651c13:	mov    rbx,rax
  651c16:	mov    rax,QWORD PTR [rsp+0x40]
  651c1b:	shr    rax,0x4
  651c1f:	shr    r12,0x4
  651c23:	cmp    r12,rax
  651c26:	cmovae r12,rax
  651c2a:	cmp    r15,r12
  651c2d:	cmovb  r12,r15
  651c31:	test   r12,r12
  651c34:	je     651fa0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x860>
  651c3a:	xor    r14d,r14d
  651c3d:	nop    DWORD PTR [rax]
  651c40:	mov    r15,r14
  651c43:	shl    r15,0x4
  651c47:	mov    rax,QWORD PTR [rsp+0xb8]
  651c4f:	lea    rsi,[rax+r15*1]
  651c53:	lea    rdi,[rsp+0x400]
  651c5b:	vzeroupper 
  651c5e:	call   65f1c0 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate6domainNtB2_6Domain8eval_row>
  651c63:	mov    rax,QWORD PTR [rsp+0xb0]
  651c6b:	lea    rsi,[rax+r15*1]
  651c6f:	lea    rdi,[rsp+0xfc0]
  651c77:	call   65f1c0 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate6domainNtB2_6Domain8eval_row>
  651c7c:	inc    r14
  651c7f:	vmovups ymm0,YMMWORD PTR [rsp+0x4df]
  651c88:	lea    rcx,[rsp+0x23e0]
  651c90:	vmovups YMMWORD PTR [rcx+0xdf],ymm0
  651c98:	vmovups ymm0,YMMWORD PTR [rsp+0x4c0]
  651ca1:	vmovups YMMWORD PTR [rcx+0xc0],ymm0
  651ca9:	vmovups ymm0,YMMWORD PTR [rsp+0x4a0]
  651cb2:	vmovups YMMWORD PTR [rcx+0xa0],ymm0
  651cba:	vmovups ymm0,YMMWORD PTR [rsp+0x480]
  651cc3:	vmovups YMMWORD PTR [rcx+0x80],ymm0
  651ccb:	vmovups ymm0,YMMWORD PTR [rsp+0x400]
  651cd4:	vmovups ymm1,YMMWORD PTR [rsp+0x420]
  651cdd:	vmovups ymm2,YMMWORD PTR [rsp+0x440]
  651ce6:	vmovups ymm3,YMMWORD PTR [rsp+0x460]
  651cef:	vmovups YMMWORD PTR [rcx+0x60],ymm3
  651cf4:	vmovups YMMWORD PTR [rcx+0x40],ymm2
  651cf9:	vmovups YMMWORD PTR [rcx+0x20],ymm1
  651cfe:	vmovups YMMWORD PTR [rcx],ymm0
  651d02:	lea    rax,[rsp+0x1fc1]
  651d0a:	vxorps xmm4,xmm4,xmm4
  651d0e:	vmovups XMMWORD PTR [rax],xmm4
  651d12:	vmovups ymm0,YMMWORD PTR [rsp+0x109f]
  651d1b:	lea    rdx,[rsp+0x2500]
  651d23:	vmovups YMMWORD PTR [rdx+0xdf],ymm0
  651d2b:	vmovups ymm0,YMMWORD PTR [rsp+0x1080]
  651d34:	vmovups YMMWORD PTR [rdx+0xc0],ymm0
  651d3c:	vmovups ymm0,YMMWORD PTR [rsp+0x1060]
  651d45:	vmovups YMMWORD PTR [rdx+0xa0],ymm0
  651d4d:	vmovups ymm0,YMMWORD PTR [rsp+0x1040]
  651d56:	vmovups YMMWORD PTR [rdx+0x80],ymm0
  651d5e:	vmovups ymm0,YMMWORD PTR [rsp+0xfc0]
  651d67:	vmovups ymm1,YMMWORD PTR [rsp+0xfe0]
  651d70:	vmovups ymm2,YMMWORD PTR [rsp+0x1000]
  651d79:	vmovups ymm3,YMMWORD PTR [rsp+0x1020]
  651d82:	vmovups YMMWORD PTR [rdx+0x60],ymm3
  651d87:	vmovups YMMWORD PTR [rdx+0x40],ymm2
  651d8c:	vmovups YMMWORD PTR [rdx+0x20],ymm1
  651d91:	vmovups YMMWORD PTR [rdx],ymm0
  651d95:	add    r15,QWORD PTR [rsp+0xa8]
  651d9d:	movzx  eax,BYTE PTR [rsp+0x1fd0]
  651da5:	vmovups xmm0,XMMWORD PTR [rsp+0x1fc0]
  651dae:	vmovups XMMWORD PTR [rcx+0xff],xmm0
  651db6:	mov    BYTE PTR [rcx+0x10f],al
  651dbc:	lea    rax,[rsp+0x801]
  651dc4:	vmovups XMMWORD PTR [rax],xmm4
  651dc8:	mov    QWORD PTR [rsp+0x24f0],0x0
  651dd4:	mov    QWORD PTR [rsp+0x24f8],0xff
  651de0:	movzx  eax,BYTE PTR [rsp+0x810]
  651de8:	mov    BYTE PTR [rcx+0x22f],al
  651dee:	vmovups xmm0,XMMWORD PTR [rsp+0x800]
  651df7:	vmovups XMMWORD PTR [rcx+0x21f],xmm0
  651dff:	mov    rax,QWORD PTR [r15]
  651e02:	mov    rcx,QWORD PTR [r15+0x8]
  651e06:	vmovq  xmm0,rax
  651e0b:	vmovq  xmm1,rcx
  651e10:	xor    rcx,rax
  651e13:	vmovq  xmm2,rcx
  651e18:	mov    rax,rdx
  651e1b:	xor    ecx,ecx
  651e1d:	vmovdqa xmm8,XMMWORD PTR [rsp+0x3b0]
  651e26:	mov    r10,QWORD PTR [rip+0x9cc03]        # 6eea30 <_DYNAMIC+0x630>
  651e2d:	nop    DWORD PTR [rax]
  651e30:	movzx  edx,BYTE PTR [rax-0x120]
  651e37:	shl    edx,0x4
  651e3a:	mov    rsi,QWORD PTR [r10+rdx*1]
  651e3e:	mov    rdx,QWORD PTR [r10+rdx*1+0x8]
  651e43:	vmovq  xmm3,rsi
  651e48:	vpclmullqlqdq xmm3,xmm0,xmm3
  651e4e:	vmovq  xmm4,rdx
  651e53:	vpclmullqlqdq xmm4,xmm1,xmm4
  651e59:	xor    rdx,rsi
  651e5c:	vmovq  xmm5,rdx
  651e61:	vpclmullqlqdq xmm5,xmm2,xmm5
  651e67:	vpternlogq xmm5,xmm3,xmm4,0x96
  651e6e:	vpshufd xmm6,xmm4,0xee
  651e73:	vpclmullqlqdq xmm6,xmm6,xmm8
  651e79:	vpshufd xmm7,xmm5,0xee
  651e7e:	vpxor  xmm4,xmm7,xmm4
  651e82:	movzx  edx,BYTE PTR [rax]
  651e85:	vpclmullqlqdq xmm4,xmm4,xmm8
  651e8b:	vpextrq rsi,xmm6,0x1
  651e91:	lea    rdi,[rsi+rsi*1]
  651e95:	lea    r8,[rsi*4+0x0]
  651e9d:	mov    r9,rsi
  651ea0:	xor    r8,rdi
  651ea3:	shl    r9,0x7
  651ea7:	xor    r9,r8
  651eaa:	vpxor  xmm3,xmm3,xmm4
  651eae:	vmovq  rdi,xmm3
  651eb3:	vpshufd xmm3,xmm3,0xee
  651eb8:	xor    rdi,r9
  651ebb:	vpternlogq xmm3,xmm5,xmm6,0x96
  651ec2:	vmovq  r8,xmm3
  651ec7:	shl    edx,0x4
  651eca:	mov    r9,QWORD PTR [r10+rdx*1]
  651ece:	mov    rdx,QWORD PTR [r10+rdx*1+0x8]
  651ed3:	xor    rdi,rsi
  651ed6:	vmovq  xmm4,rdi
  651edb:	vmovq  xmm5,r9
  651ee0:	vmovq  xmm6,rdx
  651ee5:	vpclmullqlqdq xmm3,xmm3,xmm6
  651eeb:	xor    r8,rdi
  651eee:	vpclmullqlqdq xmm4,xmm4,xmm5
  651ef4:	xor    rdx,r9
  651ef7:	vmovq  xmm5,r8
  651efc:	vmovq  xmm6,rdx
  651f01:	vpclmullqlqdq xmm5,xmm5,xmm6
  651f07:	vpshufd xmm6,xmm3,0xee
  651f0c:	vpternlogq xmm5,xmm4,xmm3,0x96
  651f13:	vpclmullqlqdq xmm6,xmm6,xmm8
  651f19:	vpshufd xmm7,xmm5,0xee
  651f1e:	vpxor  xmm3,xmm7,xmm3
  651f22:	vpclmullqlqdq xmm3,xmm3,xmm8
  651f28:	vpextrq rdx,xmm6,0x1
  651f2e:	lea    rsi,[rdx+rdx*1]
  651f32:	shl    rdx,0x2
  651f36:	xor    rdx,rsi
  651f39:	vpxor  xmm7,xmm4,xmm3
  651f3d:	vmovq  rsi,xmm7
  651f42:	xor    rsi,rdx
  651f45:	vpshufd xmm4,xmm4,0xee
  651f4a:	vpsllq xmm7,xmm6,0x7
  651f4f:	vpxor  xmm4,xmm6,xmm4
  651f53:	vpbroadcastq xmm5,xmm5
  651f58:	vpinsrq xmm5,xmm5,rsi,0x0
  651f5e:	vpalignr xmm4,xmm4,xmm7,0x8
  651f64:	vmovdqa xmm7,XMMWORD PTR [rbx+rcx*1]
  651f69:	vpblendd xmm3,xmm7,xmm3,0xc
  651f6f:	vpunpckhqdq xmm6,xmm6,xmm7
  651f73:	vpxor  xmm4,xmm4,xmm5
  651f77:	vpternlogq xmm6,xmm4,xmm3,0x96
  651f7e:	vmovdqa XMMWORD PTR [rbx+rcx*1],xmm6
  651f83:	add    rcx,0x10
  651f87:	inc    rax
  651f8a:	cmp    rcx,0xff0
  651f91:	jne    651e30 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x6f0>
  651f97:	cmp    r14,r12
  651f9a:	jne    651c40 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x500>
  651fa0:	mov    edi,0xff0
  651fa5:	vzeroupper 
  651fa8:	call   QWORD PTR [rip+0x9c64a]        # 6ee5f8 <malloc@GLIBC_2.2.5>
  651fae:	test   rax,rax
  651fb1:	je     6551a4 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3a64>
  651fb7:	mov    r14,rax
  651fba:	mov    rdi,QWORD PTR [rip+0x9e5af]        # 6f0570 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.17036336450247840325+0x18>
  651fc1:	cmp    rdi,0x10
  651fc5:	jb     655192 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3a52>
  651fcb:	mov    rax,QWORD PTR [rip+0x9e596]        # 6f0568 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.17036336450247840325+0x10>
  651fd2:	lea    rdx,[rax+0xd000]
  651fd9:	lea    rsi,[rax+0xe000]
  651fe0:	lea    rdi,[rax+0xf000]
  651fe7:	xor    r8d,r8d
  651fea:	nop    WORD PTR [rax+rax*1+0x0]
  651ff0:	mov    r9,QWORD PTR [rbx+r8*1]
  651ff4:	mov    r15,r9
  651ff7:	mov    r11,r9
  651ffa:	mov    r12,r9
  651ffd:	mov    r13,r9
  652000:	mov    r10d,r9d
  652003:	shr    r10d,0x4
  652007:	and    r10d,0xff0
  65200e:	lea    rcx,[rax+0x1000]
  652015:	vmovdqa xmm0,XMMWORD PTR [rcx+r10*1]
  65201b:	movzx  r10d,r9b
  65201f:	shl    r10d,0x4
  652023:	vpxor  xmm0,xmm0,XMMWORD PTR [rax+r10*1]
  652029:	mov    r10d,r9d
  65202c:	shr    r10d,0xc
  652030:	and    r10d,0xff0
  652037:	shr    r9d,0x14
  65203b:	and    r9d,0xfffffff0
  65203f:	lea    rcx,[rax+0x3000]
  652046:	vmovdqa xmm1,XMMWORD PTR [rcx+r9*1]
  65204c:	lea    rcx,[rax+0x2000]
  652053:	vpternlogq xmm1,xmm0,XMMWORD PTR [rcx+r10*1],0x96
  65205b:	mov    r10,QWORD PTR [rbx+r8*1+0x8]
  652060:	mov    r9,r10
  652063:	shr    r12,0x28
  652067:	movzx  r12d,r12b
  65206b:	shl    r12d,0x4
  65206f:	lea    rcx,[rax+0x5000]
  652076:	vmovdqa xmm0,XMMWORD PTR [rcx+r12*1]
  65207c:	mov    r12,r10
  65207f:	shr    r13,0x20
  652083:	movzx  r13d,r13b
  652087:	shl    r13d,0x4
  65208b:	lea    rcx,[rax+0x4000]
  652092:	vpternlogq xmm0,xmm1,XMMWORD PTR [rcx+r13*1],0x96
  65209a:	mov    r13,r10
  65209d:	shr    r15,0x38
  6520a1:	shl    r15d,0x4
  6520a5:	lea    rcx,[rax+0x7000]
  6520ac:	vmovdqa xmm1,XMMWORD PTR [rcx+r15*1]
  6520b2:	mov    r15,r10
  6520b5:	shr    r11,0x30
  6520b9:	movzx  r11d,r11b
  6520bd:	shl    r11d,0x4
  6520c1:	lea    rcx,[rax+0x6000]
  6520c8:	vpternlogq xmm1,xmm0,XMMWORD PTR [rcx+r11*1],0x96
  6520d0:	mov    r11d,r10d
  6520d3:	shr    r11d,0x4
  6520d7:	and    r11d,0xff0
  6520de:	lea    rcx,[rax+0x9000]
  6520e5:	vmovdqa xmm0,XMMWORD PTR [rcx+r11*1]
  6520eb:	movzx  r11d,r10b
  6520ef:	shl    r11d,0x4
  6520f3:	lea    rcx,[rax+0x8000]
  6520fa:	vpternlogq xmm0,xmm1,XMMWORD PTR [rcx+r11*1],0x96
  652102:	mov    r11d,r10d
  652105:	shr    r10d,0x14
  652109:	and    r10d,0xfffffff0
  65210d:	lea    rcx,[rax+0xb000]
  652114:	vmovdqa xmm1,XMMWORD PTR [rcx+r10*1]
  65211a:	shr    r11d,0xc
  65211e:	and    r11d,0xff0
  652125:	lea    rcx,[rax+0xa000]
  65212c:	vpternlogq xmm1,xmm0,XMMWORD PTR [rcx+r11*1],0x96
  652134:	shr    r13,0x28
  652138:	movzx  r10d,r13b
  65213c:	shl    r10d,0x4
  652140:	vmovdqa xmm0,XMMWORD PTR [rdx+r10*1]
  652146:	shr    r15,0x20
  65214a:	movzx  r10d,r15b
  65214e:	shl    r10d,0x4
  652152:	lea    rcx,[rax+0xc000]
  652159:	vpternlogq xmm0,xmm1,XMMWORD PTR [rcx+r10*1],0x96
  652161:	shr    r9,0x38
  652165:	shl    r9d,0x4
  652169:	vmovdqa xmm1,XMMWORD PTR [rdi+r9*1]
  65216f:	shr    r12,0x30
  652173:	movzx  r9d,r12b
  652177:	shl    r9d,0x4
  65217b:	vpternlogq xmm1,xmm0,XMMWORD PTR [rsi+r9*1],0x96
  652183:	vmovdqu XMMWORD PTR [r14+r8*1],xmm1
  652189:	add    r8,0x10
  65218d:	cmp    r8,0xff0
  652194:	jne    651ff0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x8b0>
  65219a:	mov    rdi,rbx
  65219d:	call   QWORD PTR [rip+0x9c465]        # 6ee608 <free@GLIBC_2.2.5>
  6521a3:	movabs r13,0x7fffffffffffffc0
  6521ad:	jmp    651a4a <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x30a>
  6521b2:	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  6521c0:	mov    eax,DWORD PTR [rip+0x9ecc2]        # 6f0e88 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.17036336450247840325+0x930>
  6521c6:	test   eax,eax
  6521c8:	jne    654d16 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x35d6>
  6521ce:	mov    eax,DWORD PTR [rip+0x9fddc]        # 6f1fb0 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.17036336450247840325+0x1120>
  6521d4:	test   eax,eax
  6521d6:	jne    654d2a <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x35ea>
  6521dc:	mov    edi,0xff0
  6521e1:	mov    esi,0x1
  6521e6:	call   QWORD PTR [rip+0x9c434]        # 6ee620 <calloc@GLIBC_2.2.5>
  6521ec:	test   rax,rax
  6521ef:	je     65518b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3a4b>
  6521f5:	mov    rbx,rax
  6521f8:	mov    edx,0x1000
  6521fd:	lea    rdi,[rsp+0x23c0]
  652205:	xor    esi,esi
  652207:	call   QWORD PTR [rip+0x9c41b]        # 6ee628 <memset@GLIBC_2.2.5>
  65220d:	mov    rax,QWORD PTR [rsp+0x40]
  652212:	shr    rax,0x4
  652216:	shr    r12,0x4
  65221a:	cmp    r12,rax
  65221d:	cmovae r12,rax
  652221:	cmp    r15,r12
  652224:	cmovb  r12,r15
  652228:	test   r12,r12
  65222b:	mov    QWORD PTR [rsp+0x38],rbx
  652230:	je     652b8a <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x144a>
  652236:	xor    r14d,r14d
  652239:	mov    QWORD PTR [rsp+0x138],r12
  652241:	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  652250:	mov    QWORD PTR [rsp+0x40],r14
  652255:	shl    r14,0x4
  652259:	mov    rax,QWORD PTR [rsp+0xa8]
  652261:	mov    r15,QWORD PTR [rax+r14*1]
  652265:	mov    rbx,QWORD PTR [rax+r14*1+0x8]
  65226a:	mov    rax,QWORD PTR [rsp+0xb8]
  652272:	lea    rsi,[rax+r14*1]
  652276:	lea    rdi,[rsp+0x1fc0]
  65227e:	vzeroupper 
  652281:	call   65f1c0 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate6domainNtB2_6Domain8eval_row>
  652286:	add    r14,QWORD PTR [rsp+0xb0]
  65228e:	lea    rdi,[rsp+0x400]
  652296:	mov    rsi,r14
  652299:	call   65f1c0 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate6domainNtB2_6Domain8eval_row>
  65229e:	mov    rsi,QWORD PTR [rip+0x9e2cb]        # 6f0570 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.17036336450247840325+0x18>
  6522a5:	cmp    rsi,0x10
  6522a9:	jb     655120 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x39e0>
  6522af:	inc    QWORD PTR [rsp+0x40]
  6522b4:	mov    rax,rbx
  6522b7:	shr    rax,0x38
  6522bb:	mov    rcx,rbx
  6522be:	shr    rcx,0x30
  6522c2:	mov    rsi,rbx
  6522c5:	shr    rsi,0x28
  6522c9:	mov    rdi,rbx
  6522cc:	mov    r8,r15
  6522cf:	shr    r8,0x38
  6522d3:	mov    r9,r15
  6522d6:	shr    r9,0x30
  6522da:	mov    r10,r15
  6522dd:	shr    r10,0x28
  6522e1:	mov    r11,r15
  6522e4:	mov    rdx,QWORD PTR [rip+0x9e27d]        # 6f0568 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.17036336450247840325+0x10>
  6522eb:	movzx  r14d,r15b
  6522ef:	shl    r14d,0x4
  6522f3:	mov    r12d,r15d
  6522f6:	shr    r12d,0x4
  6522fa:	and    r12d,0xff0
  652301:	vmovdqa xmm0,XMMWORD PTR [rdx+r12*1+0x1000]
  65230b:	vpxor  xmm0,xmm0,XMMWORD PTR [rdx+r14*1]
  652311:	mov    r14d,r15d
  652314:	shr    r14d,0xc
  652318:	and    r14d,0xff0
  65231f:	shr    r15d,0x14
  652323:	and    r15d,0xfffffff0
  652327:	vmovdqa xmm1,XMMWORD PTR [rdx+r15*1+0x3000]
  652331:	vpternlogq xmm1,xmm0,XMMWORD PTR [rdx+r14*1+0x2000],0x96
  65233d:	shr    r11,0x20
  652341:	movzx  r11d,r11b
  652345:	movzx  r10d,r10b
  652349:	shl    r10d,0x4
  65234d:	vmovdqa xmm0,XMMWORD PTR [rdx+r10*1+0x5000]
  652357:	shl    r11d,0x4
  65235b:	vpternlogq xmm0,xmm1,XMMWORD PTR [rdx+r11*1+0x4000],0x96
  652367:	movzx  r9d,r9b
  65236b:	shl    r9d,0x4
  65236f:	shl    r8d,0x4
  652373:	vmovdqa xmm1,XMMWORD PTR [rdx+r8*1+0x7000]
  65237d:	vpternlogq xmm1,xmm0,XMMWORD PTR [rdx+r9*1+0x6000],0x96
  652389:	movzx  r8d,bl
  65238d:	shl    r8d,0x4
  652391:	mov    r9d,ebx
  652394:	shr    r9d,0x4
  652398:	and    r9d,0xff0
  65239f:	vmovdqa xmm0,XMMWORD PTR [rdx+r9*1+0x9000]
  6523a9:	vpternlogq xmm0,xmm1,XMMWORD PTR [rdx+r8*1+0x8000],0x96
  6523b5:	shr    rdi,0x20
  6523b9:	mov    r8d,ebx
  6523bc:	shr    r8d,0xc
  6523c0:	shr    ebx,0x14
  6523c3:	and    ebx,0xfffffff0
  6523c6:	vmovdqa xmm1,XMMWORD PTR [rdx+rbx*1+0xb000]
  6523cf:	and    r8d,0xff0
  6523d6:	vpternlogq xmm1,xmm0,XMMWORD PTR [rdx+r8*1+0xa000],0x96
  6523e2:	movzx  edi,dil
  6523e6:	shl    edi,0x4
  6523e9:	movzx  esi,sil
  6523ed:	shl    esi,0x4
  6523f0:	vmovdqa xmm0,XMMWORD PTR [rdx+rsi*1+0xd000]
  6523f9:	vpternlogq xmm0,xmm1,XMMWORD PTR [rdx+rdi*1+0xc000],0x96
  652405:	movzx  ecx,cl
  652408:	shl    ecx,0x4
  65240b:	shl    eax,0x4
  65240e:	vmovdqa xmm1,XMMWORD PTR [rdx+rax*1+0xf000]
  652417:	vpternlogq xmm1,xmm0,XMMWORD PTR [rdx+rcx*1+0xe000],0x96
  652423:	vpextrq rcx,xmm1,0x1
  652429:	vmovq  rdx,xmm1
  65242e:	mov    eax,edx
  652430:	shl    eax,0x8
  652433:	movzx  eax,ax
  652436:	mov    QWORD PTR [rsp+0x108],rax
  65243e:	mov    eax,edx
  652440:	and    eax,0xff00
  652445:	mov    QWORD PTR [rsp+0x100],rax
  65244d:	mov    eax,edx
  65244f:	shr    eax,0x8
  652452:	and    eax,0xff00
  652457:	mov    QWORD PTR [rsp+0xf8],rax
  65245f:	mov    eax,edx
  652461:	shr    eax,0x10
  652464:	and    eax,0xff00
  652469:	mov    QWORD PTR [rsp+0xf0],rax
  652471:	mov    rax,rdx
  652474:	shr    rax,0x18
  652478:	and    eax,0xff00
  65247d:	mov    QWORD PTR [rsp+0xe8],rax
  652485:	mov    rax,rdx
  652488:	shr    rax,0x20
  65248c:	and    eax,0xff00
  652491:	mov    QWORD PTR [rsp+0xe0],rax
  652499:	mov    rax,rdx
  65249c:	shr    rax,0x28
  6524a0:	and    eax,0xff00
  6524a5:	mov    QWORD PTR [rsp+0xd8],rax
  6524ad:	shr    rdx,0x30
  6524b1:	and    edx,0xff00
  6524b7:	mov    QWORD PTR [rsp+0xd0],rdx
  6524bf:	mov    eax,ecx
  6524c1:	shl    eax,0x8
  6524c4:	movzx  eax,ax
  6524c7:	mov    QWORD PTR [rsp+0x3c0],rax
  6524cf:	mov    eax,ecx
  6524d1:	and    eax,0xff00
  6524d6:	mov    QWORD PTR [rsp+0xc8],rax
  6524de:	mov    eax,ecx
  6524e0:	shr    eax,0x8
  6524e3:	and    eax,0xff00
  6524e8:	mov    QWORD PTR [rsp+0xc0],rax
  6524f0:	mov    eax,ecx
  6524f2:	shr    eax,0x10
  6524f5:	and    eax,0xff00
  6524fa:	mov    QWORD PTR [rsp+0x168],rax
  652502:	mov    rax,rcx
  652505:	shr    rax,0x18
  652509:	and    eax,0xff00
  65250e:	mov    QWORD PTR [rsp+0x160],rax
  652516:	mov    rax,rcx
  652519:	shr    rax,0x20
  65251d:	and    eax,0xff00
  652522:	mov    QWORD PTR [rsp+0x158],rax
  65252a:	mov    rax,rcx
  65252d:	shr    rax,0x28
  652531:	and    eax,0xff00
  652536:	mov    QWORD PTR [rsp+0x150],rax
  65253e:	shr    rcx,0x30
  652542:	and    ecx,0xff00
  652548:	mov    QWORD PTR [rsp+0x148],rcx
  652550:	mov    rdi,QWORD PTR [rip+0x9e009]        # 6f0560 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.17036336450247840325+0x8>
  652557:	mov    r14,QWORD PTR [rip+0x9dffa]        # 6f0558 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.17036336450247840325>
  65255e:	lea    r12,[rsp+0xfc0]
  652566:	xor    ecx,ecx
  652568:	mov    QWORD PTR [rsp+0x30],rdi
  65256d:	mov    QWORD PTR [rsp+0x140],r14
  652575:	data16 cs nop WORD PTR [rax+rax*1+0x0]
  652580:	mov    al,0x1
  652582:	shl    al,cl
  652584:	movzx  r9d,al
  652588:	mov    rsi,QWORD PTR [rsp+0x108]
  652590:	or     rsi,r9
  652593:	cmp    rsi,rdi
  652596:	jae    655092 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3952>
  65259c:	mov    r13,QWORD PTR [rsp+0x100]
  6525a4:	or     r13,r9
  6525a7:	cmp    r13,rdi
  6525aa:	mov    rbx,QWORD PTR [rsp+0x38]
  6525af:	jae    6550b7 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3977>
  6525b5:	mov    rax,QWORD PTR [rsp+0xf8]
  6525bd:	or     rax,r9
  6525c0:	cmp    rax,rdi
  6525c3:	jae    6550bc <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x397c>
  6525c9:	mov    r15,QWORD PTR [rsp+0xf0]
  6525d1:	or     r15,r9
  6525d4:	cmp    r15,rdi
  6525d7:	jae    6550c1 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3981>
  6525dd:	mov    r8,QWORD PTR [rsp+0xe8]
  6525e5:	or     r8,r9
  6525e8:	cmp    r8,rdi
  6525eb:	jae    655071 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3931>
  6525f1:	mov    rdx,QWORD PTR [rsp+0xe0]
  6525f9:	or     rdx,r9
  6525fc:	cmp    rdx,rdi
  6525ff:	jae    65506c <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x392c>
  652605:	mov    r10,QWORD PTR [rsp+0xd8]
  65260d:	or     r10,r9
  652610:	cmp    r10,rdi
  652613:	jae    655076 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3936>
  652619:	mov    r11,QWORD PTR [rsp+0xd0]
  652621:	or     r11,r9
  652624:	cmp    r11,rdi
  652627:	jae    655080 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3940>
  65262d:	mov    QWORD PTR [rsp+0x48],rdx
  652632:	mov    rdx,QWORD PTR [rsp+0x3c0]
  65263a:	or     rdx,r9
  65263d:	cmp    rdx,rdi
  652640:	jae    65506c <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x392c>
  652646:	mov    QWORD PTR [rsp+0x280],r8
  65264e:	mov    r8,QWORD PTR [rsp+0xc8]
  652656:	or     r8,r9
  652659:	cmp    r8,rdi
  65265c:	jae    655071 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3931>
  652662:	mov    r14,QWORD PTR [rsp+0xc0]
  65266a:	or     r14,r9
  65266d:	cmp    r14,rdi
  652670:	jae    6550b2 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3972>
  652676:	mov    rbx,QWORD PTR [rsp+0x168]
  65267e:	or     rbx,r9
  652681:	cmp    rbx,rdi
  652684:	jae    65508a <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x394a>
  65268a:	mov    QWORD PTR [rsp+0x58],r15
  65268f:	mov    QWORD PTR [rsp+0x240],rcx
  652697:	mov    r15,r12
  65269a:	mov    r12,QWORD PTR [rsp+0x160]
  6526a2:	or     r12,r9
  6526a5:	cmp    r12,rdi
  6526a8:	jae    65507b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x393b>
  6526ae:	mov    QWORD PTR [rsp+0x50],rax
  6526b3:	mov    rax,QWORD PTR [rsp+0x158]
  6526bb:	or     rax,r9
  6526be:	cmp    rax,rdi
  6526c1:	jae    65508f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x394f>
  6526c7:	mov    QWORD PTR [rsp+0x110],rsi
  6526cf:	mov    rsi,QWORD PTR [rsp+0x150]
  6526d7:	or     rsi,r9
  6526da:	cmp    rsi,rdi
  6526dd:	jae    655092 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3952>
  6526e3:	or     r9,QWORD PTR [rsp+0x148]
  6526eb:	cmp    r9,rdi
  6526ee:	jae    655085 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3945>
  6526f4:	mov    rdi,QWORD PTR [rsp+0x140]
  6526fc:	movzx  ecx,BYTE PTR [rdi+r8*1]
  652701:	shl    ecx,0x8
  652704:	movzx  edx,BYTE PTR [rdi+rdx*1]
  652708:	or     rdx,rcx
  65270b:	movzx  ecx,BYTE PTR [rdi+r14*1]
  652710:	mov    r14,rdi
  652713:	shl    ecx,0x10
  652716:	or     rdx,rcx
  652719:	movzx  ecx,BYTE PTR [rdi+rbx*1]
  65271d:	shl    ecx,0x18
  652720:	or     rdx,rcx
  652723:	movzx  ecx,BYTE PTR [rdi+r12*1]
  652728:	shl    rcx,0x20
  65272c:	or     rdx,rcx
  65272f:	movzx  eax,BYTE PTR [rdi+rax*1]
  652733:	shl    rax,0x28
  652737:	or     rdx,rax
  65273a:	movzx  eax,BYTE PTR [rdi+rsi*1]
  65273e:	shl    rax,0x30
  652742:	or     rdx,rax
  652745:	movzx  eax,BYTE PTR [rdi+r9*1]
  65274a:	shl    rax,0x38
  65274e:	or     rdx,rax
  652751:	movzx  ecx,BYTE PTR [rdi+r13*1]
  652756:	shl    ecx,0x8
  652759:	mov    rax,QWORD PTR [rsp+0x110]
  652761:	movzx  eax,BYTE PTR [rdi+rax*1]
  652765:	or     rax,rcx
  652768:	mov    rcx,QWORD PTR [rsp+0x50]
  65276d:	movzx  ecx,BYTE PTR [rdi+rcx*1]
  652771:	shl    ecx,0x10
  652774:	or     rax,rcx
  652777:	mov    rcx,QWORD PTR [rsp+0x58]
  65277c:	movzx  ecx,BYTE PTR [rdi+rcx*1]
  652780:	shl    ecx,0x18
  652783:	or     rax,rcx
  652786:	mov    rcx,QWORD PTR [rsp+0x280]
  65278e:	movzx  ecx,BYTE PTR [rdi+rcx*1]
  652792:	shl    rcx,0x20
  652796:	or     rax,rcx
  652799:	mov    rcx,QWORD PTR [rsp+0x48]
  65279e:	movzx  ecx,BYTE PTR [rdi+rcx*1]
  6527a2:	shl    rcx,0x28
  6527a6:	or     rax,rcx
  6527a9:	movzx  ecx,BYTE PTR [rdi+r10*1]
  6527ae:	shl    rcx,0x30
  6527b2:	or     rax,rcx
  6527b5:	movzx  ecx,BYTE PTR [rdi+r11*1]
  6527ba:	shl    rcx,0x38
  6527be:	or     rax,rcx
  6527c1:	mov    QWORD PTR [r15+0x8],rdx
  6527c5:	mov    QWORD PTR [r15],rax
  6527c8:	mov    rcx,QWORD PTR [rsp+0x240]
  6527d0:	inc    rcx
  6527d3:	add    r15,0x10
  6527d7:	cmp    rcx,0x8
  6527db:	movabs r13,0x7fffffffffffffc0
  6527e5:	mov    rdi,QWORD PTR [rsp+0x30]
  6527ea:	mov    r12,r15
  6527ed:	jne    652580 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0xe40>
  6527f3:	vmovups ymm0,YMMWORD PTR [rsp+0xfc0]
  6527fc:	vmovups ymm1,YMMWORD PTR [rsp+0xfe0]
  652805:	vmovups ymm2,YMMWORD PTR [rsp+0x1000]
  65280e:	vmovdqu ymm3,YMMWORD PTR [rsp+0x1020]
  652817:	vmovdqa YMMWORD PTR [rsp+0x860],ymm3
  652820:	vmovaps YMMWORD PTR [rsp+0x840],ymm2
  652829:	vmovaps YMMWORD PTR [rsp+0x820],ymm1
  652832:	vmovaps YMMWORD PTR [rsp+0x800],ymm0
  65283b:	lea    rcx,[rsp+0x23f0]
  652843:	xor    edx,edx
  652845:	mov    r12,QWORD PTR [rsp+0x138]
  65284d:	nop    DWORD PTR [rax]
  652850:	lea    rsi,[rdx+0x1]
  652854:	lea    rax,[rdx+0x2]
  652858:	lea    rdi,[r13+rdx*1+0x40]
  65285d:	and    rdi,rsi
  652860:	shl    rdi,0x4
  652864:	tzcnt  r8,rsi
  652869:	shl    r8d,0x4
  65286d:	vmovaps xmm0,XMMWORD PTR [rsp+r8*1+0x800]
  652877:	vxorps xmm0,xmm0,XMMWORD PTR [rsp+rdi*1+0x23c0]
  652880:	vmovaps XMMWORD PTR [rcx-0x20],xmm0
  652885:	and    rsi,rax
  652888:	shl    rsi,0x4
  65288c:	vmovq  xmm0,QWORD PTR [rsp+rsi*1+0x23c8]
  652895:	add    rdx,0x3
  652899:	vmovq  xmm1,QWORD PTR [rsp+rsi*1+0x23c0]
  6528a2:	vpunpcklqdq xmm0,xmm1,xmm0
  6528a6:	tzcnt  rsi,rax
  6528ab:	shl    esi,0x4
  6528ae:	vmovq  xmm1,QWORD PTR [rsp+rsi*1+0x808]
  6528b7:	vmovq  xmm2,QWORD PTR [rsp+rsi*1+0x800]
  6528c0:	vpunpcklqdq xmm1,xmm2,xmm1
  6528c4:	vpxor  xmm0,xmm1,xmm0
  6528c8:	vpextrq QWORD PTR [rcx-0x8],xmm0,0x1
  6528cf:	vmovq  QWORD PTR [rcx-0x10],xmm0
  6528d4:	and    rax,rdx
  6528d7:	shl    rax,0x4
  6528db:	vmovq  xmm0,QWORD PTR [rsp+rax*1+0x23c8]
  6528e4:	vmovq  xmm1,QWORD PTR [rsp+rax*1+0x23c0]
  6528ed:	vpunpcklqdq xmm0,xmm1,xmm0
  6528f1:	tzcnt  rax,rdx
  6528f6:	shl    eax,0x4
  6528f9:	vmovq  xmm1,QWORD PTR [rsp+rax*1+0x808]
  652902:	vmovq  xmm2,QWORD PTR [rsp+rax*1+0x800]
  65290b:	vpunpcklqdq xmm1,xmm2,xmm1
  65290f:	vpxor  xmm0,xmm1,xmm0
  652913:	vmovdqa XMMWORD PTR [rcx],xmm0
  652917:	add    rcx,0x30
  65291b:	cmp    rdx,0xff
  652922:	jne    652850 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1110>
  652928:	vmovups ymm0,YMMWORD PTR [rsp+0x209f]
  652931:	lea    rcx,[rsp+0xff0]
  652939:	vmovups YMMWORD PTR [rcx+0xcf],ymm0
  652941:	vmovups ymm0,YMMWORD PTR [rsp+0x2080]
  65294a:	vmovups YMMWORD PTR [rcx+0xb0],ymm0
  652952:	vmovups ymm0,YMMWORD PTR [rsp+0x2060]
  65295b:	vmovups YMMWORD PTR [rcx+0x90],ymm0
  652963:	vmovups ymm0,YMMWORD PTR [rsp+0x2040]
  65296c:	vmovups YMMWORD PTR [rcx+0x70],ymm0
  652971:	vmovups ymm0,YMMWORD PTR [rsp+0x1fc0]
  65297a:	vmovups ymm1,YMMWORD PTR [rsp+0x1fe0]
  652983:	vmovups ymm2,YMMWORD PTR [rsp+0x2000]
  65298c:	vmovups ymm3,YMMWORD PTR [rsp+0x2020]
  652995:	vmovups YMMWORD PTR [rcx+0x50],ymm3
  65299a:	vmovups YMMWORD PTR [rcx+0x30],ymm2
  65299f:	vmovups YMMWORD PTR [rcx+0x10],ymm1
  6529a4:	vmovups YMMWORD PTR [rcx-0x10],ymm0
  6529a9:	lea    rax,[rsp+0x230]
  6529b1:	vpxor  xmm4,xmm4,xmm4
  6529b5:	vmovdqu XMMWORD PTR [rax],xmm4
  6529b9:	vmovups ymm0,YMMWORD PTR [rsp+0x4df]
  6529c2:	vmovups YMMWORD PTR [rcx+0x1ef],ymm0
  6529ca:	vmovups ymm0,YMMWORD PTR [rsp+0x4c0]
  6529d3:	vmovups YMMWORD PTR [rcx+0x1d0],ymm0
  6529db:	vmovups ymm0,YMMWORD PTR [rsp+0x4a0]
  6529e4:	vmovups YMMWORD PTR [rcx+0x1b0],ymm0
  6529ec:	vmovups ymm0,YMMWORD PTR [rsp+0x480]
  6529f5:	vmovups YMMWORD PTR [rcx+0x190],ymm0
  6529fd:	vmovups ymm0,YMMWORD PTR [rsp+0x400]
  652a06:	vmovdqu ymm1,YMMWORD PTR [rsp+0x420]
  652a0f:	vmovdqu ymm2,YMMWORD PTR [rsp+0x440]
  652a18:	vmovdqu ymm3,YMMWORD PTR [rsp+0x460]
  652a21:	vmovdqu YMMWORD PTR [rcx+0x170],ymm3
  652a29:	vmovdqu YMMWORD PTR [rcx+0x150],ymm2
  652a31:	vmovdqu YMMWORD PTR [rcx+0x130],ymm1
  652a39:	vmovups YMMWORD PTR [rcx+0x110],ymm0
  652a41:	movzx  eax,BYTE PTR [rsp+0x23f]
  652a49:	vmovups xmm0,XMMWORD PTR [rsp+0x22f]
  652a52:	vmovups XMMWORD PTR [rcx+0xef],xmm0
  652a5a:	mov    BYTE PTR [rcx+0xff],al
  652a60:	lea    rax,[rsp+0x21f]
  652a68:	vmovdqu XMMWORD PTR [rax],xmm4
  652a6c:	mov    QWORD PTR [rsp+0x10f0],0x0
  652a78:	mov    QWORD PTR [rsp+0x10f8],0xff
  652a84:	movzx  eax,BYTE PTR [rsp+0x22e]
  652a8c:	mov    BYTE PTR [rcx+0x21f],al
  652a92:	vmovdqu xmm0,XMMWORD PTR [rsp+0x21e]
  652a9b:	vmovdqu XMMWORD PTR [rcx+0x20f],xmm0
  652aa3:	lea    rcx,[rsp+0x1102]
  652aab:	xor    edx,edx
  652aad:	mov    rbx,QWORD PTR [rsp+0x38]
  652ab2:	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  652ac0:	movzx  edi,BYTE PTR [rcx-0x122]
  652ac7:	movzx  eax,BYTE PTR [rcx-0x2]
  652acb:	shl    edi,0x8
  652ace:	or     rdi,rax
  652ad1:	mov    rsi,QWORD PTR [rsp+0x30]
  652ad6:	cmp    rdi,rsi
  652ad9:	jae    655012 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x38d2>
  652adf:	movzx  eax,BYTE PTR [r14+rdi*1]
  652ae4:	shl    eax,0x4
  652ae7:	vmovdqa xmm0,XMMWORD PTR [rbx+rdx*1]
  652aec:	vpxor  xmm0,xmm0,XMMWORD PTR [rsp+rax*1+0x23c0]
  652af5:	vmovdqa XMMWORD PTR [rbx+rdx*1],xmm0
  652afa:	movzx  edi,BYTE PTR [rcx-0x121]
  652b01:	movzx  eax,BYTE PTR [rcx-0x1]
  652b05:	shl    edi,0x8
  652b08:	or     rdi,rax
  652b0b:	cmp    rdi,rsi
  652b0e:	jae    655012 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x38d2>
  652b14:	movzx  eax,BYTE PTR [r14+rdi*1]
  652b19:	shl    eax,0x4
  652b1c:	vmovdqa xmm0,XMMWORD PTR [rbx+rdx*1+0x10]
  652b22:	vpxor  xmm0,xmm0,XMMWORD PTR [rsp+rax*1+0x23c0]
  652b2b:	vmovdqa XMMWORD PTR [rbx+rdx*1+0x10],xmm0
  652b31:	movzx  edi,BYTE PTR [rcx-0x120]
  652b38:	movzx  eax,BYTE PTR [rcx]
  652b3b:	shl    edi,0x8
  652b3e:	or     rdi,rax
  652b41:	cmp    rdi,rsi
  652b44:	jae    655012 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x38d2>
  652b4a:	movzx  eax,BYTE PTR [r14+rdi*1]
  652b4f:	shl    eax,0x4
  652b52:	vmovdqa xmm0,XMMWORD PTR [rbx+rdx*1+0x20]
  652b58:	vpxor  xmm0,xmm0,XMMWORD PTR [rsp+rax*1+0x23c0]
  652b61:	vmovdqa XMMWORD PTR [rbx+rdx*1+0x20],xmm0
  652b67:	add    rdx,0x30
  652b6b:	add    rcx,0x3
  652b6f:	cmp    rdx,0xff0
  652b76:	jne    652ac0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1380>
  652b7c:	mov    r14,QWORD PTR [rsp+0x40]
  652b81:	cmp    r14,r12
  652b84:	jne    652250 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0xb10>
  652b8a:	mov    edi,0xff0
  652b8f:	vzeroupper 
  652b92:	call   QWORD PTR [rip+0x9ba60]        # 6ee5f8 <malloc@GLIBC_2.2.5>
  652b98:	test   rax,rax
  652b9b:	je     6551c8 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3a88>
  652ba1:	mov    r14,rax
  652ba4:	mov    edx,0xff0
  652ba9:	mov    rdi,rax
  652bac:	mov    rsi,rbx
  652baf:	call   QWORD PTR [rip+0x9ba63]        # 6ee618 <memcpy@GLIBC_2.14>
  652bb5:	mov    rdi,rbx
  652bb8:	call   QWORD PTR [rip+0x9ba4a]        # 6ee608 <free@GLIBC_2.2.5>
  652bbe:	jmp    651a4a <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x30a>
  652bc3:	mov    DWORD PTR [rsp+0x74],0x0
  652bcb:	jmp    652bd3 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1493>
  652bcd:	mov    al,0x1
  652bcf:	mov    DWORD PTR [rsp+0x74],eax
  652bd3:	mov    eax,DWORD PTR [rip+0x9f3d7]        # 6f1fb0 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.17036336450247840325+0x1120>
  652bd9:	test   eax,eax
  652bdb:	jne    654d34 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x35f4>
  652be1:	mov    eax,DWORD PTR [rip+0x9e2a1]        # 6f0e88 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.17036336450247840325+0x930>
  652be7:	test   eax,eax
  652be9:	jne    654d3e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x35fe>
  652bef:	mov    QWORD PTR [rsp+0x1b0],r15
  652bf7:	mov    edx,0x1000
  652bfc:	lea    rdi,[rsp+0xfc0]
  652c04:	xor    esi,esi
  652c06:	mov    r15,QWORD PTR [rip+0x9ba1b]        # 6ee628 <memset@GLIBC_2.2.5>
  652c0d:	call   r15
  652c10:	mov    edx,0x400
  652c15:	lea    rdi,[rsp+0x800]
  652c1d:	xor    esi,esi
  652c1f:	call   r15
  652c22:	mov    edx,0x400
  652c27:	lea    rdi,[rsp+0x1fc0]
  652c2f:	xor    esi,esi
  652c31:	call   r15
  652c34:	cmp    QWORD PTR [rsp+0xa0],r14
  652c3c:	jne    652c5a <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x151a>
  652c3e:	xor    eax,eax
  652c40:	mov    rcx,QWORD PTR [rsp+0x98]
  652c48:	cmp    rcx,rbx
  652c4b:	mov    rsi,QWORD PTR [rsp+0x1b0]
  652c53:	jne    652c96 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1556>
  652c55:	jmp    654050 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2910>
  652c5a:	mov    rax,QWORD PTR [rsp+0x40]
  652c5f:	lea    rcx,[rax+0x3ff]
  652c66:	shr    rcx,0xa
  652c6a:	lea    rax,[r12+0x3ff]
  652c72:	shr    rax,0xa
  652c76:	cmp    rax,rcx
  652c79:	cmovae rax,rcx
  652c7d:	mov    rcx,QWORD PTR [rsp+0x98]
  652c85:	cmp    rcx,rbx
  652c88:	mov    rsi,QWORD PTR [rsp+0x1b0]
  652c90:	je     654050 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2910>
  652c96:	lea    rcx,[rsi+0x3f]
  652c9a:	shr    rcx,0x6
  652c9e:	cmp    rcx,rax
  652ca1:	cmovae rcx,rax
  652ca5:	mov    QWORD PTR [rsp+0x1f8],rcx
  652cad:	test   rcx,rcx
  652cb0:	je     654050 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2910>
  652cb6:	mov    rcx,QWORD PTR [rip+0x9d8ab]        # 6f0568 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.17036336450247840325+0x10>
  652cbd:	mov    rax,QWORD PTR [rip+0x9d8ac]        # 6f0570 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.17036336450247840325+0x18>
  652cc4:	mov    QWORD PTR [rsp+0x90],rax
  652ccc:	lea    r9,[rcx+0x1000]
  652cd3:	lea    rax,[rcx+0x2000]
  652cda:	mov    QWORD PTR [rsp+0x280],rax
  652ce2:	lea    rax,[rcx+0x3000]
  652ce9:	mov    QWORD PTR [rsp+0x240],rax
  652cf1:	lea    rax,[rcx+0x4000]
  652cf8:	mov    QWORD PTR [rsp+0x58],rax
  652cfd:	lea    rax,[rcx+0x5000]
  652d04:	mov    QWORD PTR [rsp+0x50],rax
  652d09:	lea    rax,[rcx+0x6000]
  652d10:	mov    QWORD PTR [rsp+0x110],rax
  652d18:	lea    rax,[rcx+0x7000]
  652d1f:	mov    QWORD PTR [rsp+0x38],rax
  652d24:	lea    rax,[rcx+0x8000]
  652d2b:	mov    QWORD PTR [rsp+0x108],rax
  652d33:	lea    rax,[rcx+0x9000]
  652d3a:	mov    QWORD PTR [rsp+0x100],rax
  652d42:	lea    rax,[rcx+0xa000]
  652d49:	mov    QWORD PTR [rsp+0xf8],rax
  652d51:	mov    r14,QWORD PTR [rip+0x9f140]        # 6f1e98 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.17036336450247840325+0x1008>
  652d58:	add    r14,QWORD PTR [rip+0x9f179]        # 6f1ed8 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.17036336450247840325+0x1048>
  652d5f:	lea    rax,[rcx+0xb000]
  652d66:	mov    QWORD PTR [rsp+0xf0],rax
  652d6e:	lea    rax,[rcx+0xc000]
  652d75:	mov    QWORD PTR [rsp+0xe8],rax
  652d7d:	lea    rax,[rcx+0xd000]
  652d84:	mov    QWORD PTR [rsp+0xe0],rax
  652d8c:	lea    rax,[rcx+0xe000]
  652d93:	mov    QWORD PTR [rsp+0xd8],rax
  652d9b:	lea    rax,[rcx+0xf000]
  652da2:	mov    QWORD PTR [rsp+0xd0],rax
  652daa:	mov    rax,QWORD PTR [rsp+0x128]
  652db2:	mov    QWORD PTR [rsp+0x1c8],rax
  652dba:	mov    rax,QWORD PTR [rsp+0x120]
  652dc2:	mov    QWORD PTR [rsp+0x1c0],rax
  652dca:	mov    rax,QWORD PTR [rsp+0x178]
  652dd2:	mov    QWORD PTR [rsp+0x1b8],rax
  652dda:	mov    rax,QWORD PTR [rsp+0x180]
  652de2:	mov    QWORD PTR [rsp+0xc8],rax
  652dea:	mov    rdx,r12
  652ded:	mov    rdi,QWORD PTR [rsp+0x40]
  652df2:	mov    rax,rsi
  652df5:	xor    r8d,r8d
  652df8:	mov    QWORD PTR [rsp+0x138],r12
  652e00:	mov    QWORD PTR [rsp+0x1a8],rcx
  652e08:	mov    QWORD PTR [rsp+0x1a0],r9
  652e10:	jmp    652f6f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x182f>
  652e15:	data16 cs nop WORD PTR [rax+rax*1+0x0]
  652e20:	inc    QWORD PTR [rsp+0xc0]
  652e28:	lea    rdi,[rsp+0xfc0]
  652e30:	lea    r12,[rsp+0x23c0]
  652e38:	mov    rsi,r12
  652e3b:	mov    r15,QWORD PTR [rsp+0x150]
  652e43:	mov    rdx,r15
  652e46:	lea    rcx,[rsp+0x800]
  652e4e:	mov    r8,r15
  652e51:	lea    r9,[rsp+0x1fc0]
  652e59:	push   0x0
  652e5b:	push   r15
  652e5d:	vzeroupper 
  652e60:	call   6677d0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled12consume_four>
  652e65:	add    rsp,0x10
  652e69:	lea    rbx,[rsp+0xfc0]
  652e71:	mov    rdi,rbx
  652e74:	mov    rsi,r12
  652e77:	mov    rdx,r15
  652e7a:	lea    r12,[rsp+0x800]
  652e82:	mov    rcx,r12
  652e85:	mov    r8,r15
  652e88:	lea    r9,[rsp+0x1fc0]
  652e90:	push   0x4
  652e92:	push   r15
  652e94:	call   6677d0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled12consume_four>
  652e99:	add    rsp,0x10
  652e9d:	mov    rdi,rbx
  652ea0:	lea    rsi,[rsp+0x23c0]
  652ea8:	mov    rdx,r15
  652eab:	mov    rcx,r12
  652eae:	mov    r8,r15
  652eb1:	lea    r9,[rsp+0x1fc0]
  652eb9:	push   0x8
  652ebb:	push   r15
  652ebd:	call   6677d0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled12consume_four>
  652ec2:	add    rsp,0x10
  652ec6:	mov    rdi,rbx
  652ec9:	lea    rsi,[rsp+0x23c0]
  652ed1:	mov    rdx,r15
  652ed4:	mov    rcx,r12
  652ed7:	mov    r8,r15
  652eda:	lea    r9,[rsp+0x1fc0]
  652ee2:	push   0xc
  652ee4:	push   r15
  652ee6:	call   6677d0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled12consume_four>
  652eeb:	mov    r8,QWORD PTR [rsp+0xd0]
  652ef3:	add    rsp,0x10
  652ef7:	mov    rax,QWORD PTR [rsp+0x158]
  652eff:	add    rax,0xffffffffffffffc0
  652f03:	mov    rdi,QWORD PTR [rsp+0x160]
  652f0b:	add    rdi,0xfffffffffffffc00
  652f12:	mov    rdx,QWORD PTR [rsp+0x168]
  652f1a:	add    rdx,0xfffffffffffffc00
  652f21:	add    QWORD PTR [rsp+0xc8],0x400
  652f2d:	add    QWORD PTR [rsp+0x1b8],0x400
  652f39:	add    QWORD PTR [rsp+0x1c0],0x400
  652f45:	add    QWORD PTR [rsp+0x1c8],0x400
  652f51:	cmp    r8,QWORD PTR [rsp+0x1f8]
  652f59:	mov    r12,QWORD PTR [rsp+0x138]
  652f61:	mov    rsi,QWORD PTR [rsp+0x1b0]
  652f69:	je     654050 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2910>
  652f6f:	cmp    rax,0x40
  652f73:	mov    r10d,0x40
  652f79:	cmovb  r10,rax
  652f7d:	cmp    rax,0x1
  652f81:	mov    QWORD PTR [rsp+0x158],rax
  652f89:	adc    rax,0x0
  652f8d:	cmp    rax,0x40
  652f91:	mov    ecx,0x40
  652f96:	cmovae rax,rcx
  652f9a:	cmp    rdi,0x400
  652fa1:	mov    ecx,0x400
  652fa6:	mov    QWORD PTR [rsp+0x160],rdi
  652fae:	cmovb  rcx,rdi
  652fb2:	shr    ecx,0x4
  652fb5:	cmp    rdx,0x400
  652fbc:	mov    edi,0x400
  652fc1:	mov    QWORD PTR [rsp+0x168],rdx
  652fc9:	cmovb  rdi,rdx
  652fcd:	shr    edi,0x4
  652fd0:	cmp    rcx,rdi
  652fd3:	cmovb  rdi,rcx
  652fd7:	mov    QWORD PTR [rsp+0x48],rdi
  652fdc:	mov    rdx,r8
  652fdf:	shl    rdx,0xa
  652fe3:	mov    rdi,QWORD PTR [rsp+0x40]
  652fe8:	sub    rdi,rdx
  652feb:	cmp    rdi,0x400
  652ff2:	mov    ecx,0x400
  652ff7:	cmovae rdi,rcx
  652ffb:	mov    QWORD PTR [rsp+0x148],rdi
  653003:	mov    r9,r12
  653006:	mov    QWORD PTR [rsp+0x30],rdx
  65300b:	sub    r9,rdx
  65300e:	cmp    r9,0x400
  653015:	cmovae r9,rcx
  653019:	mov    QWORD PTR [rsp+0x140],r9
  653021:	mov    QWORD PTR [rsp+0xc0],r8
  653029:	mov    rcx,r8
  65302c:	shl    rcx,0x6
  653030:	mov    rdx,rsi
  653033:	sub    rdx,rcx
  653036:	cmp    rdx,0x40
  65303a:	mov    ebx,0x40
  65303f:	cmovb  rbx,rdx
  653043:	cmp    BYTE PTR [rsp+0x74],0x0
  653048:	mov    QWORD PTR [rsp+0x150],rbx
  653050:	je     6535d0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1e90>
  653056:	cmp    rdx,0x40
  65305a:	jb     653a43 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2303>
  653060:	mov    r15,r10
  653063:	mov    rax,QWORD PTR [rsp+0xa8]
  65306b:	mov    rcx,QWORD PTR [rsp+0x30]
  653070:	add    rax,rcx
  653073:	vmovdqu64 zmm0,ZMMWORD PTR [rax]
  653079:	vmovdqu64 zmm1,ZMMWORD PTR [rax+0x40]
  653080:	vmovdqu64 zmm2,ZMMWORD PTR [rax+0x80]
  653087:	vmovdqu64 zmm3,ZMMWORD PTR [rax+0xc0]
  65308e:	vmovdqu64 zmm4,ZMMWORD PTR [rax+0x100]
  653095:	vmovdqu64 zmm5,ZMMWORD PTR [rax+0x140]
  65309c:	vmovdqu64 zmm6,ZMMWORD PTR [rax+0x180]
  6530a3:	vmovdqu64 zmm7,ZMMWORD PTR [rax+0x1c0]
  6530aa:	vmovdqu64 zmm8,ZMMWORD PTR [rax+0x200]
  6530b1:	vmovdqu64 zmm9,ZMMWORD PTR [rax+0x240]
  6530b8:	vmovdqu64 zmm10,ZMMWORD PTR [rax+0x280]
  6530bf:	vmovdqu64 zmm11,ZMMWORD PTR [rax+0x2c0]
  6530c6:	vmovdqu64 zmm12,ZMMWORD PTR [rax+0x300]
  6530cd:	vmovdqu64 zmm13,ZMMWORD PTR [rax+0x340]
  6530d4:	vmovdqu64 zmm14,ZMMWORD PTR [rax+0x380]
  6530db:	vmovdqu64 zmm15,ZMMWORD PTR [rax+0x3c0]
  6530e2:	vpunpcklbw zmm16,zmm0,zmm1
  6530e8:	vpunpckhbw zmm0,zmm0,zmm1
  6530ee:	vpunpcklbw zmm1,zmm2,zmm3
  6530f4:	vpunpckhbw zmm2,zmm2,zmm3
  6530fa:	vpunpcklbw zmm3,zmm4,zmm5
  653100:	vpunpckhbw zmm4,zmm4,zmm5
  653106:	vpunpcklbw zmm5,zmm6,zmm7
  65310c:	vpunpckhbw zmm6,zmm6,zmm7
  653112:	vpunpcklbw zmm7,zmm8,zmm9
  653118:	vpunpckhbw zmm8,zmm8,zmm9
  65311e:	vpunpcklbw zmm9,zmm10,zmm11
  653124:	vpunpckhbw zmm10,zmm10,zmm11
  65312a:	vpunpcklbw zmm11,zmm12,zmm13
  653130:	vpunpckhbw zmm12,zmm12,zmm13
  653136:	vpunpcklbw zmm13,zmm14,zmm15
  65313c:	vpunpckhbw zmm14,zmm14,zmm15
  653142:	vpunpcklwd zmm15,zmm16,zmm1
  653148:	vpunpckhwd zmm1,zmm16,zmm1
  65314e:	vpunpcklwd zmm16,zmm0,zmm2
  653154:	vpunpckhwd zmm0,zmm0,zmm2
  65315a:	vpunpcklwd zmm2,zmm3,zmm5
  653160:	vpunpckhwd zmm3,zmm3,zmm5
  653166:	vpunpcklwd zmm5,zmm4,zmm6
  65316c:	vpunpckhwd zmm4,zmm4,zmm6
  653172:	vpunpcklwd zmm6,zmm7,zmm9
  653178:	vpunpckhwd zmm7,zmm7,zmm9
  65317e:	vpunpcklwd zmm9,zmm8,zmm10
  653184:	vpunpckhwd zmm8,zmm8,zmm10
  65318a:	vpunpcklwd zmm10,zmm11,zmm13
  653190:	vpunpckhwd zmm11,zmm11,zmm13
  653196:	vpunpcklwd zmm13,zmm12,zmm14
  65319c:	vpunpckhwd zmm12,zmm12,zmm14
  6531a2:	vpunpckldq zmm14,zmm15,zmm2
  6531a8:	vpunpckhdq zmm2,zmm15,zmm2
  6531ae:	vpunpckldq zmm15,zmm1,zmm3
  6531b4:	vpunpckhdq zmm1,zmm1,zmm3
  6531ba:	vpunpckldq zmm3,zmm16,zmm5
  6531c0:	vpunpckhdq zmm5,zmm16,zmm5
  6531c6:	vpunpckldq zmm16,zmm0,zmm4
  6531cc:	vpunpckhdq zmm0,zmm0,zmm4
  6531d2:	vpunpckldq zmm4,zmm6,zmm10
  6531d8:	vpunpckhdq zmm6,zmm6,zmm10
  6531de:	vpunpckldq zmm10,zmm7,zmm11
  6531e4:	vpunpckhdq zmm7,zmm7,zmm11
  6531ea:	vpunpckldq zmm11,zmm9,zmm13
  6531f0:	vpunpckhdq zmm9,zmm9,zmm13
  6531f6:	vpunpckldq zmm13,zmm8,zmm12
  6531fc:	vpunpckhdq zmm8,zmm8,zmm12
  653202:	vpunpcklqdq zmm12,zmm14,zmm4
  653208:	vmovdqa64 ZMMWORD PTR [rsp+0xf80],zmm12
  653210:	vpunpckhqdq zmm4,zmm14,zmm4
  653216:	vmovdqa64 ZMMWORD PTR [rsp+0xf40],zmm4
  65321e:	vpunpcklqdq zmm4,zmm2,zmm6
  653224:	vmovdqa64 ZMMWORD PTR [rsp+0xf00],zmm4
  65322c:	vpunpckhqdq zmm2,zmm2,zmm6
  653232:	vmovdqa64 ZMMWORD PTR [rsp+0xec0],zmm2
  65323a:	vpunpcklqdq zmm2,zmm15,zmm10
  653240:	vmovdqa64 ZMMWORD PTR [rsp+0xe80],zmm2
  653248:	vpunpckhqdq zmm2,zmm15,zmm10
  65324e:	vmovdqa64 ZMMWORD PTR [rsp+0xe40],zmm2
  653256:	vpunpcklqdq zmm2,zmm1,zmm7
  65325c:	vmovdqa64 ZMMWORD PTR [rsp+0xe00],zmm2
  653264:	vpunpckhqdq zmm1,zmm1,zmm7
  65326a:	vmovdqa64 ZMMWORD PTR [rsp+0xdc0],zmm1
  653272:	vpunpcklqdq zmm1,zmm3,zmm11
  653278:	vmovdqa64 ZMMWORD PTR [rsp+0xd80],zmm1
  653280:	vpunpckhqdq zmm1,zmm3,zmm11
  653286:	vmovdqa64 ZMMWORD PTR [rsp+0xd40],zmm1
  65328e:	vpunpcklqdq zmm1,zmm5,zmm9
  653294:	vmovdqa64 ZMMWORD PTR [rsp+0xd00],zmm1
  65329c:	vpunpckhqdq zmm1,zmm5,zmm9
  6532a2:	vmovdqa64 ZMMWORD PTR [rsp+0xcc0],zmm1
  6532aa:	vpunpcklqdq zmm1,zmm16,zmm13
  6532b0:	vmovdqa64 ZMMWORD PTR [rsp+0xc80],zmm1
  6532b8:	vpunpckhqdq zmm1,zmm16,zmm13
  6532be:	vmovdqa64 ZMMWORD PTR [rsp+0xc40],zmm1
  6532c6:	vpunpcklqdq zmm1,zmm0,zmm8
  6532cc:	vmovdqa64 ZMMWORD PTR [rsp+0xc00],zmm1
  6532d4:	vpunpckhqdq zmm0,zmm0,zmm8
  6532da:	vmovdqa64 ZMMWORD PTR [rsp+0x3c0],zmm0
  6532e2:	mov    edx,0x400
  6532e7:	lea    rdi,[rsp+0x400]
  6532ef:	xor    esi,esi
  6532f1:	vzeroupper 
  6532f4:	call   QWORD PTR [rip+0x9b32e]        # 6ee628 <memset@GLIBC_2.2.5>
  6532fa:	vmovdqa64 zmm16,ZMMWORD PTR [rsp+0xc00]
  653302:	vmovdqa64 zmm15,ZMMWORD PTR [rsp+0xc40]
  65330a:	vmovdqa64 zmm14,ZMMWORD PTR [rsp+0xc80]
  653312:	vmovdqa64 zmm13,ZMMWORD PTR [rsp+0xcc0]
  65331a:	vmovdqa64 zmm12,ZMMWORD PTR [rsp+0xd00]
  653322:	vmovdqa64 zmm11,ZMMWORD PTR [rsp+0xd40]
  65332a:	vmovdqa64 zmm10,ZMMWORD PTR [rsp+0xd80]
  653332:	vmovdqa64 zmm9,ZMMWORD PTR [rsp+0xdc0]
  65333a:	vmovdqa64 zmm8,ZMMWORD PTR [rsp+0xe00]
  653342:	vmovdqa64 zmm7,ZMMWORD PTR [rsp+0xe40]
  65334a:	vmovdqa64 zmm6,ZMMWORD PTR [rsp+0xe80]
  653352:	vmovdqa64 zmm5,ZMMWORD PTR [rsp+0xec0]
  65335a:	vmovdqa64 zmm4,ZMMWORD PTR [rsp+0xf00]
  653362:	vmovdqa64 zmm3,ZMMWORD PTR [rsp+0xf40]
  65336a:	vmovdqa64 zmm2,ZMMWORD PTR [rsp+0xf80]
  653372:	xor    eax,eax
  653374:	lea    rdx,[rip+0x9d1dd]        # 6f0558 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.17036336450247840325>
  65337b:	jmp    653398 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1c58>
  65337d:	nop    DWORD PTR [rax]
  653380:	vmovdqa64 ZMMWORD PTR [rsp+rax*1+0x400],zmm0
  653388:	add    rax,0x40
  65338c:	cmp    rax,0x400
  653392:	je     6537c0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2080>
  653398:	mov    rcx,QWORD PTR [rdx+rax*2+0x130]
  6533a0:	test   rcx,rcx
  6533a3:	je     6535b0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1e70>
  6533a9:	vpbroadcastq zmm0,rcx
  6533af:	vgf2p8affineqb zmm0,zmm2,zmm0,0x0
  6533b6:	mov    rcx,QWORD PTR [rdx+rax*2+0x138]
  6533be:	test   rcx,rcx
  6533c1:	je     6533d6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1c96>
  6533c3:	vpbroadcastq zmm1,rcx
  6533c9:	vgf2p8affineqb zmm1,zmm3,zmm1,0x0
  6533d0:	vpxord zmm0,zmm0,zmm1
  6533d6:	mov    rcx,QWORD PTR [rdx+rax*2+0x140]
  6533de:	test   rcx,rcx
  6533e1:	je     6533f6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1cb6>
  6533e3:	vpbroadcastq zmm1,rcx
  6533e9:	vgf2p8affineqb zmm1,zmm4,zmm1,0x0
  6533f0:	vpxord zmm0,zmm0,zmm1
  6533f6:	mov    rcx,QWORD PTR [rdx+rax*2+0x148]
  6533fe:	test   rcx,rcx
  653401:	je     653416 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1cd6>
  653403:	vpbroadcastq zmm1,rcx
  653409:	vgf2p8affineqb zmm1,zmm5,zmm1,0x0
  653410:	vpxord zmm0,zmm0,zmm1
  653416:	mov    rcx,QWORD PTR [rdx+rax*2+0x150]
  65341e:	test   rcx,rcx
  653421:	je     653436 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1cf6>
  653423:	vpbroadcastq zmm1,rcx
  653429:	vgf2p8affineqb zmm1,zmm6,zmm1,0x0
  653430:	vpxord zmm0,zmm0,zmm1
  653436:	mov    rcx,QWORD PTR [rdx+rax*2+0x158]
  65343e:	test   rcx,rcx
  653441:	je     653456 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1d16>
  653443:	vpbroadcastq zmm1,rcx
  653449:	vgf2p8affineqb zmm1,zmm7,zmm1,0x0
  653450:	vpxord zmm0,zmm0,zmm1
  653456:	mov    rcx,QWORD PTR [rdx+rax*2+0x160]
  65345e:	test   rcx,rcx
  653461:	je     653476 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1d36>
  653463:	vpbroadcastq zmm1,rcx
  653469:	vgf2p8affineqb zmm1,zmm8,zmm1,0x0
  653470:	vpxord zmm0,zmm0,zmm1
  653476:	mov    rcx,QWORD PTR [rdx+rax*2+0x168]
  65347e:	test   rcx,rcx
  653481:	je     653496 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1d56>
  653483:	vpbroadcastq zmm1,rcx
  653489:	vgf2p8affineqb zmm1,zmm9,zmm1,0x0
  653490:	vpxord zmm0,zmm0,zmm1
  653496:	mov    rcx,QWORD PTR [rdx+rax*2+0x170]
  65349e:	test   rcx,rcx
  6534a1:	je     6534b6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1d76>
  6534a3:	vpbroadcastq zmm1,rcx
  6534a9:	vgf2p8affineqb zmm1,zmm10,zmm1,0x0
  6534b0:	vpxord zmm0,zmm0,zmm1
  6534b6:	mov    rcx,QWORD PTR [rdx+rax*2+0x178]
  6534be:	test   rcx,rcx
  6534c1:	je     6534d6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1d96>
  6534c3:	vpbroadcastq zmm1,rcx
  6534c9:	vgf2p8affineqb zmm1,zmm11,zmm1,0x0
  6534d0:	vpxord zmm0,zmm0,zmm1
  6534d6:	mov    rcx,QWORD PTR [rdx+rax*2+0x180]
  6534de:	test   rcx,rcx
  6534e1:	je     6534f6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1db6>
  6534e3:	vpbroadcastq zmm1,rcx
  6534e9:	vgf2p8affineqb zmm1,zmm12,zmm1,0x0
  6534f0:	vpxord zmm0,zmm0,zmm1
  6534f6:	mov    rcx,QWORD PTR [rdx+rax*2+0x188]
  6534fe:	test   rcx,rcx
  653501:	je     653516 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1dd6>
  653503:	vpbroadcastq zmm1,rcx
  653509:	vgf2p8affineqb zmm1,zmm13,zmm1,0x0
  653510:	vpxord zmm0,zmm0,zmm1
  653516:	mov    rcx,QWORD PTR [rdx+rax*2+0x190]
  65351e:	test   rcx,rcx
  653521:	je     653536 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1df6>
  653523:	vpbroadcastq zmm1,rcx
  653529:	vgf2p8affineqb zmm1,zmm14,zmm1,0x0
  653530:	vpxord zmm0,zmm0,zmm1
  653536:	mov    rcx,QWORD PTR [rdx+rax*2+0x198]
  65353e:	test   rcx,rcx
  653541:	je     653556 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1e16>
  653543:	vpbroadcastq zmm1,rcx
  653549:	vgf2p8affineqb zmm1,zmm15,zmm1,0x0
  653550:	vpxord zmm0,zmm0,zmm1
  653556:	mov    rcx,QWORD PTR [rdx+rax*2+0x1a0]
  65355e:	test   rcx,rcx
  653561:	je     653576 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1e36>
  653563:	vpbroadcastq zmm1,rcx
  653569:	vgf2p8affineqb zmm1,zmm16,zmm1,0x0
  653570:	vpxord zmm0,zmm0,zmm1
  653576:	mov    rcx,QWORD PTR [rdx+rax*2+0x1a8]
  65357e:	test   rcx,rcx
  653581:	je     653380 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1c40>
  653587:	vpbroadcastq zmm1,rcx
  65358d:	vmovdqa64 zmm17,ZMMWORD PTR [rsp+0x3c0]
  653595:	vgf2p8affineqb zmm1,zmm17,zmm1,0x0
  65359c:	vpxord zmm0,zmm0,zmm1
  6535a2:	jmp    653380 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1c40>
  6535a7:	nop    WORD PTR [rax+rax*1+0x0]
  6535b0:	vpxor  xmm0,xmm0,xmm0
  6535b4:	mov    rcx,QWORD PTR [rdx+rax*2+0x138]
  6535bc:	test   rcx,rcx
  6535bf:	jne    6533c3 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1c83>
  6535c5:	jmp    6533d6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1c96>
  6535ca:	nop    WORD PTR [rax+rax*1+0x0]
  6535d0:	cmp    rsi,rcx
  6535d3:	je     653c5e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x251e>
  6535d9:	cmp    QWORD PTR [rsp+0x90],0x10
  6535e2:	jb     65516e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3a2e>
  6535e8:	shl    eax,0x4
  6535eb:	xor    ecx,ecx
  6535ed:	mov    rbx,QWORD PTR [rsp+0x1a8]
  6535f5:	mov    r15,QWORD PTR [rsp+0x1a0]
  6535fd:	nop    DWORD PTR [rax]
  653600:	mov    r11,QWORD PTR [rsp+0xc8]
  653608:	mov    rdx,QWORD PTR [r11+rcx*1-0x8]
  65360d:	mov    r8,rdx
  653610:	mov    rdi,rdx
  653613:	mov    rsi,rdx
  653616:	mov    r10,rdx
  653619:	mov    r9d,edx
  65361c:	shr    r9d,0x4
  653620:	and    r9d,0xff0
  653627:	vmovdqa xmm0,XMMWORD PTR [r15+r9*1]
  65362d:	movzx  r9d,dl
  653631:	shl    r9d,0x4
  653635:	vpxor  xmm0,xmm0,XMMWORD PTR [rbx+r9*1]
  65363b:	mov    r9d,edx
  65363e:	shr    r9d,0xc
  653642:	and    r9d,0xff0
  653649:	shr    edx,0x14
  65364c:	and    edx,0xfffffff0
  65364f:	mov    r12,QWORD PTR [rsp+0x240]
  653657:	vmovdqa xmm1,XMMWORD PTR [r12+rdx*1]
  65365d:	mov    rdx,QWORD PTR [rsp+0x280]
  653665:	vpternlogq xmm1,xmm0,XMMWORD PTR [rdx+r9*1],0x96
  65366d:	mov    r9,QWORD PTR [r11+rcx*1]
  653671:	mov    rdx,r9
  653674:	shr    rsi,0x28
  653678:	movzx  esi,sil
  65367c:	shl    esi,0x4
  65367f:	mov    r11,QWORD PTR [rsp+0x50]
  653684:	vmovdqa xmm0,XMMWORD PTR [r11+rsi*1]
  65368a:	mov    rsi,r9
  65368d:	shr    r10,0x20
  653691:	movzx  r10d,r10b
  653695:	shl    r10d,0x4
  653699:	mov    r11,QWORD PTR [rsp+0x58]
  65369e:	vpternlogq xmm0,xmm1,XMMWORD PTR [r11+r10*1],0x96
  6536a6:	mov    r10,r9
  6536a9:	shr    r8,0x38
  6536ad:	shl    r8d,0x4
  6536b1:	mov    r11,QWORD PTR [rsp+0x38]
  6536b6:	vmovdqa xmm1,XMMWORD PTR [r11+r8*1]
  6536bc:	mov    r8,r9
  6536bf:	shr    rdi,0x30
  6536c3:	movzx  edi,dil
  6536c7:	shl    edi,0x4
  6536ca:	mov    r11,QWORD PTR [rsp+0x110]
  6536d2:	vpternlogq xmm1,xmm0,XMMWORD PTR [r11+rdi*1],0x96
  6536da:	mov    edi,r9d
  6536dd:	shr    edi,0x4
  6536e0:	and    edi,0xff0
  6536e6:	mov    r11,QWORD PTR [rsp+0x100]
  6536ee:	vmovdqa xmm0,XMMWORD PTR [r11+rdi*1]
  6536f4:	movzx  edi,r9b
  6536f8:	shl    edi,0x4
  6536fb:	mov    r11,QWORD PTR [rsp+0x108]
  653703:	vpternlogq xmm0,xmm1,XMMWORD PTR [r11+rdi*1],0x96
  65370b:	mov    edi,r9d
  65370e:	shr    r9d,0x14
  653712:	and    r9d,0xfffffff0
  653716:	mov    r11,QWORD PTR [rsp+0xf0]
  65371e:	vmovdqa xmm1,XMMWORD PTR [r11+r9*1]
  653724:	shr    edi,0xc
  653727:	and    edi,0xff0
  65372d:	mov    r9,QWORD PTR [rsp+0xf8]
  653735:	vpternlogq xmm1,xmm0,XMMWORD PTR [r9+rdi*1],0x96
  65373d:	shr    r10,0x28
  653741:	movzx  edi,r10b
  653745:	shl    edi,0x4
  653748:	mov    r9,QWORD PTR [rsp+0xe0]
  653750:	vmovdqa xmm0,XMMWORD PTR [r9+rdi*1]
  653756:	shr    r8,0x20
  65375a:	movzx  edi,r8b
  65375e:	shl    edi,0x4
  653761:	mov    r8,QWORD PTR [rsp+0xe8]
  653769:	vpternlogq xmm0,xmm1,XMMWORD PTR [r8+rdi*1],0x96
  653771:	shr    rdx,0x38
  653775:	shl    edx,0x4
  653778:	mov    rdi,QWORD PTR [rsp+0xd0]
  653780:	vmovdqa xmm1,XMMWORD PTR [rdi+rdx*1]
  653785:	shr    rsi,0x30
  653789:	movzx  edx,sil
  65378d:	shl    edx,0x4
  653790:	mov    rsi,QWORD PTR [rsp+0xd8]
  653798:	vpternlogq xmm1,xmm0,XMMWORD PTR [rsi+rdx*1],0x96
  6537a0:	vmovdqu XMMWORD PTR [rsp+rcx*1+0x800],xmm1
  6537a9:	add    rcx,0x10
  6537ad:	cmp    rax,rcx
  6537b0:	jne    653600 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1ec0>
  6537b6:	jmp    653c5e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x251e>
  6537bb:	nop    DWORD PTR [rax+rax*1+0x0]
  6537c0:	vmovdqa64 zmm0,ZMMWORD PTR [rsp+0x400]
  6537c8:	vmovdqa64 zmm1,ZMMWORD PTR [rsp+0x440]
  6537d0:	vmovdqa64 zmm2,ZMMWORD PTR [rsp+0x480]
  6537d8:	vmovdqa64 zmm3,ZMMWORD PTR [rsp+0x4c0]
  6537e0:	vmovdqa64 zmm4,ZMMWORD PTR [rsp+0x500]
  6537e8:	vmovdqa64 zmm5,ZMMWORD PTR [rsp+0x540]
  6537f0:	vmovdqa64 zmm6,ZMMWORD PTR [rsp+0x580]
  6537f8:	vmovdqa64 zmm7,ZMMWORD PTR [rsp+0x5c0]
  653800:	vmovdqa64 zmm8,ZMMWORD PTR [rsp+0x600]
  653808:	vmovdqa64 zmm9,ZMMWORD PTR [rsp+0x640]
  653810:	vmovdqa64 zmm10,ZMMWORD PTR [rsp+0x680]
  653818:	vmovdqa64 zmm11,ZMMWORD PTR [rsp+0x6c0]
  653820:	vmovdqa64 zmm12,ZMMWORD PTR [rsp+0x700]
  653828:	vmovdqa64 zmm13,ZMMWORD PTR [rsp+0x740]
  653830:	vmovdqa64 zmm14,ZMMWORD PTR [rsp+0x780]
  653838:	vmovdqa64 zmm15,ZMMWORD PTR [rsp+0x7c0]
  653840:	vpunpcklbw zmm16,zmm0,zmm1
  653846:	vpunpckhbw zmm0,zmm0,zmm1
  65384c:	vpunpcklbw zmm1,zmm2,zmm3
  653852:	vpunpckhbw zmm2,zmm2,zmm3
  653858:	vpunpcklbw zmm3,zmm4,zmm5
  65385e:	vpunpckhbw zmm4,zmm4,zmm5
  653864:	vpunpcklbw zmm5,zmm6,zmm7
  65386a:	vpunpckhbw zmm6,zmm6,zmm7
  653870:	vpunpcklbw zmm7,zmm8,zmm9
  653876:	vpunpckhbw zmm8,zmm8,zmm9
  65387c:	vpunpcklbw zmm9,zmm10,zmm11
  653882:	vpunpckhbw zmm10,zmm10,zmm11
  653888:	vpunpcklbw zmm11,zmm12,zmm13
  65388e:	vpunpckhbw zmm12,zmm12,zmm13
  653894:	vpunpcklbw zmm13,zmm14,zmm15
  65389a:	vpunpckhbw zmm14,zmm14,zmm15
  6538a0:	vpunpcklwd zmm15,zmm16,zmm1
  6538a6:	vpunpckhwd zmm1,zmm16,zmm1
  6538ac:	vpunpcklwd zmm16,zmm0,zmm2
  6538b2:	vpunpckhwd zmm0,zmm0,zmm2
  6538b8:	vpunpcklwd zmm2,zmm3,zmm5
  6538be:	vpunpckhwd zmm3,zmm3,zmm5
  6538c4:	vpunpcklwd zmm5,zmm4,zmm6
  6538ca:	vpunpckhwd zmm4,zmm4,zmm6
  6538d0:	vpunpcklwd zmm6,zmm7,zmm9
  6538d6:	vpunpckhwd zmm7,zmm7,zmm9
  6538dc:	vpunpcklwd zmm9,zmm8,zmm10
  6538e2:	vpunpckhwd zmm8,zmm8,zmm10
  6538e8:	vpunpcklwd zmm10,zmm11,zmm13
  6538ee:	vpunpckhwd zmm11,zmm11,zmm13
  6538f4:	vpunpcklwd zmm13,zmm12,zmm14
  6538fa:	vpunpckhwd zmm12,zmm12,zmm14
  653900:	vpunpckldq zmm14,zmm15,zmm2
  653906:	vpunpckhdq zmm2,zmm15,zmm2
  65390c:	vpunpckldq zmm15,zmm1,zmm3
  653912:	vpunpckhdq zmm1,zmm1,zmm3
  653918:	vpunpckldq zmm3,zmm16,zmm5
  65391e:	vpunpckhdq zmm5,zmm16,zmm5
  653924:	vpunpckldq zmm16,zmm0,zmm4
  65392a:	vpunpckhdq zmm0,zmm0,zmm4
  653930:	vpunpckldq zmm4,zmm6,zmm10
  653936:	vpunpckhdq zmm6,zmm6,zmm10
  65393c:	vpunpckldq zmm10,zmm7,zmm11
  653942:	vpunpckhdq zmm7,zmm7,zmm11
  653948:	vpunpckldq zmm11,zmm9,zmm13
  65394e:	vpunpckhdq zmm9,zmm9,zmm13
  653954:	vpunpckldq zmm13,zmm8,zmm12
  65395a:	vpunpckhdq zmm8,zmm8,zmm12
  653960:	vpunpcklqdq zmm12,zmm14,zmm4
  653966:	vpunpckhqdq zmm4,zmm14,zmm4
  65396c:	vpunpcklqdq zmm14,zmm2,zmm6
  653972:	vpunpckhqdq zmm2,zmm2,zmm6
  653978:	vpunpcklqdq zmm6,zmm15,zmm10
  65397e:	vpunpckhqdq zmm10,zmm15,zmm10
  653984:	vpunpcklqdq zmm15,zmm1,zmm7
  65398a:	vpunpckhqdq zmm1,zmm1,zmm7
  653990:	vpunpcklqdq zmm7,zmm3,zmm11
  653996:	vpunpckhqdq zmm3,zmm3,zmm11
  65399c:	vpunpcklqdq zmm11,zmm5,zmm9
  6539a2:	vpunpckhqdq zmm5,zmm5,zmm9
  6539a8:	vpunpcklqdq zmm9,zmm16,zmm13
  6539ae:	vpunpckhqdq zmm13,zmm16,zmm13
  6539b4:	vpunpcklqdq zmm16,zmm0,zmm8
  6539ba:	vpunpckhqdq zmm0,zmm0,zmm8
  6539c0:	vmovdqu64 ZMMWORD PTR [rsp+0x800],zmm12
  6539c8:	vmovdqu64 ZMMWORD PTR [rsp+0x840],zmm4
  6539d0:	vmovdqu64 ZMMWORD PTR [rsp+0x880],zmm14
  6539d8:	vmovdqu64 ZMMWORD PTR [rsp+0x8c0],zmm2
  6539e0:	vmovdqu64 ZMMWORD PTR [rsp+0x900],zmm6
  6539e8:	vmovdqu64 ZMMWORD PTR [rsp+0x940],zmm10
  6539f0:	vmovdqu64 ZMMWORD PTR [rsp+0x980],zmm15
  6539f8:	vmovdqu64 ZMMWORD PTR [rsp+0x9c0],zmm1
  653a00:	vmovdqu64 ZMMWORD PTR [rsp+0xa00],zmm7
  653a08:	vmovdqu64 ZMMWORD PTR [rsp+0xa40],zmm3
  653a10:	vmovdqu64 ZMMWORD PTR [rsp+0xa80],zmm11
  653a18:	vmovdqu64 ZMMWORD PTR [rsp+0xac0],zmm5
  653a20:	vmovdqu64 ZMMWORD PTR [rsp+0xb00],zmm9
  653a28:	vmovdqu64 ZMMWORD PTR [rsp+0xb40],zmm13
  653a30:	vmovdqu64 ZMMWORD PTR [rsp+0xb80],zmm16
  653a38:	vmovdqu64 ZMMWORD PTR [rsp+0xbc0],zmm0
  653a40:	mov    r10,r15
  653a43:	test   bl,0x3f
  653a46:	je     653c5e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x251e>
  653a4c:	cmp    QWORD PTR [rsp+0x90],0x10
  653a55:	jb     65516e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3a2e>
  653a5b:	mov    eax,r10d
  653a5e:	and    eax,0x40
  653a61:	shl    eax,0x4
  653a64:	lea    rcx,[rsp+rax*1+0x800]
  653a6c:	mov    QWORD PTR [rsp+0x3c0],rcx
  653a74:	add    rax,QWORD PTR [rsp+0x1b8]
  653a7c:	and    r10d,0x3f
  653a80:	shl    r10d,0x4
  653a84:	xor    edx,edx
  653a86:	mov    rbx,QWORD PTR [rsp+0x1a8]
  653a8e:	mov    r15,QWORD PTR [rsp+0x1a0]
  653a96:	cs nop WORD PTR [rax+rax*1+0x0]
  653aa0:	mov    rsi,QWORD PTR [rax+rdx*1]
  653aa4:	mov    r9,rsi
  653aa7:	mov    r8,rsi
  653aaa:	mov    rdi,rsi
  653aad:	mov    r11,rsi
  653ab0:	mov    r12,r10
  653ab3:	mov    r10d,esi
  653ab6:	shr    r10d,0x4
  653aba:	and    r10d,0xff0
  653ac1:	vmovdqa xmm0,XMMWORD PTR [r15+r10*1]
  653ac7:	movzx  r10d,sil
  653acb:	shl    r10d,0x4
  653acf:	vpxor  xmm0,xmm0,XMMWORD PTR [rbx+r10*1]
  653ad5:	mov    r10d,esi
  653ad8:	shr    r10d,0xc
  653adc:	and    r10d,0xff0
  653ae3:	shr    esi,0x14
  653ae6:	and    esi,0xfffffff0
  653ae9:	mov    rcx,QWORD PTR [rsp+0x240]
  653af1:	vmovdqa xmm1,XMMWORD PTR [rcx+rsi*1]
  653af6:	mov    rcx,QWORD PTR [rsp+0x280]
  653afe:	vpternlogq xmm1,xmm0,XMMWORD PTR [rcx+r10*1],0x96
  653b06:	mov    r10,QWORD PTR [rax+rdx*1+0x8]
  653b0b:	mov    rsi,r10
  653b0e:	shr    rdi,0x28
  653b12:	movzx  edi,dil
  653b16:	shl    edi,0x4
  653b19:	mov    rcx,QWORD PTR [rsp+0x50]
  653b1e:	vmovdqa xmm0,XMMWORD PTR [rcx+rdi*1]
  653b23:	mov    rdi,r10
  653b26:	shr    r11,0x20
  653b2a:	movzx  r11d,r11b
  653b2e:	shl    r11d,0x4
  653b32:	mov    rcx,QWORD PTR [rsp+0x58]
  653b37:	vpternlogq xmm0,xmm1,XMMWORD PTR [rcx+r11*1],0x96
  653b3f:	mov    r11,r10
  653b42:	shr    r9,0x38
  653b46:	shl    r9d,0x4
  653b4a:	mov    rcx,QWORD PTR [rsp+0x38]
  653b4f:	vmovdqa xmm1,XMMWORD PTR [rcx+r9*1]
  653b55:	mov    r9,r10
  653b58:	shr    r8,0x30
  653b5c:	movzx  r8d,r8b
  653b60:	shl    r8d,0x4
  653b64:	mov    rcx,QWORD PTR [rsp+0x110]
  653b6c:	vpternlogq xmm1,xmm0,XMMWORD PTR [rcx+r8*1],0x96
  653b74:	mov    r8d,r10d
  653b77:	shr    r8d,0x4
  653b7b:	and    r8d,0xff0
  653b82:	mov    rcx,QWORD PTR [rsp+0x100]
  653b8a:	vmovdqa xmm0,XMMWORD PTR [rcx+r8*1]
  653b90:	movzx  r8d,r10b
  653b94:	shl    r8d,0x4
  653b98:	mov    rcx,QWORD PTR [rsp+0x108]
  653ba0:	vpternlogq xmm0,xmm1,XMMWORD PTR [rcx+r8*1],0x96
  653ba8:	mov    r8d,r10d
  653bab:	shr    r10d,0x14
  653baf:	and    r10d,0xfffffff0
  653bb3:	mov    rcx,QWORD PTR [rsp+0xf0]
  653bbb:	vmovdqa xmm1,XMMWORD PTR [rcx+r10*1]
  653bc1:	mov    r10,r12
  653bc4:	shr    r8d,0xc
  653bc8:	and    r8d,0xff0
  653bcf:	mov    rcx,QWORD PTR [rsp+0xf8]
  653bd7:	vpternlogq xmm1,xmm0,XMMWORD PTR [rcx+r8*1],0x96
  653bdf:	shr    r11,0x28
  653be3:	movzx  r8d,r11b
  653be7:	shl    r8d,0x4
  653beb:	mov    rcx,QWORD PTR [rsp+0xe0]
  653bf3:	vmovdqa xmm0,XMMWORD PTR [rcx+r8*1]
  653bf9:	shr    r9,0x20
  653bfd:	movzx  r8d,r9b
  653c01:	shl    r8d,0x4
  653c05:	mov    rcx,QWORD PTR [rsp+0xe8]
  653c0d:	vpternlogq xmm0,xmm1,XMMWORD PTR [rcx+r8*1],0x96
  653c15:	shr    rsi,0x38
  653c19:	shl    esi,0x4
  653c1c:	mov    rcx,QWORD PTR [rsp+0xd0]
  653c24:	vmovdqa xmm1,XMMWORD PTR [rcx+rsi*1]
  653c29:	shr    rdi,0x30
  653c2d:	movzx  esi,dil
  653c31:	shl    esi,0x4
  653c34:	mov    rcx,QWORD PTR [rsp+0xd8]
  653c3c:	vpternlogq xmm1,xmm0,XMMWORD PTR [rcx+rsi*1],0x96
  653c44:	mov    rsi,QWORD PTR [rsp+0x3c0]
  653c4c:	vmovdqu XMMWORD PTR [rsi+rdx*1],xmm1
  653c51:	add    rdx,0x10
  653c55:	cmp    r12,rdx
  653c58:	jne    653aa0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2360>
  653c5e:	mov    rax,QWORD PTR [rsp+0x148]
  653c66:	shr    rax,0x4
  653c6a:	mov    rcx,QWORD PTR [rsp+0x140]
  653c72:	shr    rcx,0x4
  653c76:	cmp    rcx,rax
  653c79:	cmovae rcx,rax
  653c7d:	test   rcx,rcx
  653c80:	je     652e20 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x16e0>
  653c86:	mov    rax,QWORD PTR [rsp+0xb8]
  653c8e:	mov    rcx,QWORD PTR [rsp+0x30]
  653c93:	add    rax,rcx
  653c96:	add    rcx,QWORD PTR [rsp+0xb0]
  653c9e:	mov    QWORD PTR [rsp+0x30],rcx
  653ca3:	mov    rcx,QWORD PTR [rsp+0x1c8]
  653cab:	mov    rdx,QWORD PTR [rsp+0x1c0]
  653cb3:	xor    esi,esi
  653cb5:	jmp    653cfc <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x25bc>
  653cb7:	nop    WORD PTR [rax+rax*1+0x0]
  653cc0:	shl    rsi,0x7
  653cc4:	vgf2p8mulb zmm1,zmm1,zmm3
  653cca:	vmovdqa64 ZMMWORD PTR [rsp+rsi*1+0x23c0],zmm1
  653cd5:	vgf2p8mulb zmm0,zmm0,zmm2
  653cdb:	vmovdqa64 ZMMWORD PTR [rsp+rsi*1+0x2400],zmm0
  653ce6:	add    rdx,0x10
  653cea:	add    rcx,0x10
  653cee:	mov    rsi,rdi
  653cf1:	cmp    rdi,QWORD PTR [rsp+0x48]
  653cf6:	je     652e20 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x16e0>
  653cfc:	lea    rdi,[rsi+0x1]
  653d00:	mov    r9,rsi
  653d03:	shl    r9,0x4
  653d07:	mov    r11,QWORD PTR [rsp+0x30]
  653d0c:	mov    r8,QWORD PTR [r11+r9*1]
  653d10:	vmovups xmm0,XMMWORD PTR [rax+r9*1]
  653d16:	mov    r10d,DWORD PTR [rax+r9*1]
  653d1a:	vandps xmm0,xmm0,XMMWORD PTR [r11+r9*1]
  653d20:	vmovups XMMWORD PTR [rsp+r9*1+0x1fc0],xmm0
  653d2a:	movzx  r9d,r10b
  653d2e:	shl    r9d,0x7
  653d32:	vmovdqu64 zmm1,ZMMWORD PTR [r14+r9*1]
  653d39:	vmovdqu64 zmm0,ZMMWORD PTR [r14+r9*1+0x40]
  653d41:	mov    r9d,0x1
  653d47:	mov    r10d,0x8
  653d4d:	jmp    653d95 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2655>
  653d4f:	vmovdqa64 zmm3,ZMMWORD PTR [rip+0xffffffffff9bd2e7]        # 11040 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x132>
  653d59:	vpermq zmm2,zmm3,zmm2
  653d5f:	xor    r15,0x40
  653d63:	vmovdqu64 zmm3,ZMMWORD PTR [r11+r15*1]
  653d6a:	vshufi64x2 zmm3,zmm3,zmm3,0x4e
  653d71:	vpxord zmm1,zmm2,zmm1
  653d77:	vpshufd zmm2,zmm3,0x4e
  653d7e:	vpxord zmm0,zmm2,zmm0
  653d84:	inc    r9
  653d87:	add    r10,0x8
  653d8b:	cmp    r9,0x10
  653d8f:	je     653eb0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2770>
  653d95:	movzx  r11d,BYTE PTR [rdx+r9*1]
  653d9a:	shl    r11d,0x7
  653d9e:	add    r11,r14
  653da1:	mov    r12d,r9d
  653da4:	shr    r12d,1
  653da7:	and    r12d,0x3
  653dab:	mov    r15,r10
  653dae:	and    r15,r13
  653db1:	vmovdqu64 zmm2,ZMMWORD PTR [r15+r11*1]
  653db8:	test   r9b,0x1
  653dbc:	jne    653de0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x26a0>
  653dbe:	lea    rbx,[rip+0xffffffffff9be69f]        # 12464 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1556>
  653dc5:	movsxd r12,DWORD PTR [rbx+r12*4]
  653dc9:	add    r12,rbx
  653dcc:	jmp    r12
  653dcf:	xor    r15,0x40
  653dd3:	vmovdqu64 zmm3,ZMMWORD PTR [r11+r15*1]
  653dda:	jmp    653e78 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2738>
  653ddf:	nop
  653de0:	lea    rbx,[rip+0xffffffffff9be66d]        # 12454 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1546>
  653de7:	movsxd r12,DWORD PTR [rbx+r12*4]
  653deb:	add    r12,rbx
  653dee:	jmp    r12
  653df1:	vpshufd zmm2,zmm2,0x4e
  653df8:	xor    r15,0x40
  653dfc:	vmovdqu64 zmm3,ZMMWORD PTR [r11+r15*1]
  653e03:	jmp    653d71 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2631>
  653e08:	vshufi64x2 zmm2,zmm2,zmm2,0x4e
  653e0f:	xor    r15,0x40
  653e13:	vmovdqu64 zmm3,ZMMWORD PTR [r11+r15*1]
  653e1a:	vshufi64x2 zmm3,zmm3,zmm3,0x4e
  653e21:	jmp    653e78 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2738>
  653e23:	vshufi64x2 zmm2,zmm2,zmm2,0x1b
  653e2a:	xor    r15,0x40
  653e2e:	vmovdqu64 zmm3,ZMMWORD PTR [r11+r15*1]
  653e35:	vshufi64x2 zmm3,zmm3,zmm3,0x1b
  653e3c:	jmp    653e78 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2738>
  653e3e:	vmovdqa64 zmm3,ZMMWORD PTR [rip+0xffffffffff9bd1b8]        # 11000 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xf2>
  653e48:	vpermq zmm2,zmm3,zmm2
  653e4e:	xor    r15,0x40
  653e52:	vmovdqu64 zmm3,ZMMWORD PTR [r11+r15*1]
  653e59:	vshufi64x2 zmm3,zmm3,zmm3,0x1b
  653e60:	jmp    653d71 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2631>
  653e65:	vpermq zmm2,zmm2,0x4e
  653e6c:	xor    r15,0x40
  653e70:	vpermq zmm3,ZMMWORD PTR [r11+r15*1],0x4e
  653e78:	vpxord zmm1,zmm1,zmm2
  653e7e:	vpxord zmm0,zmm0,zmm3
  653e84:	inc    r9
  653e87:	add    r10,0x8
  653e8b:	cmp    r9,0x10
  653e8f:	jne    653d95 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2655>
  653e95:	jmp    653eb0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2770>
  653e97:	vpermq zmm2,zmm2,0x1b
  653e9e:	xor    r15,0x40
  653ea2:	vpermq zmm3,ZMMWORD PTR [r11+r15*1],0x4e
  653eaa:	jmp    653d71 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2631>
  653eaf:	nop
  653eb0:	movzx  r8d,r8b
  653eb4:	shl    r8d,0x7
  653eb8:	vmovdqu64 zmm3,ZMMWORD PTR [r14+r8*1]
  653ebf:	vmovdqu64 zmm2,ZMMWORD PTR [r14+r8*1+0x40]
  653ec7:	mov    r8d,0x1
  653ecd:	mov    r9d,0x8
  653ed3:	jmp    653f1b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x27db>
  653ed5:	vmovdqa64 zmm5,ZMMWORD PTR [rip+0xffffffffff9bd161]        # 11040 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x132>
  653edf:	vpermq zmm4,zmm5,zmm4
  653ee5:	xor    r11,0x40
  653ee9:	vmovdqu64 zmm5,ZMMWORD PTR [r10+r11*1]
  653ef0:	vshufi64x2 zmm5,zmm5,zmm5,0x4e
  653ef7:	vpxord zmm3,zmm4,zmm3
  653efd:	vpshufd zmm4,zmm5,0x4e
  653f04:	vpxord zmm2,zmm4,zmm2
  653f0a:	inc    r8
  653f0d:	add    r9,0x8
  653f11:	cmp    r8,0x10
  653f15:	je     653cc0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2580>
  653f1b:	movzx  r10d,BYTE PTR [rcx+r8*1]
  653f20:	shl    r10d,0x7
  653f24:	add    r10,r14
  653f27:	mov    r15d,r8d
  653f2a:	shr    r15d,1
  653f2d:	and    r15d,0x3
  653f31:	mov    r11,r9
  653f34:	and    r11,r13
  653f37:	vmovdqu64 zmm4,ZMMWORD PTR [r11+r10*1]
  653f3e:	test   r8b,0x1
  653f42:	jne    653f70 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2830>
  653f44:	lea    rbx,[rip+0xffffffffff9be539]        # 12484 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1576>
  653f4b:	movsxd r15,DWORD PTR [rbx+r15*4]
  653f4f:	add    r15,rbx
  653f52:	jmp    r15
  653f55:	xor    r11,0x40
  653f59:	vmovdqu64 zmm5,ZMMWORD PTR [r10+r11*1]
  653f60:	jmp    654008 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x28c8>
  653f65:	data16 cs nop WORD PTR [rax+rax*1+0x0]
  653f70:	lea    rbx,[rip+0xffffffffff9be4fd]        # 12474 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1566>
  653f77:	movsxd r15,DWORD PTR [rbx+r15*4]
  653f7b:	add    r15,rbx
  653f7e:	jmp    r15
  653f81:	vpshufd zmm4,zmm4,0x4e
  653f88:	xor    r11,0x40
  653f8c:	vmovdqu64 zmm5,ZMMWORD PTR [r10+r11*1]
  653f93:	jmp    653ef7 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x27b7>
  653f98:	vshufi64x2 zmm4,zmm4,zmm4,0x4e
  653f9f:	xor    r11,0x40
  653fa3:	vmovdqu64 zmm5,ZMMWORD PTR [r10+r11*1]
  653faa:	vshufi64x2 zmm5,zmm5,zmm5,0x4e
  653fb1:	jmp    654008 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x28c8>
  653fb3:	vshufi64x2 zmm4,zmm4,zmm4,0x1b
  653fba:	xor    r11,0x40
  653fbe:	vmovdqu64 zmm5,ZMMWORD PTR [r10+r11*1]
  653fc5:	vshufi64x2 zmm5,zmm5,zmm5,0x1b
  653fcc:	jmp    654008 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x28c8>
  653fce:	vmovdqa64 zmm5,ZMMWORD PTR [rip+0xffffffffff9bd028]        # 11000 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xf2>
  653fd8:	vpermq zmm4,zmm5,zmm4
  653fde:	xor    r11,0x40
  653fe2:	vmovdqu64 zmm5,ZMMWORD PTR [r10+r11*1]
  653fe9:	vshufi64x2 zmm5,zmm5,zmm5,0x1b
  653ff0:	jmp    653ef7 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x27b7>
  653ff5:	vpermq zmm4,zmm4,0x4e
  653ffc:	xor    r11,0x40
  654000:	vpermq zmm5,ZMMWORD PTR [r10+r11*1],0x4e
  654008:	vpxord zmm3,zmm3,zmm4
  65400e:	vpxord zmm2,zmm2,zmm5
  654014:	inc    r8
  654017:	add    r9,0x8
  65401b:	cmp    r8,0x10
  65401f:	jne    653f1b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x27db>
  654025:	jmp    653cc0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2580>
  65402a:	vpermq zmm4,zmm4,0x1b
  654031:	xor    r11,0x40
  654035:	vpermq zmm5,ZMMWORD PTR [r10+r11*1],0x4e
  65403d:	jmp    653ef7 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x27b7>
  654042:	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  654050:	mov    edi,0xff0
  654055:	call   QWORD PTR [rip+0x9a59d]        # 6ee5f8 <malloc@GLIBC_2.2.5>
  65405b:	test   rax,rax
  65405e:	je     6551b6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3a76>
  654064:	mov    r14,rax
  654067:	mov    eax,0xf
  65406c:	lea    rcx,[rsp+0x1ec0]
  654074:	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  654080:	movzx  edx,BYTE PTR [rcx-0xf00]
  654087:	mov    BYTE PTR [rsp+0x48],dl
  65408b:	movzx  esi,BYTE PTR [rcx-0xe00]
  654092:	movzx  edi,BYTE PTR [rcx-0xd00]
  654099:	movzx  r8d,BYTE PTR [rcx-0xc00]
  6540a1:	movzx  r9d,BYTE PTR [rcx-0xb00]
  6540a9:	movzx  r10d,BYTE PTR [rcx-0xa00]
  6540b1:	movzx  r11d,BYTE PTR [rcx-0x900]
  6540b9:	movzx  ebx,BYTE PTR [rcx-0x800]
  6540c0:	movzx  r15d,BYTE PTR [rcx-0x700]
  6540c8:	movzx  r12d,BYTE PTR [rcx-0x600]
  6540d0:	movzx  r13d,BYTE PTR [rcx-0x500]
  6540d8:	movzx  edx,BYTE PTR [rcx-0x400]
  6540df:	mov    BYTE PTR [rsp+0x30],dl
  6540e3:	movzx  edx,BYTE PTR [rcx-0x300]
  6540ea:	mov    BYTE PTR [rsp+0x280],dl
  6540f1:	movzx  edx,BYTE PTR [rcx-0x200]
  6540f8:	mov    BYTE PTR [rsp+0x240],dl
  6540ff:	movzx  edx,BYTE PTR [rcx-0x100]
  654106:	mov    BYTE PTR [rsp+0x58],dl
  65410a:	movzx  edx,BYTE PTR [rcx]
  65410d:	mov    BYTE PTR [rsp+0x50],dl
  654111:	movzx  edx,BYTE PTR [rsp+0x48]
  654116:	mov    BYTE PTR [r14+rax*1-0xf],dl
  65411b:	mov    BYTE PTR [r14+rax*1-0xe],sil
  654120:	mov    BYTE PTR [r14+rax*1-0xd],dil
  654125:	mov    BYTE PTR [r14+rax*1-0xc],r8b
  65412a:	mov    BYTE PTR [r14+rax*1-0xb],r9b
  65412f:	mov    BYTE PTR [r14+rax*1-0xa],r10b
  654134:	mov    BYTE PTR [r14+rax*1-0x9],r11b
  654139:	mov    BYTE PTR [r14+rax*1-0x8],bl
  65413e:	mov    BYTE PTR [r14+rax*1-0x7],r15b
  654143:	mov    BYTE PTR [r14+rax*1-0x6],r12b
  654148:	mov    BYTE PTR [r14+rax*1-0x5],r13b
  65414d:	movzx  edx,BYTE PTR [rsp+0x30]
  654152:	mov    BYTE PTR [r14+rax*1-0x4],dl
  654157:	movzx  edx,BYTE PTR [rsp+0x280]
  65415f:	mov    BYTE PTR [r14+rax*1-0x3],dl
  654164:	movzx  edx,BYTE PTR [rsp+0x240]
  65416c:	mov    BYTE PTR [r14+rax*1-0x2],dl
  654171:	movzx  edx,BYTE PTR [rsp+0x58]
  654176:	mov    BYTE PTR [r14+rax*1-0x1],dl
  65417b:	movzx  edx,BYTE PTR [rsp+0x50]
  654180:	mov    BYTE PTR [r14+rax*1],dl
  654184:	add    rax,0x10
  654188:	inc    rcx
  65418b:	cmp    rax,0xfff
  654191:	jne    654080 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2940>
  654197:	jmp    651a40 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x300>
  65419c:	mov    eax,DWORD PTR [rip+0x9cce6]        # 6f0e88 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.17036336450247840325+0x930>
  6541a2:	test   eax,eax
  6541a4:	jne    654d48 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3608>
  6541aa:	mov    eax,DWORD PTR [rip+0x9de00]        # 6f1fb0 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.17036336450247840325+0x1120>
  6541b0:	test   eax,eax
  6541b2:	jne    654d52 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3612>
  6541b8:	mov    rax,QWORD PTR [rsp+0x40]
  6541bd:	shr    rax,0x4
  6541c1:	shr    r12,0x4
  6541c5:	cmp    r12,rax
  6541c8:	cmovae r12,rax
  6541cc:	cmp    r15,r12
  6541cf:	cmovb  r12,r15
  6541d3:	mov    edx,0x1000
  6541d8:	lea    rdi,[rsp+0x23c0]
  6541e0:	xor    esi,esi
  6541e2:	call   QWORD PTR [rip+0x9a440]        # 6ee628 <memset@GLIBC_2.2.5>
  6541e8:	test   r12,r12
  6541eb:	je     654bc6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3486>
  6541f1:	mov    rbx,QWORD PTR [rsp+0x128]
  6541f9:	mov    r14,QWORD PTR [rsp+0x120]
  654201:	xor    esi,esi
  654203:	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  654210:	mov    QWORD PTR [rsp+0x30],rsi
  654215:	mov    rcx,rsi
  654218:	shl    rcx,0x4
  65421c:	mov    rax,QWORD PTR [rsp+0xb8]
  654224:	mov    r15,rcx
  654227:	lea    rsi,[rax+rcx*1]
  65422b:	vpxor  xmm0,xmm0,xmm0
  65422f:	vmovdqa YMMWORD PTR [rsp+0x1020],ymm0
  654238:	vmovdqa YMMWORD PTR [rsp+0x1000],ymm0
  654241:	vmovdqa YMMWORD PTR [rsp+0xfe0],ymm0
  65424a:	vmovdqa YMMWORD PTR [rsp+0xfc0],ymm0
  654253:	mov    rax,QWORD PTR [rip+0x9dc6e]        # 6f1ec8 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.17036336450247840325+0x1038>
  65425a:	cmp    rax,0x80
  654260:	mov    QWORD PTR [rsp+0x48],rsi
  654265:	je     654500 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2dc0>
  65426b:	cmp    rax,0x40
  65426f:	je     655139 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x39f9>
  654275:	cmp    rax,0xf
  654279:	jbe    654530 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2df0>
  65427f:	mov    QWORD PTR [rsp+0x1fc0],0x10
  65428b:	cmp    QWORD PTR [rip+0x9dc3d],0x10        # 6f1ed0 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.17036336450247840325+0x1040>
  654293:	jne    65502c <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x38ec>
  654299:	mov    QWORD PTR [rsp+0x400],0x80
  6542a5:	cmp    rax,0x80
  6542ab:	jne    655044 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3904>
  6542b1:	mov    rax,QWORD PTR [rip+0x9dbe0]        # 6f1e98 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.17036336450247840325+0x1008>
  6542b8:	add    rax,QWORD PTR [rip+0x9dc19]        # 6f1ed8 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.17036336450247840325+0x1048>
  6542bf:	movzx  ecx,BYTE PTR [rsi]
  6542c2:	shl    ecx,0x7
  6542c5:	vmovups ymm0,YMMWORD PTR [rcx+rax*1]
  6542ca:	vmovups ymm1,YMMWORD PTR [rcx+rax*1+0x20]
  6542d0:	vmovups ymm2,YMMWORD PTR [rcx+rax*1+0x40]
  6542d6:	vmovups ymm3,YMMWORD PTR [rcx+rax*1+0x60]
  6542dc:	vmovaps YMMWORD PTR [rsp+0xfc0],ymm0
  6542e5:	vmovaps YMMWORD PTR [rsp+0xfe0],ymm1
  6542ee:	vmovaps YMMWORD PTR [rsp+0x1000],ymm2
  6542f7:	vmovaps YMMWORD PTR [rsp+0x1020],ymm3
  654300:	vmovdqa xmm0,XMMWORD PTR [rsp+0xfc0]
  654309:	vmovdqa xmm1,XMMWORD PTR [rsp+0xfd0]
  654312:	vmovdqa xmm2,XMMWORD PTR [rsp+0xfe0]
  65431b:	vmovdqa xmm3,XMMWORD PTR [rsp+0xff0]
  654324:	vmovdqa xmm4,XMMWORD PTR [rsp+0x1000]
  65432d:	vmovdqa xmm5,XMMWORD PTR [rsp+0x1010]
  654336:	vmovdqa xmm6,XMMWORD PTR [rsp+0x1020]
  65433f:	vmovdqa xmm7,XMMWORD PTR [rsp+0x1030]
  654348:	mov    ecx,0x1
  65434d:	jmp    65440b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2ccb>
  654352:	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  654360:	vpxor  xmm0,xmm0,XMMWORD PTR [rsi+rdx*1]
  654365:	vmovdqa XMMWORD PTR [rsp+0xfc0],xmm0
  65436e:	mov    rdi,rdx
  654371:	xor    rdi,0x10
  654375:	vpxor  xmm1,xmm1,XMMWORD PTR [rsi+rdi*1]
  65437a:	vmovdqa XMMWORD PTR [rsp+0xfd0],xmm1
  654383:	mov    rdi,rdx
  654386:	xor    rdi,0x20
  65438a:	vpxor  xmm2,xmm2,XMMWORD PTR [rsi+rdi*1]
  65438f:	vmovdqa XMMWORD PTR [rsp+0xfe0],xmm2
  654398:	mov    rdi,rdx
  65439b:	xor    rdi,0x30
  65439f:	vpxor  xmm3,xmm3,XMMWORD PTR [rsi+rdi*1]
  6543a4:	vmovdqa XMMWORD PTR [rsp+0xff0],xmm3
  6543ad:	mov    rdi,rdx
  6543b0:	xor    rdi,0x40
  6543b4:	vpxor  xmm4,xmm4,XMMWORD PTR [rsi+rdi*1]
  6543b9:	vmovdqa XMMWORD PTR [rsp+0x1000],xmm4
  6543c2:	mov    rdi,rdx
  6543c5:	xor    rdi,0x50
  6543c9:	vpxor  xmm5,xmm5,XMMWORD PTR [rsi+rdi*1]
  6543ce:	vmovdqa XMMWORD PTR [rsp+0x1010],xmm5
  6543d7:	mov    rdi,rdx
  6543da:	xor    rdi,0x60
  6543de:	vpxor  xmm6,xmm6,XMMWORD PTR [rsi+rdi*1]
  6543e3:	vmovdqa XMMWORD PTR [rsp+0x1020],xmm6
  6543ec:	xor    rdx,0x70
  6543f0:	vpxor  xmm7,xmm7,XMMWORD PTR [rsi+rdx*1]
  6543f5:	vmovdqa XMMWORD PTR [rsp+0x1030],xmm7
  6543fe:	inc    rcx
  654401:	cmp    rcx,0x10
  654405:	je     654553 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2e13>
  65440b:	mov    rdx,rcx
  65440e:	shr    rdx,1
  654411:	movzx  esi,BYTE PTR [r14+rcx*1]
  654416:	shl    esi,0x7
  654419:	add    rsi,rax
  65441c:	shl    rdx,0x4
  654420:	test   cl,0x1
  654423:	je     654360 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2c20>
  654429:	vpshufd xmm8,XMMWORD PTR [rsi+rdx*1],0x4e
  65442f:	vpxor  xmm0,xmm8,xmm0
  654433:	vmovdqa XMMWORD PTR [rsp+0xfc0],xmm0
  65443c:	mov    rdi,rdx
  65443f:	xor    rdi,0x10
  654443:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  654449:	vpxor  xmm1,xmm8,xmm1
  65444d:	vmovdqa XMMWORD PTR [rsp+0xfd0],xmm1
  654456:	mov    rdi,rdx
  654459:	xor    rdi,0x20
  65445d:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  654463:	vpxor  xmm2,xmm8,xmm2
  654467:	vmovdqa XMMWORD PTR [rsp+0xfe0],xmm2
  654470:	mov    rdi,rdx
  654473:	xor    rdi,0x30
  654477:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  65447d:	vpxor  xmm3,xmm8,xmm3
  654481:	vmovdqa XMMWORD PTR [rsp+0xff0],xmm3
  65448a:	mov    rdi,rdx
  65448d:	xor    rdi,0x40
  654491:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  654497:	vpxor  xmm4,xmm8,xmm4
  65449b:	vmovdqa XMMWORD PTR [rsp+0x1000],xmm4
  6544a4:	mov    rdi,rdx
  6544a7:	xor    rdi,0x50
  6544ab:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  6544b1:	vpxor  xmm5,xmm8,xmm5
  6544b5:	vmovdqa XMMWORD PTR [rsp+0x1010],xmm5
  6544be:	mov    rdi,rdx
  6544c1:	xor    rdi,0x60
  6544c5:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  6544cb:	vpxor  xmm6,xmm8,xmm6
  6544cf:	vmovdqa XMMWORD PTR [rsp+0x1020],xmm6
  6544d8:	xor    rdx,0x70
  6544dc:	vpshufd xmm8,XMMWORD PTR [rsi+rdx*1],0x4e
  6544e2:	vpxor  xmm7,xmm8,xmm7
  6544e6:	vmovdqa XMMWORD PTR [rsp+0x1030],xmm7
  6544ef:	inc    rcx
  6544f2:	cmp    rcx,0x10
  6544f6:	jne    65440b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2ccb>
  6544fc:	jmp    654553 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2e13>
  6544fe:	xchg   ax,ax
  654500:	mov    edx,0x10
  654505:	mov    r8d,0x80
  65450b:	lea    rdi,[rip+0x9d97e]        # 6f1e90 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.17036336450247840325+0x1000>
  654512:	lea    rcx,[rsp+0xfc0]
  65451a:	vzeroupper 
  65451d:	call   QWORD PTR [rip+0x9a53d]        # 6eea60 <_DYNAMIC+0x660>
  654523:	jmp    654553 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2e13>
  654525:	data16 cs nop WORD PTR [rax+rax*1+0x0]
  654530:	mov    edx,0x10
  654535:	mov    r8d,0x80
  65453b:	lea    rdi,[rip+0x9d94e]        # 6f1e90 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.17036336450247840325+0x1000>
  654542:	lea    rcx,[rsp+0xfc0]
  65454a:	vzeroupper 
  65454d:	call   QWORD PTR [rip+0x9a505]        # 6eea58 <_DYNAMIC+0x658>
  654553:	mov    rax,QWORD PTR [rsp+0xb0]
  65455b:	lea    rsi,[rax+r15*1]
  65455f:	vmovups zmm0,ZMMWORD PTR [rsp+0xfc0]
  654567:	vmovaps ZMMWORD PTR [rsp+0x280],zmm0
  65456f:	vmovdqu64 zmm0,ZMMWORD PTR [rsp+0x1000]
  654577:	vmovdqa64 ZMMWORD PTR [rsp+0x240],zmm0
  65457f:	vpxor  xmm0,xmm0,xmm0
  654583:	vmovdqa YMMWORD PTR [rsp+0x1020],ymm0
  65458c:	vmovdqa YMMWORD PTR [rsp+0x1000],ymm0
  654595:	vmovdqa YMMWORD PTR [rsp+0xfe0],ymm0
  65459e:	vmovdqa YMMWORD PTR [rsp+0xfc0],ymm0
  6545a7:	mov    rax,QWORD PTR [rip+0x9d91a]        # 6f1ec8 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.17036336450247840325+0x1038>
  6545ae:	cmp    rax,0x80
  6545b4:	je     654850 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3110>
  6545ba:	cmp    rax,0x40
  6545be:	je     655139 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x39f9>
  6545c4:	cmp    rax,0xf
  6545c8:	jbe    654880 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3140>
  6545ce:	mov    QWORD PTR [rsp+0x1fc0],0x10
  6545da:	cmp    QWORD PTR [rip+0x9d8ee],0x10        # 6f1ed0 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.17036336450247840325+0x1040>
  6545e2:	jne    65502c <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x38ec>
  6545e8:	mov    QWORD PTR [rsp+0x400],0x80
  6545f4:	cmp    rax,0x80
  6545fa:	jne    655044 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3904>
  654600:	mov    rax,QWORD PTR [rip+0x9d891]        # 6f1e98 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.17036336450247840325+0x1008>
  654607:	add    rax,QWORD PTR [rip+0x9d8ca]        # 6f1ed8 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.17036336450247840325+0x1048>
  65460e:	mov    r13,rsi
  654611:	movzx  ecx,BYTE PTR [rsi]
  654614:	shl    ecx,0x7
  654617:	vmovups ymm0,YMMWORD PTR [rcx+rax*1]
  65461c:	vmovups ymm1,YMMWORD PTR [rcx+rax*1+0x20]
  654622:	vmovups ymm2,YMMWORD PTR [rcx+rax*1+0x40]
  654628:	vmovups ymm3,YMMWORD PTR [rcx+rax*1+0x60]
  65462e:	vmovaps YMMWORD PTR [rsp+0xfc0],ymm0
  654637:	vmovaps YMMWORD PTR [rsp+0xfe0],ymm1
  654640:	vmovaps YMMWORD PTR [rsp+0x1000],ymm2
  654649:	vmovaps YMMWORD PTR [rsp+0x1020],ymm3
  654652:	vmovdqa xmm0,XMMWORD PTR [rsp+0xfc0]
  65465b:	vmovdqa xmm1,XMMWORD PTR [rsp+0xfd0]
  654664:	vmovdqa xmm2,XMMWORD PTR [rsp+0xfe0]
  65466d:	vmovdqa xmm3,XMMWORD PTR [rsp+0xff0]
  654676:	vmovdqa xmm4,XMMWORD PTR [rsp+0x1000]
  65467f:	vmovdqa xmm5,XMMWORD PTR [rsp+0x1010]
  654688:	vmovdqa xmm6,XMMWORD PTR [rsp+0x1020]
  654691:	vmovdqa xmm7,XMMWORD PTR [rsp+0x1030]
  65469a:	mov    ecx,0x1
  65469f:	jmp    65475b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x301b>
  6546a4:	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  6546b0:	vpxor  xmm0,xmm0,XMMWORD PTR [rsi+rdx*1]
  6546b5:	vmovdqa XMMWORD PTR [rsp+0xfc0],xmm0
  6546be:	mov    rdi,rdx
  6546c1:	xor    rdi,0x10
  6546c5:	vpxor  xmm1,xmm1,XMMWORD PTR [rsi+rdi*1]
  6546ca:	vmovdqa XMMWORD PTR [rsp+0xfd0],xmm1
  6546d3:	mov    rdi,rdx
  6546d6:	xor    rdi,0x20
  6546da:	vpxor  xmm2,xmm2,XMMWORD PTR [rsi+rdi*1]
  6546df:	vmovdqa XMMWORD PTR [rsp+0xfe0],xmm2
  6546e8:	mov    rdi,rdx
  6546eb:	xor    rdi,0x30
  6546ef:	vpxor  xmm3,xmm3,XMMWORD PTR [rsi+rdi*1]
  6546f4:	vmovdqa XMMWORD PTR [rsp+0xff0],xmm3
  6546fd:	mov    rdi,rdx
  654700:	xor    rdi,0x40
  654704:	vpxor  xmm4,xmm4,XMMWORD PTR [rsi+rdi*1]
  654709:	vmovdqa XMMWORD PTR [rsp+0x1000],xmm4
  654712:	mov    rdi,rdx
  654715:	xor    rdi,0x50
  654719:	vpxor  xmm5,xmm5,XMMWORD PTR [rsi+rdi*1]
  65471e:	vmovdqa XMMWORD PTR [rsp+0x1010],xmm5
  654727:	mov    rdi,rdx
  65472a:	xor    rdi,0x60
  65472e:	vpxor  xmm6,xmm6,XMMWORD PTR [rsi+rdi*1]
  654733:	vmovdqa XMMWORD PTR [rsp+0x1020],xmm6
  65473c:	xor    rdx,0x70
  654740:	vpxor  xmm7,xmm7,XMMWORD PTR [rsi+rdx*1]
  654745:	vmovdqa XMMWORD PTR [rsp+0x1030],xmm7
  65474e:	inc    rcx
  654751:	cmp    rcx,0x10
  654755:	je     6548a6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3166>
  65475b:	mov    rdx,rcx
  65475e:	shr    rdx,1
  654761:	movzx  esi,BYTE PTR [rbx+rcx*1]
  654765:	shl    esi,0x7
  654768:	add    rsi,rax
  65476b:	shl    rdx,0x4
  65476f:	test   cl,0x1
  654772:	je     6546b0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2f70>
  654778:	vpshufd xmm8,XMMWORD PTR [rsi+rdx*1],0x4e
  65477e:	vpxor  xmm0,xmm8,xmm0
  654782:	vmovdqa XMMWORD PTR [rsp+0xfc0],xmm0
  65478b:	mov    rdi,rdx
  65478e:	xor    rdi,0x10
  654792:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  654798:	vpxor  xmm1,xmm8,xmm1
  65479c:	vmovdqa XMMWORD PTR [rsp+0xfd0],xmm1
  6547a5:	mov    rdi,rdx
  6547a8:	xor    rdi,0x20
  6547ac:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  6547b2:	vpxor  xmm2,xmm8,xmm2
  6547b6:	vmovdqa XMMWORD PTR [rsp+0xfe0],xmm2
  6547bf:	mov    rdi,rdx
  6547c2:	xor    rdi,0x30
  6547c6:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  6547cc:	vpxor  xmm3,xmm8,xmm3
  6547d0:	vmovdqa XMMWORD PTR [rsp+0xff0],xmm3
  6547d9:	mov    rdi,rdx
  6547dc:	xor    rdi,0x40
  6547e0:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  6547e6:	vpxor  xmm4,xmm8,xmm4
  6547ea:	vmovdqa XMMWORD PTR [rsp+0x1000],xmm4
  6547f3:	mov    rdi,rdx
  6547f6:	xor    rdi,0x50
  6547fa:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  654800:	vpxor  xmm5,xmm8,xmm5
  654804:	vmovdqa XMMWORD PTR [rsp+0x1010],xmm5
  65480d:	mov    rdi,rdx
  654810:	xor    rdi,0x60
  654814:	vpshufd xmm8,XMMWORD PTR [rsi+rdi*1],0x4e
  65481a:	vpxor  xmm6,xmm8,xmm6
  65481e:	vmovdqa XMMWORD PTR [rsp+0x1020],xmm6
  654827:	xor    rdx,0x70
  65482b:	vpshufd xmm8,XMMWORD PTR [rsi+rdx*1],0x4e
  654831:	vpxor  xmm7,xmm8,xmm7
  654835:	vmovdqa XMMWORD PTR [rsp+0x1030],xmm7
  65483e:	inc    rcx
  654841:	cmp    rcx,0x10
  654845:	jne    65475b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x301b>
  65484b:	jmp    6548a6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3166>
  65484d:	nop    DWORD PTR [rax]
  654850:	mov    edx,0x10
  654855:	mov    r8d,0x80
  65485b:	lea    rdi,[rip+0x9d62e]        # 6f1e90 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.17036336450247840325+0x1000>
  654862:	mov    r13,rsi
  654865:	lea    rcx,[rsp+0xfc0]
  65486d:	vzeroupper 
  654870:	call   QWORD PTR [rip+0x9a1ea]        # 6eea60 <_DYNAMIC+0x660>
  654876:	jmp    6548a6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3166>
  654878:	nop    DWORD PTR [rax+rax*1+0x0]
  654880:	mov    edx,0x10
  654885:	mov    r8d,0x80
  65488b:	lea    rdi,[rip+0x9d5fe]        # 6f1e90 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.17036336450247840325+0x1000>
  654892:	mov    r13,rsi
  654895:	lea    rcx,[rsp+0xfc0]
  65489d:	vzeroupper 
  6548a0:	call   QWORD PTR [rip+0x9a1b2]        # 6eea58 <_DYNAMIC+0x658>
  6548a6:	mov    rdi,QWORD PTR [rip+0x9bcc3]        # 6f0570 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.17036336450247840325+0x18>
  6548ad:	cmp    rdi,0x10
  6548b1:	jb     655176 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3a36>
  6548b7:	add    r15,QWORD PTR [rsp+0xa8]
  6548bf:	mov    rcx,QWORD PTR [r15]
  6548c2:	mov    rax,QWORD PTR [rip+0x9bc9f]        # 6f0568 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.17036336450247840325+0x10>
  6548c9:	mov    rdi,rcx
  6548cc:	mov    rsi,rcx
  6548cf:	mov    rdx,rcx
  6548d2:	mov    r9,rcx
  6548d5:	mov    r8d,ecx
  6548d8:	shr    r8d,0x4
  6548dc:	and    r8d,0xff0
  6548e3:	vmovdqa xmm0,XMMWORD PTR [rax+r8*1+0x1000]
  6548ed:	movzx  r8d,cl
  6548f1:	shl    r8d,0x4
  6548f5:	vpxor  xmm0,xmm0,XMMWORD PTR [rax+r8*1]
  6548fb:	mov    r8d,ecx
  6548fe:	shr    ecx,0x14
  654901:	and    ecx,0xfffffff0
  654904:	vmovdqa xmm1,XMMWORD PTR [rax+rcx*1+0x3000]
  65490d:	shr    r8d,0xc
  654911:	and    r8d,0xff0
  654918:	vpternlogq xmm1,xmm0,XMMWORD PTR [rax+r8*1+0x2000],0x96
  654924:	mov    r8,QWORD PTR [r15+0x8]
  654928:	mov    rcx,r8
  65492b:	shr    rdx,0x28
  65492f:	movzx  edx,dl
  654932:	shl    edx,0x4
  654935:	vmovdqa xmm0,XMMWORD PTR [rax+rdx*1+0x5000]
  65493e:	mov    rdx,r8
  654941:	shr    r9,0x20
  654945:	movzx  r9d,r9b
  654949:	shl    r9d,0x4
  65494d:	vpternlogq xmm0,xmm1,XMMWORD PTR [rax+r9*1+0x4000],0x96
  654959:	mov    r9,r8
  65495c:	shr    rdi,0x38
  654960:	shl    edi,0x4
  654963:	vmovdqa xmm1,XMMWORD PTR [rax+rdi*1+0x7000]
  65496c:	mov    rdi,r8
  65496f:	shr    rsi,0x30
  654973:	movzx  esi,sil
  654977:	shl    esi,0x4
  65497a:	vpternlogq xmm1,xmm0,XMMWORD PTR [rax+rsi*1+0x6000],0x96
  654986:	mov    esi,r8d
  654989:	shr    esi,0x4
  65498c:	and    esi,0xff0
  654992:	vmovdqa xmm0,XMMWORD PTR [rax+rsi*1+0x9000]
  65499b:	movzx  esi,r8b
  65499f:	shl    esi,0x4
  6549a2:	vpternlogq xmm0,xmm1,XMMWORD PTR [rax+rsi*1+0x8000],0x96
  6549ae:	mov    esi,r8d
  6549b1:	shr    r8d,0x14
  6549b5:	and    r8d,0xfffffff0
  6549b9:	vmovdqa xmm1,XMMWORD PTR [rax+r8*1+0xb000]
  6549c3:	shr    esi,0xc
  6549c6:	and    esi,0xff0
  6549cc:	vpternlogq xmm1,xmm0,XMMWORD PTR [rax+rsi*1+0xa000],0x96
  6549d8:	shr    r9,0x28
  6549dc:	movzx  esi,r9b
  6549e0:	shl    esi,0x4
  6549e3:	vmovdqa xmm0,XMMWORD PTR [rax+rsi*1+0xd000]
  6549ec:	shr    rdi,0x20
  6549f0:	movzx  esi,dil
  6549f4:	shl    esi,0x4
  6549f7:	vpternlogq xmm0,xmm1,XMMWORD PTR [rax+rsi*1+0xc000],0x96
  654a03:	shr    rcx,0x38
  654a07:	shl    ecx,0x4
  654a0a:	vmovdqa xmm1,XMMWORD PTR [rax+rcx*1+0xf000]
  654a13:	shr    rdx,0x30
  654a17:	movzx  ecx,dl
  654a1a:	shl    ecx,0x4
  654a1d:	vpternlogq xmm1,xmm0,XMMWORD PTR [rax+rcx*1+0xe000],0x96
  654a29:	vmovdqu64 zmm0,ZMMWORD PTR [rsp+0xfc0]
  654a31:	vmovdqa64 zmm3,ZMMWORD PTR [rip+0xffffffffff9bc645]        # 11080 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x172>
  654a3b:	vpandq zmm2,zmm3,ZMMWORD PTR [rsp+0x1000]
  654a43:	vmovdqu XMMWORD PTR [rsp+0xfe0],xmm1
  654a4c:	mov    rax,QWORD PTR [r13+0x0]
  654a50:	mov    rcx,QWORD PTR [r13+0x8]
  654a54:	mov    rdx,QWORD PTR [rsp+0x48]
  654a59:	and    rax,QWORD PTR [rdx]
  654a5c:	and    rcx,QWORD PTR [rdx+0x8]
  654a60:	mov    rsi,QWORD PTR [rsp+0x30]
  654a65:	inc    rsi
  654a68:	vgf2p8mulb zmm0,zmm0,ZMMWORD PTR [rsp+0x280]
  654a70:	vpandq zmm1,zmm3,ZMMWORD PTR [rsp+0x240]
  654a78:	vgf2p8mulb zmm1,zmm1,zmm2
  654a7e:	lea    rdx,[rsp+0xff0]
  654a86:	vpxor  xmm2,xmm2,xmm2
  654a8a:	vmovdqu XMMWORD PTR [rdx],xmm2
  654a8e:	kmovq  k1,rax
  654a93:	kmovq  k2,rcx
  654a98:	lea    rax,[rsp+0xfe1]
  654aa0:	xor    ecx,ecx
  654aa2:	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  654ab0:	vpbroadcastb zmm2,BYTE PTR [rax-0x1]
  654ab7:	vmovdqu8 zmm3{k1}{z},zmm2
  654abd:	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x23c0]
  654ac8:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x23c0],zmm3
  654ad3:	vmovdqu8 zmm3{k2}{z},zmm2
  654ad9:	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x2400]
  654ae4:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x2400],zmm3
  654aef:	vgf2p8mulb zmm3,zmm0,zmm2
  654af5:	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x2440]
  654b00:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x2440],zmm3
  654b0b:	vgf2p8mulb zmm2,zmm1,zmm2
  654b11:	vpxord zmm2,zmm2,ZMMWORD PTR [rsp+rcx*1+0x2480]
  654b1c:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x2480],zmm2
  654b27:	vpbroadcastb zmm2,BYTE PTR [rax]
  654b2d:	vmovdqu8 zmm3{k1}{z},zmm2
  654b33:	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x24c0]
  654b3e:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x24c0],zmm3
  654b49:	vmovdqu8 zmm3{k2}{z},zmm2
  654b4f:	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x2500]
  654b5a:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x2500],zmm3
  654b65:	vgf2p8mulb zmm3,zmm0,zmm2
  654b6b:	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x2540]
  654b76:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x2540],zmm3
  654b81:	vgf2p8mulb zmm2,zmm1,zmm2
  654b87:	vpxord zmm2,zmm2,ZMMWORD PTR [rsp+rcx*1+0x2580]
  654b92:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x2580],zmm2
  654b9d:	add    rcx,0x200
  654ba4:	add    rax,0x2
  654ba8:	cmp    rcx,0x1000
  654baf:	jne    654ab0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3370>
  654bb5:	add    r14,0x10
  654bb9:	add    rbx,0x10
  654bbd:	cmp    rsi,r12
  654bc0:	jne    654210 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2ad0>
  654bc6:	mov    edi,0xff0
  654bcb:	vzeroupper 
  654bce:	call   QWORD PTR [rip+0x99a24]        # 6ee5f8 <malloc@GLIBC_2.2.5>
  654bd4:	test   rax,rax
  654bd7:	je     6551b6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3a76>
  654bdd:	mov    r14,rax
  654be0:	mov    eax,0xf
  654be5:	lea    rcx,[rsp+0x32c0]
  654bed:	nop    DWORD PTR [rax]
  654bf0:	movzx  edx,BYTE PTR [rcx-0xf00]
  654bf7:	mov    BYTE PTR [rsp+0x48],dl
  654bfb:	movzx  esi,BYTE PTR [rcx-0xe00]
  654c02:	movzx  edi,BYTE PTR [rcx-0xd00]
  654c09:	movzx  r8d,BYTE PTR [rcx-0xc00]
  654c11:	movzx  r9d,BYTE PTR [rcx-0xb00]
  654c19:	movzx  r10d,BYTE PTR [rcx-0xa00]
  654c21:	movzx  r11d,BYTE PTR [rcx-0x900]
  654c29:	movzx  ebx,BYTE PTR [rcx-0x800]
  654c30:	movzx  r15d,BYTE PTR [rcx-0x700]
  654c38:	movzx  r12d,BYTE PTR [rcx-0x600]
  654c40:	movzx  r13d,BYTE PTR [rcx-0x500]
  654c48:	movzx  edx,BYTE PTR [rcx-0x400]
  654c4f:	mov    BYTE PTR [rsp+0x30],dl
  654c53:	movzx  edx,BYTE PTR [rcx-0x300]
  654c5a:	mov    BYTE PTR [rsp+0x280],dl
  654c61:	movzx  edx,BYTE PTR [rcx-0x200]
  654c68:	mov    BYTE PTR [rsp+0x240],dl
  654c6f:	movzx  edx,BYTE PTR [rcx-0x100]
  654c76:	mov    BYTE PTR [rsp+0x58],dl
  654c7a:	movzx  edx,BYTE PTR [rcx]
  654c7d:	mov    BYTE PTR [rsp+0x50],dl
  654c81:	movzx  edx,BYTE PTR [rsp+0x48]
  654c86:	mov    BYTE PTR [r14+rax*1-0xf],dl
  654c8b:	mov    BYTE PTR [r14+rax*1-0xe],sil
  654c90:	mov    BYTE PTR [r14+rax*1-0xd],dil
  654c95:	mov    BYTE PTR [r14+rax*1-0xc],r8b
  654c9a:	mov    BYTE PTR [r14+rax*1-0xb],r9b
  654c9f:	mov    BYTE PTR [r14+rax*1-0xa],r10b
  654ca4:	mov    BYTE PTR [r14+rax*1-0x9],r11b
  654ca9:	mov    BYTE PTR [r14+rax*1-0x8],bl
  654cae:	mov    BYTE PTR [r14+rax*1-0x7],r15b
  654cb3:	mov    BYTE PTR [r14+rax*1-0x6],r12b
  654cb8:	mov    BYTE PTR [r14+rax*1-0x5],r13b
  654cbd:	movzx  edx,BYTE PTR [rsp+0x30]
  654cc2:	mov    BYTE PTR [r14+rax*1-0x4],dl
  654cc7:	movzx  edx,BYTE PTR [rsp+0x280]
  654ccf:	mov    BYTE PTR [r14+rax*1-0x3],dl
  654cd4:	movzx  edx,BYTE PTR [rsp+0x240]
  654cdc:	mov    BYTE PTR [r14+rax*1-0x2],dl
  654ce1:	movzx  edx,BYTE PTR [rsp+0x58]
  654ce6:	mov    BYTE PTR [r14+rax*1-0x1],dl
  654ceb:	movzx  edx,BYTE PTR [rsp+0x50]
  654cf0:	mov    BYTE PTR [r14+rax*1],dl
  654cf4:	add    rax,0x10
  654cf8:	inc    rcx
  654cfb:	cmp    rax,0xfff
  654d01:	jne    654bf0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x34b0>
  654d07:	jmp    651a40 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x300>
  654d0c:	call   64ed34 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.17036336450247840325>
  654d11:	jmp    651bec <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x4ac>
  654d16:	call   64ed34 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.17036336450247840325>
  654d1b:	jmp    6521ce <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0xa8e>
  654d20:	call   64ed86 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate6domain6DomainE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.17036336450247840325>
  654d25:	jmp    651bfa <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x4ba>
  654d2a:	call   64ed86 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate6domain6DomainE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.17036336450247840325>
  654d2f:	jmp    6521dc <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0xa9c>
  654d34:	call   64ed86 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate6domain6DomainE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.17036336450247840325>
  654d39:	jmp    652be1 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x14a1>
  654d3e:	call   64ed34 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.17036336450247840325>
  654d43:	jmp    652bef <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x14af>
  654d48:	call   64ed34 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.17036336450247840325>
  654d4d:	jmp    6541aa <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2a6a>
  654d52:	call   64ed86 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate6domain6DomainE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.17036336450247840325>
  654d57:	jmp    6541b8 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2a78>
  654d5c:	mov    eax,0xff
  654d61:	mov    QWORD PTR [rsp+0x60],rax
  654d66:	mov    rcx,QWORD PTR [rsp+0x118]
  654d6e:	mov    rax,QWORD PTR [rsp+0x60]
  654d73:	mov    QWORD PTR [rcx],rax
  654d76:	mov    rax,QWORD PTR [rsp+0x68]
  654d7b:	mov    QWORD PTR [rcx+0x8],rax
  654d7f:	mov    QWORD PTR [rcx+0x10],r10
  654d83:	jmp    655003 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x38c3>
  654d88:	mov    r14,rsi
  654d8b:	mov    r12,r9
  654d8e:	mov    r15,rcx
  654d91:	call   QWORD PTR [rip+0x99c31]        # 6ee9c8 <_DYNAMIC+0x5c8>
  654d97:	mov    rcx,r15
  654d9a:	mov    r9,r12
  654d9d:	mov    rsi,r14
  654da0:	mov    rax,QWORD PTR [rax]
  654da3:	mov    rax,QWORD PTR [rax+0x208]
  654daa:	shr    rcx,1
  654dad:	cmp    rcx,rax
  654db0:	cmovbe rcx,rax
  654db4:	mov    QWORD PTR [rsp+0x208],rcx
  654dbc:	mov    QWORD PTR [rsp+0x400],rsi
  654dc4:	mov    rax,QWORD PTR [r9]
  654dc7:	mov    r11,QWORD PTR [r9+0x8]
  654dcb:	mov    rcx,QWORD PTR [r9+0x10]
  654dcf:	mov    rdx,QWORD PTR [r9+0x18]
  654dd3:	mov    r10,QWORD PTR [r9+0x20]
  654dd7:	mov    rdi,QWORD PTR [r9+0x28]
  654ddb:	mov    r8,rcx
  654dde:	imul   r8,rsi
  654de2:	cmp    r11,r8
  654de5:	cmovb  r8,r11
  654de9:	lea    r14,[rax+r8*1]
  654ded:	sub    r11,r8
  654df0:	lea    r15,[rsp+0x200]
  654df8:	mov    QWORD PTR [rsp+0x2c8],r15
  654e00:	lea    r15,[rsp+0x400]
  654e08:	mov    QWORD PTR [rsp+0x2d0],r15
  654e10:	lea    r12,[rsp+0x208]
  654e18:	mov    QWORD PTR [rsp+0x2d8],r12
  654e20:	mov    QWORD PTR [rsp+0x2e0],r14
  654e28:	mov    r14,rdi
  654e2b:	imul   r14,rsi
  654e2f:	cmp    r10,r14
  654e32:	cmovb  r14,r10
  654e36:	mov    QWORD PTR [rsp+0x2e8],r11
  654e3e:	lea    r11,[rdx+r14*1]
  654e42:	mov    QWORD PTR [rsp+0x2f0],rcx
  654e4a:	mov    QWORD PTR [rsp+0x2f8],r11
  654e52:	mov    r11,QWORD PTR [r9+0x30]
  654e56:	sub    r10,r14
  654e59:	mov    QWORD PTR [rsp+0x300],r10
  654e61:	mov    r10,QWORD PTR [r9+0x38]
  654e65:	mov    r9,QWORD PTR [r9+0x40]
  654e69:	imul   rsi,r9
  654e6d:	cmp    r10,rsi
  654e70:	cmovb  rsi,r10
  654e74:	mov    r13,rsi
  654e77:	shl    r13,0x4
  654e7b:	add    r13,r11
  654e7e:	mov    QWORD PTR [rsp+0x308],rdi
  654e86:	mov    QWORD PTR [rsp+0x310],r13
  654e8e:	sub    r10,rsi
  654e91:	mov    QWORD PTR [rsp+0x318],r10
  654e99:	mov    r10,QWORD PTR [rbx]
  654e9c:	mov    r13,QWORD PTR [rbx+0x8]
  654ea0:	mov    rbx,QWORD PTR [rbx+0x10]
  654ea4:	mov    QWORD PTR [rsp+0x320],r9
  654eac:	mov    QWORD PTR [rsp+0x328],r10
  654eb4:	mov    QWORD PTR [rsp+0x330],r13
  654ebc:	mov    QWORD PTR [rsp+0x338],rbx
  654ec4:	mov    QWORD PTR [rsp+0x340],r15
  654ecc:	mov    QWORD PTR [rsp+0x348],r12
  654ed4:	mov    QWORD PTR [rsp+0x350],rax
  654edc:	mov    QWORD PTR [rsp+0x358],r8
  654ee4:	mov    QWORD PTR [rsp+0x360],rcx
  654eec:	mov    QWORD PTR [rsp+0x368],rdx
  654ef4:	mov    QWORD PTR [rsp+0x370],r14
  654efc:	mov    QWORD PTR [rsp+0x378],rdi
  654f04:	mov    QWORD PTR [rsp+0x380],r11
  654f0c:	mov    QWORD PTR [rsp+0x388],rsi
  654f14:	mov    QWORD PTR [rsp+0x390],r9
  654f1c:	mov    QWORD PTR [rsp+0x398],r10
  654f24:	mov    QWORD PTR [rsp+0x3a0],r13
  654f2c:	mov    QWORD PTR [rsp+0x3a8],rbx
  654f34:	mov    rbx,0xffffffffffffffa8
  654f3b:	mov    rdx,QWORD PTR fs:[rbx]
  654f3f:	test   rdx,rdx
  654f42:	je     654f65 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3825>
  654f44:	lea    rdi,[rsp+0xfc0]
  654f4c:	lea    rsi,[rsp+0x2c8]
  654f54:	xor    ecx,ecx
  654f56:	call   65b5c0 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_>
  654f5b:	mov    rdi,QWORD PTR [rsp+0x118]
  654f63:	jmp    654faa <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x386a>
  654f65:	call   QWORD PTR [rip+0x99a5d]        # 6ee9c8 <_DYNAMIC+0x5c8>
  654f6b:	mov    rsi,QWORD PTR [rax]
  654f6e:	mov    rdx,QWORD PTR fs:[rbx]
  654f72:	test   rdx,rdx
  654f75:	mov    rbx,QWORD PTR [rsp+0x118]
  654f7d:	je     6550e4 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x39a4>
  654f83:	cmp    QWORD PTR [rdx+0x110],rsi
  654f8a:	jne    655102 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x39c2>
  654f90:	lea    rdi,[rsp+0xfc0]
  654f98:	lea    rsi,[rsp+0x2c8]
  654fa0:	xor    ecx,ecx
  654fa2:	call   65b5c0 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_>
  654fa7:	mov    rdi,rbx
  654faa:	mov    rax,QWORD PTR [rsp+0xfd0]
  654fb2:	mov    QWORD PTR [rsp+0x23d0],rax
  654fba:	vmovups xmm0,XMMWORD PTR [rsp+0xfc0]
  654fc3:	vmovaps XMMWORD PTR [rsp+0x23c0],xmm0
  654fcc:	lea    rdx,[rsp+0x23d8]
  654fd4:	vmovdqu xmm0,XMMWORD PTR [rsp+0xfd8]
  654fdd:	vmovdqu XMMWORD PTR [rsp+0x23d8],xmm0
  654fe6:	mov    rax,QWORD PTR [rsp+0xfe8]
  654fee:	mov    QWORD PTR [rsp+0x23e8],rax
  654ff6:	lea    rsi,[rsp+0x23c0]
  654ffe:	call   65d880 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0B5_>
  655003:	lea    rsp,[rbp-0x28]
  655007:	pop    rbx
  655008:	pop    r12
  65500a:	pop    r13
  65500c:	pop    r14
  65500e:	pop    r15
  655010:	pop    rbp
  655011:	ret    
  655012:	lea    rdx,[rip+0x962ff]        # 6eb318 <anon.e5d9da81ce3e97b5bac39b4a40b5610e.12.llvm.17036336450247840325+0xd8>
  655019:	mov    rsi,QWORD PTR [rsp+0x30]
  65501e:	vzeroupper 
  655021:	call   QWORD PTR [rip+0x99629]        # 6ee650 <_DYNAMIC+0x250>
  655027:	jmp    6551d8 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3a98>
  65502c:	lea    rdx,[rip+0x9ce9d]        # 6f1ed0 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.17036336450247840325+0x1040>
  655033:	lea    rsi,[rsp+0x1fc0]
  65503b:	lea    r9,[rip+0x95116]        # 6ea158 <anon.01aa214378fce5a476fd64886db4a5c0.1330.llvm.5168012589948291949>
  655042:	jmp    65505a <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x391a>
  655044:	lea    rsi,[rsp+0x400]
  65504c:	lea    r9,[rip+0x9511d]        # 6ea170 <anon.01aa214378fce5a476fd64886db4a5c0.1331.llvm.5168012589948291949>
  655053:	lea    rdx,[rip+0x9ce6e]        # 6f1ec8 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.17036336450247840325+0x1038>
  65505a:	xor    edi,edi
  65505c:	xor    ecx,ecx
  65505e:	vzeroupper 
  655061:	call   QWORD PTR [rip+0x99761]        # 6ee7c8 <_DYNAMIC+0x3c8>
  655067:	jmp    6551d8 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3a98>
  65506c:	mov    rsi,rdx
  65506f:	jmp    655097 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3957>
  655071:	mov    rsi,r8
  655074:	jmp    655097 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3957>
  655076:	mov    rsi,r10
  655079:	jmp    655097 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3957>
  65507b:	mov    rsi,r12
  65507e:	jmp    655092 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3952>
  655080:	mov    rsi,r11
  655083:	jmp    655097 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3957>
  655085:	mov    rsi,r9
  655088:	jmp    655092 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3952>
  65508a:	mov    rsi,rbx
  65508d:	jmp    655092 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3952>
  65508f:	mov    rsi,rax
  655092:	mov    rbx,QWORD PTR [rsp+0x38]
  655097:	mov    rdi,rsi
  65509a:	mov    rsi,QWORD PTR [rsp+0x30]
  65509f:	mov    rdx,QWORD PTR [rsp+0x170]
  6550a7:	call   QWORD PTR [rip+0x995a3]        # 6ee650 <_DYNAMIC+0x250>
  6550ad:	jmp    6551d8 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3a98>
  6550b2:	mov    rsi,r14
  6550b5:	jmp    655097 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3957>
  6550b7:	mov    rsi,r13
  6550ba:	jmp    655097 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3957>
  6550bc:	mov    rsi,rax
  6550bf:	jmp    655097 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3957>
  6550c1:	mov    rsi,r15
  6550c4:	jmp    655097 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3957>
  6550c6:	lea    rdi,[rip+0xffffffffff9bd4ef]        # 125bc <anon.e5d9da81ce3e97b5bac39b4a40b5610e.70.llvm.17036336450247840325+0x1>
  6550cd:	lea    rdx,[rip+0x96934]        # 6eba08 <anon.e5d9da81ce3e97b5bac39b4a40b5610e.12.llvm.17036336450247840325+0x7c8>
  6550d4:	mov    esi,0x37
  6550d9:	call   QWORD PTR [rip+0x995d9]        # 6ee6b8 <_DYNAMIC+0x2b8>
  6550df:	jmp    6551d8 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3a98>
  6550e4:	sub    rsi,0xffffffffffffff80
  6550e8:	lea    rdi,[rsp+0xfc0]
  6550f0:	lea    rdx,[rsp+0x2c8]
  6550f8:	call   64ee90 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_3zip11ZipProducerIB2V_INtNtNtB1P_5slice6chunks14ChunksProducerhEB3p_EIB3q_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5X_s_0ENCB5X_0EE0NCB1G_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7f_E0TB7f_B7f_EEB61_>
  6550fd:	jmp    654fa7 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3867>
  655102:	sub    rsi,0xffffffffffffff80
  655106:	lea    rdi,[rsp+0xfc0]
  65510e:	lea    rcx,[rsp+0x2c8]
  655116:	call   64fb10 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_3zip11ZipProducerIB2W_INtNtNtB1Q_5slice6chunks14ChunksProducerhEB3q_EIB3r_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5Y_s_0ENCB5Y_0EE0NCB1H_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7g_E0TB7g_B7g_EEB62_>
  65511b:	jmp    654fa7 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3867>
  655120:	mov    QWORD PTR [rsp+0x30],rsi
  655125:	lea    rax,[rip+0x966cc]        # 6eb7f8 <anon.e5d9da81ce3e97b5bac39b4a40b5610e.12.llvm.17036336450247840325+0x5b8>
  65512c:	mov    QWORD PTR [rsp+0x170],rax
  655134:	jmp    655092 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3952>
  655139:	mov    QWORD PTR [rsp+0x1fc0],0x10
  655145:	cmp    QWORD PTR [rip+0x9cd83],0x10        # 6f1ed0 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.17036336450247840325+0x1040>
  65514d:	jne    6551ea <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3aaa>
  655153:	lea    rsi,[rsp+0x400]
  65515b:	mov    QWORD PTR [rsi],0x80
  655162:	lea    r9,[rip+0x9504f]        # 6ea1b8 <anon.01aa214378fce5a476fd64886db4a5c0.1334.llvm.5168012589948291949>
  655169:	jmp    655053 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3913>
  65516e:	mov    rdi,QWORD PTR [rsp+0x90]
  655176:	lea    rdx,[rip+0x9667b]        # 6eb7f8 <anon.e5d9da81ce3e97b5bac39b4a40b5610e.12.llvm.17036336450247840325+0x5b8>
  65517d:	mov    rsi,rdi
  655180:	vzeroupper 
  655183:	call   QWORD PTR [rip+0x994c7]        # 6ee650 <_DYNAMIC+0x250>
  655189:	jmp    6551d8 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3a98>
  65518b:	mov    edi,0x10
  655190:	jmp    6551bb <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3a7b>
  655192:	lea    rdx,[rip+0x9665f]        # 6eb7f8 <anon.e5d9da81ce3e97b5bac39b4a40b5610e.12.llvm.17036336450247840325+0x5b8>
  655199:	mov    rsi,rdi
  65519c:	call   QWORD PTR [rip+0x994ae]        # 6ee650 <_DYNAMIC+0x250>
  6551a2:	jmp    6551d8 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3a98>
  6551a4:	mov    edi,0x1
  6551a9:	mov    esi,0xff0
  6551ae:	call   QWORD PTR [rip+0x9948c]        # 6ee640 <_DYNAMIC+0x240>
  6551b4:	jmp    6551d8 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3a98>
  6551b6:	mov    edi,0x1
  6551bb:	mov    esi,0xff0
  6551c0:	call   QWORD PTR [rip+0x9947a]        # 6ee640 <_DYNAMIC+0x240>
  6551c6:	jmp    6551d8 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3a98>
  6551c8:	mov    edi,0x1
  6551cd:	mov    esi,0xff0
  6551d2:	call   QWORD PTR [rip+0x99468]        # 6ee640 <_DYNAMIC+0x240>
  6551d8:	ud2    
  6551da:	mov    edi,0x1
  6551df:	mov    esi,0xff0
  6551e4:	call   QWORD PTR [rip+0x99456]        # 6ee640 <_DYNAMIC+0x240>
  6551ea:	lea    r9,[rip+0x94faf]        # 6ea1a0 <anon.01aa214378fce5a476fd64886db4a5c0.1333.llvm.5168012589948291949>
  6551f1:	lea    rdx,[rip+0x9ccd8]        # 6f1ed0 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.17036336450247840325+0x1040>
  6551f8:	lea    rsi,[rsp+0x1fc0]
  655200:	jmp    65505a <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x391a>
  655205:	jmp    65523e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3afe>
  655207:	jmp    655228 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3ae8>
  655209:	mov    r15,rax
  65520c:	jmp    655246 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3b06>
  65520e:	mov    r15,rax
  655211:	mov    rdi,rbx
  655214:	mov    rbx,QWORD PTR [rip+0x993ed]        # 6ee608 <free@GLIBC_2.2.5>
  65521b:	call   rbx
  65521d:	mov    rdi,r14
  655220:	call   rbx
  655222:	jmp    65524f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3b0f>
  655224:	jmp    65523e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3afe>
  655226:	jmp    65523e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3afe>
  655228:	mov    r15,rax
  65522b:	mov    rbx,QWORD PTR [rsp+0x38]
  655230:	jmp    655246 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3b06>
  655232:	mov    r15,rax
  655235:	jmp    655246 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3b06>
  655237:	mov    r15,rax
  65523a:	jmp    655257 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3b17>
  65523c:	jmp    655243 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3b03>
  65523e:	mov    r15,rax
  655241:	jmp    65524f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3b0f>
  655243:	mov    r15,rax
  655246:	mov    rdi,rbx
  655249:	call   QWORD PTR [rip+0x993b9]        # 6ee608 <free@GLIBC_2.2.5>
  65524f:	cmp    QWORD PTR [rsp+0x60],0x0
  655255:	je     655262 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3b22>
  655257:	mov    rdi,QWORD PTR [rsp+0x68]
  65525c:	call   QWORD PTR [rip+0x993a6]        # 6ee608 <free@GLIBC_2.2.5>
  655262:	mov    rdi,r15
  655265:	call   6e7df0 <_Unwind_Resume@plt>
  65526a:	int3   
  65526b:	int3   
  65526c:	int3   
  65526d:	int3   
  65526e:	int3   
  65526f:	int3   

