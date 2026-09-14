0000000000657570 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_>:
  657570:	push   rbp
  657571:	mov    rbp,rsp
  657574:	push   r15
  657576:	push   r14
  657578:	push   r13
  65757a:	push   r12
  65757c:	push   rbx
  65757d:	and    rsp,0xffffffffffffffc0
  657581:	sub    rsp,0xfc0
  657588:	mov    QWORD PTR [rsp],0x0
  657590:	sub    rsp,0x1000
  657597:	mov    QWORD PTR [rsp],0x0
  65759f:	sub    rsp,0x1000
  6575a6:	mov    QWORD PTR [rsp],0x0
  6575ae:	sub    rsp,0x1000
  6575b5:	mov    QWORD PTR [rsp],0x0
  6575bd:	sub    rsp,0x480
  6575c4:	mov    QWORD PTR [rsp+0xd8],rdi
  6575cc:	mov    rbx,QWORD PTR [rbp+0x10]
  6575d0:	mov    QWORD PTR [rsp+0x268],rsi
  6575d8:	mov    QWORD PTR [rsp+0x270],rcx
  6575e0:	mov    QWORD PTR [rsp+0x278],r8
  6575e8:	shr    rsi,1
  6575eb:	cmp    rsi,r8
  6575ee:	jae    6576a1 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x131>
  6575f4:	mov    rax,QWORD PTR [r9]
  6575f7:	mov    QWORD PTR [rsp+0x180],rax
  6575ff:	mov    r14,QWORD PTR [r9+0x8]
  657603:	mov    r15,QWORD PTR [r9+0x10]
  657607:	mov    rax,QWORD PTR [r9+0x18]
  65760b:	mov    QWORD PTR [rsp+0x178],rax
  657613:	mov    r12,QWORD PTR [r9+0x20]
  657617:	mov    rax,QWORD PTR [r9+0x28]
  65761b:	mov    QWORD PTR [rsp+0xe0],rax
  657623:	mov    rax,QWORD PTR [r9+0x30]
  657627:	mov    QWORD PTR [rsp+0x170],rax
  65762f:	mov    rax,QWORD PTR [r9+0x38]
  657633:	mov    QWORD PTR [rsp+0x130],rax
  65763b:	mov    rax,QWORD PTR [r9+0x40]
  65763f:	mov    QWORD PTR [rsp+0x78],rax
  657644:	mov    rbx,QWORD PTR [rbx+0x10]
  657648:	mov    edi,0xff0
  65764d:	mov    esi,0x1
  657652:	call   QWORD PTR [rip+0xa0be0]        # 6f8238 <calloc@GLIBC_2.2.5>
  657658:	mov    QWORD PTR [rsp+0xe8],rax
  657660:	test   rax,rax
  657663:	je     65b34f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3ddf>
  657669:	test   r15,r15
  65766c:	mov    rax,QWORD PTR [rsp+0xe0]
  657674:	je     65b270 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3d00>
  65767a:	test   rax,rax
  65767d:	je     65b270 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3d00>
  657683:	test   r14,r14
  657686:	je     6576d5 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x165>
  657688:	mov    rax,r14
  65768b:	or     rax,r15
  65768e:	shr    rax,0x20
  657692:	je     6576e0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x170>
  657694:	mov    rax,r14
  657697:	xor    edx,edx
  657699:	div    r15
  65769c:	mov    rsi,rax
  65769f:	jmp    6576ea <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x17a>
  6576a1:	test   dl,dl
  6576a3:	je     6576c4 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x154>
  6576a5:	mov    rax,0xffffffffffffffa8
  6576ac:	mov    rax,QWORD PTR fs:[rax]
  6576b0:	test   rax,rax
  6576b3:	je     65af71 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3a01>
  6576b9:	add    rax,0x110
  6576bf:	jmp    65af89 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3a19>
  6576c4:	test   rcx,rcx
  6576c7:	je     6575f4 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x84>
  6576cd:	shr    rcx,1
  6576d0:	jmp    65af9d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3a2d>
  6576d5:	xor    esi,esi
  6576d7:	test   r12,r12
  6576da:	jne    6576ff <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x18f>
  6576dc:	xor    ecx,ecx
  6576de:	jmp    657733 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1c3>
  6576e0:	mov    eax,r14d
  6576e3:	xor    edx,edx
  6576e5:	div    r15d
  6576e8:	mov    esi,eax
  6576ea:	cmp    rdx,0x1
  6576ee:	sbb    rsi,0xffffffffffffffff
  6576f2:	mov    rax,QWORD PTR [rsp+0xe0]
  6576fa:	test   r12,r12
  6576fd:	je     6576dc <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x16c>
  6576ff:	mov    rcx,rax
  657702:	mov    rax,r12
  657705:	or     rax,rcx
  657708:	shr    rax,0x20
  65770c:	je     65771b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1ab>
  65770e:	mov    rax,r12
  657711:	xor    edx,edx
  657713:	div    rcx
  657716:	mov    rcx,rax
  657719:	jmp    657724 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1b4>
  65771b:	mov    eax,r12d
  65771e:	xor    edx,edx
  657720:	div    ecx
  657722:	mov    ecx,eax
  657724:	cmp    rdx,0x1
  657728:	sbb    rcx,0xffffffffffffffff
  65772c:	cmp    rcx,rsi
  65772f:	cmovae rcx,rsi
  657733:	mov    rax,QWORD PTR [rsp+0x78]
  657738:	test   rax,rax
  65773b:	je     65b270 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3d00>
  657741:	mov    rax,QWORD PTR [rsp+0x130]
  657749:	test   rax,rax
  65774c:	je     65af32 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x39c2>
  657752:	mov    rdx,rax
  657755:	mov    rsi,QWORD PTR [rsp+0x78]
  65775a:	or     rdx,rsi
  65775d:	shr    rdx,0x20
  657761:	je     65776a <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1fa>
  657763:	xor    edx,edx
  657765:	div    rsi
  657768:	jmp    65776e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1fe>
  65776a:	xor    edx,edx
  65776c:	div    esi
  65776e:	cmp    rdx,0x1
  657772:	sbb    rax,0xffffffffffffffff
  657776:	cmp    rax,rcx
  657779:	cmovae rax,rcx
  65777d:	test   rax,rax
  657780:	je     65af32 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x39c2>
  657786:	mov    QWORD PTR [rsp+0x218],rax
  65778e:	mov    rax,QWORD PTR [rbx]
  657791:	mov    QWORD PTR [rsp+0x238],rax
  657799:	movabs r13,0x7fffffffffffffc0
  6577a3:	mov    rax,QWORD PTR [rsp+0x78]
  6577a8:	shl    rax,0x4
  6577ac:	mov    QWORD PTR [rsp+0x210],rax
  6577b4:	mov    eax,0x87
  6577b9:	vmovq  xmm0,rax
  6577be:	vmovdqa XMMWORD PTR [rsp+0x3f0],xmm0
  6577c7:	mov    eax,0xff
  6577cc:	mov    QWORD PTR [rsp+0x50],rax
  6577d1:	xor    ecx,ecx
  6577d3:	lea    rax,[rip+0x9d67e]        # 6f4e58 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0xd8>
  6577da:	mov    QWORD PTR [rsp+0x158],rax
  6577e2:	mov    rax,QWORD PTR [rsp+0x178]
  6577ea:	mov    QWORD PTR [rsp+0x168],rax
  6577f2:	mov    rax,QWORD PTR [rsp+0x180]
  6577fa:	mov    QWORD PTR [rsp+0x160],rax
  657802:	mov    rax,QWORD PTR [rsp+0x170]
  65780a:	mov    QWORD PTR [rsp+0x138],rax
  657812:	mov    eax,0xff
  657817:	mov    QWORD PTR [rsp+0x80],rax
  65781f:	mov    QWORD PTR [rsp+0x230],r14
  657827:	mov    QWORD PTR [rsp+0x228],r15
  65782f:	mov    QWORD PTR [rsp+0x220],r12
  657837:	mov    rax,QWORD PTR [rsp+0xe0]
  65783f:	jmp    657955 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3e5>
  657844:	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  657850:	movabs r13,0x7fffffffffffffc0
  65785a:	mov    r15,QWORD PTR [rsp+0xf0]
  657862:	inc    r15
  657865:	mov    QWORD PTR [rsp+0x2418],0xff
  657871:	mov    QWORD PTR [rsp+0x2420],r12
  657879:	mov    QWORD PTR [rsp+0x2428],0xff
  657885:	mov    rax,QWORD PTR [rsp+0x50]
  65788a:	mov    QWORD PTR [rsp+0x2400],rax
  657892:	mov    rax,QWORD PTR [rsp+0xe8]
  65789a:	mov    QWORD PTR [rsp+0x2408],rax
  6578a2:	mov    rax,QWORD PTR [rsp+0x80]
  6578aa:	mov    QWORD PTR [rsp+0x2410],rax
  6578b2:	lea    rdi,[rsp+0x1400]
  6578ba:	lea    rsi,[rsp+0x2400]
  6578c2:	lea    rdx,[rsp+0x2418]
  6578ca:	call   6643c0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0B5_>
  6578cf:	mov    rax,QWORD PTR [rsp+0x1400]
  6578d7:	mov    QWORD PTR [rsp+0x50],rax
  6578dc:	mov    rax,QWORD PTR [rsp+0x1408]
  6578e4:	mov    QWORD PTR [rsp+0xe8],rax
  6578ec:	mov    rax,QWORD PTR [rsp+0x1410]
  6578f4:	mov    QWORD PTR [rsp+0x80],rax
  6578fc:	mov    rax,QWORD PTR [rsp+0x138]
  657904:	add    rax,QWORD PTR [rsp+0x210]
  65790c:	mov    QWORD PTR [rsp+0x138],rax
  657914:	mov    rcx,r15
  657917:	mov    r15,QWORD PTR [rsp+0x228]
  65791f:	add    QWORD PTR [rsp+0x160],r15
  657927:	mov    rax,QWORD PTR [rsp+0xe0]
  65792f:	add    QWORD PTR [rsp+0x168],rax
  657937:	cmp    rcx,QWORD PTR [rsp+0x218]
  65793f:	mov    r14,QWORD PTR [rsp+0x230]
  657947:	mov    r12,QWORD PTR [rsp+0x220]
  65794f:	je     65af44 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x39d4>
  657955:	mov    rdx,rcx
  657958:	imul   rdx,r15
  65795c:	sub    r14,rdx
  65795f:	cmp    r15,r14
  657962:	cmovb  r14,r15
  657966:	mov    QWORD PTR [rsp+0x58],r14
  65796b:	add    rdx,QWORD PTR [rsp+0x180]
  657973:	mov    QWORD PTR [rsp+0x100],rdx
  65797b:	mov    r14,rcx
  65797e:	imul   r14,rax
  657982:	mov    rdx,r12
  657985:	sub    rdx,r14
  657988:	cmp    rax,rdx
  65798b:	cmovb  rdx,rax
  65798f:	mov    QWORD PTR [rsp+0x18],rdx
  657994:	mov    rax,QWORD PTR [rsp+0x178]
  65799c:	add    rax,r14
  65799f:	mov    QWORD PTR [rsp+0xf8],rax
  6579a7:	mov    rbx,rcx
  6579aa:	mov    rax,QWORD PTR [rsp+0x78]
  6579af:	imul   rbx,rax
  6579b3:	mov    rdx,QWORD PTR [rsp+0x130]
  6579bb:	sub    rdx,rbx
  6579be:	cmp    rax,rdx
  6579c1:	cmovb  rdx,rax
  6579c5:	mov    QWORD PTR [rsp+0x20],rdx
  6579ca:	mov    rax,rbx
  6579cd:	shl    rax,0x4
  6579d1:	add    rax,QWORD PTR [rsp+0x170]
  6579d9:	mov    QWORD PTR [rsp+0x108],rax
  6579e1:	mov    rax,QWORD PTR [rsp+0x238]
  6579e9:	movzx  eax,BYTE PTR [rax]
  6579ec:	lea    rdx,[rip+0xffffffffff9bab8d]        # 12580 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1532>
  6579f3:	movsxd rax,DWORD PTR [rdx+rax*4]
  6579f7:	add    rax,rdx
  6579fa:	mov    QWORD PTR [rsp+0xf0],rcx
  657a02:	jmp    rax
  657a04:	mov    eax,DWORD PTR [rip+0xa309e]        # 6faaa8 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.16073103944965275176+0x930>
  657a0a:	test   eax,eax
  657a0c:	jne    65aee2 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3972>
  657a12:	mov    eax,DWORD PTR [rip+0xa41b8]        # 6fbbd0 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.16073103944965275176+0x1120>
  657a18:	test   eax,eax
  657a1a:	jne    65aeec <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x397c>
  657a20:	mov    edi,0xff0
  657a25:	mov    esi,0x1
  657a2a:	call   QWORD PTR [rip+0xa0808]        # 6f8238 <calloc@GLIBC_2.2.5>
  657a30:	test   rax,rax
  657a33:	je     65b300 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3d90>
  657a39:	mov    rbx,rax
  657a3c:	mov    rcx,QWORD PTR [rsp+0x58]
  657a41:	shr    rcx,0x4
  657a45:	mov    rax,QWORD PTR [rsp+0x18]
  657a4a:	shr    rax,0x4
  657a4e:	cmp    rax,rcx
  657a51:	cmovae rax,rcx
  657a55:	mov    rcx,QWORD PTR [rsp+0x20]
  657a5a:	cmp    rcx,rax
  657a5d:	cmovb  rax,rcx
  657a61:	test   rax,rax
  657a64:	lea    r12,[rsp+0x1400]
  657a6c:	je     657de0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x870>
  657a72:	mov    r13,rax
  657a75:	xor    r14d,r14d
  657a78:	nop    DWORD PTR [rax+rax*1+0x0]
  657a80:	mov    r15,r14
  657a83:	shl    r15,0x4
  657a87:	mov    rax,QWORD PTR [rsp+0x100]
  657a8f:	lea    rsi,[rax+r15*1]
  657a93:	lea    rdi,[rsp+0x440]
  657a9b:	vzeroupper 
  657a9e:	call   667e30 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate6domainNtB2_6Domain8eval_row>
  657aa3:	mov    rax,QWORD PTR [rsp+0xf8]
  657aab:	lea    rsi,[rax+r15*1]
  657aaf:	mov    rdi,r12
  657ab2:	call   667e30 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate6domainNtB2_6Domain8eval_row>
  657ab7:	inc    r14
  657aba:	vmovups ymm0,YMMWORD PTR [rsp+0x51f]
  657ac3:	lea    rcx,[rsp+0x2420]
  657acb:	vmovups YMMWORD PTR [rcx+0xdf],ymm0
  657ad3:	vmovups ymm0,YMMWORD PTR [rsp+0x500]
  657adc:	vmovups YMMWORD PTR [rcx+0xc0],ymm0
  657ae4:	vmovups ymm0,YMMWORD PTR [rsp+0x4e0]
  657aed:	vmovups YMMWORD PTR [rcx+0xa0],ymm0
  657af5:	vmovups ymm0,YMMWORD PTR [rsp+0x4c0]
  657afe:	vmovups YMMWORD PTR [rcx+0x80],ymm0
  657b06:	vmovups ymm0,YMMWORD PTR [rsp+0x440]
  657b0f:	vmovups ymm1,YMMWORD PTR [rsp+0x460]
  657b18:	vmovups ymm2,YMMWORD PTR [rsp+0x480]
  657b21:	vmovups ymm3,YMMWORD PTR [rsp+0x4a0]
  657b2a:	vmovups YMMWORD PTR [rcx+0x60],ymm3
  657b2f:	vmovups YMMWORD PTR [rcx+0x40],ymm2
  657b34:	vmovups YMMWORD PTR [rcx+0x20],ymm1
  657b39:	vmovups YMMWORD PTR [rcx],ymm0
  657b3d:	lea    rax,[rsp+0x1001]
  657b45:	vxorps xmm4,xmm4,xmm4
  657b49:	vmovups XMMWORD PTR [rax],xmm4
  657b4d:	vmovups ymm0,YMMWORD PTR [rsp+0x14df]
  657b56:	lea    rdx,[rsp+0x2540]
  657b5e:	vmovups YMMWORD PTR [rdx+0xdf],ymm0
  657b66:	vmovups ymm0,YMMWORD PTR [rsp+0x14c0]
  657b6f:	vmovups YMMWORD PTR [rdx+0xc0],ymm0
  657b77:	vmovups ymm0,YMMWORD PTR [rsp+0x14a0]
  657b80:	vmovups YMMWORD PTR [rdx+0xa0],ymm0
  657b88:	vmovups ymm0,YMMWORD PTR [rsp+0x1480]
  657b91:	vmovups YMMWORD PTR [rdx+0x80],ymm0
  657b99:	vmovups ymm0,YMMWORD PTR [rsp+0x1400]
  657ba2:	vmovups ymm1,YMMWORD PTR [rsp+0x1420]
  657bab:	vmovups ymm2,YMMWORD PTR [rsp+0x1440]
  657bb4:	vmovups ymm3,YMMWORD PTR [rsp+0x1460]
  657bbd:	vmovups YMMWORD PTR [rdx+0x60],ymm3
  657bc2:	vmovups YMMWORD PTR [rdx+0x40],ymm2
  657bc7:	vmovups YMMWORD PTR [rdx+0x20],ymm1
  657bcc:	vmovups YMMWORD PTR [rdx],ymm0
  657bd0:	add    r15,QWORD PTR [rsp+0x108]
  657bd8:	movzx  eax,BYTE PTR [rsp+0x1010]
  657be0:	vmovups xmm0,XMMWORD PTR [rsp+0x1000]
  657be9:	vmovups XMMWORD PTR [rcx+0xff],xmm0
  657bf1:	mov    BYTE PTR [rcx+0x10f],al
  657bf7:	lea    rax,[rsp+0x841]
  657bff:	vmovups XMMWORD PTR [rax],xmm4
  657c03:	mov    QWORD PTR [rsp+0x2530],0x0
  657c0f:	mov    QWORD PTR [rsp+0x2538],0xff
  657c1b:	movzx  eax,BYTE PTR [rsp+0x850]
  657c23:	mov    BYTE PTR [rcx+0x22f],al
  657c29:	vmovups xmm0,XMMWORD PTR [rsp+0x840]
  657c32:	vmovups XMMWORD PTR [rcx+0x21f],xmm0
  657c3a:	mov    rax,QWORD PTR [r15]
  657c3d:	mov    rcx,QWORD PTR [r15+0x8]
  657c41:	vmovq  xmm0,rax
  657c46:	vmovq  xmm1,rcx
  657c4b:	xor    rcx,rax
  657c4e:	vmovq  xmm2,rcx
  657c53:	mov    rax,rdx
  657c56:	xor    ecx,ecx
  657c58:	vmovdqa xmm8,XMMWORD PTR [rsp+0x3f0]
  657c61:	mov    r10,QWORD PTR [rip+0xa09e0]        # 6f8648 <_DYNAMIC+0x630>
  657c68:	nop    DWORD PTR [rax+rax*1+0x0]
  657c70:	movzx  edx,BYTE PTR [rax-0x120]
  657c77:	shl    edx,0x4
  657c7a:	mov    rsi,QWORD PTR [r10+rdx*1]
  657c7e:	mov    rdx,QWORD PTR [r10+rdx*1+0x8]
  657c83:	vmovq  xmm3,rsi
  657c88:	vpclmullqlqdq xmm3,xmm0,xmm3
  657c8e:	vmovq  xmm4,rdx
  657c93:	vpclmullqlqdq xmm4,xmm1,xmm4
  657c99:	xor    rdx,rsi
  657c9c:	vmovq  xmm5,rdx
  657ca1:	vpclmullqlqdq xmm5,xmm2,xmm5
  657ca7:	vpternlogq xmm5,xmm3,xmm4,0x96
  657cae:	vpshufd xmm6,xmm4,0xee
  657cb3:	vpclmullqlqdq xmm6,xmm6,xmm8
  657cb9:	vpshufd xmm7,xmm5,0xee
  657cbe:	vpxor  xmm4,xmm7,xmm4
  657cc2:	movzx  edx,BYTE PTR [rax]
  657cc5:	vpclmullqlqdq xmm4,xmm4,xmm8
  657ccb:	vpextrq rsi,xmm6,0x1
  657cd1:	lea    rdi,[rsi+rsi*1]
  657cd5:	lea    r8,[rsi*4+0x0]
  657cdd:	mov    r9,rsi
  657ce0:	xor    r8,rdi
  657ce3:	shl    r9,0x7
  657ce7:	xor    r9,r8
  657cea:	vpxor  xmm3,xmm3,xmm4
  657cee:	vmovq  rdi,xmm3
  657cf3:	vpshufd xmm3,xmm3,0xee
  657cf8:	xor    rdi,r9
  657cfb:	vpternlogq xmm3,xmm5,xmm6,0x96
  657d02:	vmovq  r8,xmm3
  657d07:	shl    edx,0x4
  657d0a:	mov    r9,QWORD PTR [r10+rdx*1]
  657d0e:	mov    rdx,QWORD PTR [r10+rdx*1+0x8]
  657d13:	xor    rdi,rsi
  657d16:	vmovq  xmm4,rdi
  657d1b:	vmovq  xmm5,r9
  657d20:	vmovq  xmm6,rdx
  657d25:	vpclmullqlqdq xmm3,xmm3,xmm6
  657d2b:	xor    r8,rdi
  657d2e:	vpclmullqlqdq xmm4,xmm4,xmm5
  657d34:	xor    rdx,r9
  657d37:	vmovq  xmm5,r8
  657d3c:	vmovq  xmm6,rdx
  657d41:	vpclmullqlqdq xmm5,xmm5,xmm6
  657d47:	vpshufd xmm6,xmm3,0xee
  657d4c:	vpternlogq xmm5,xmm4,xmm3,0x96
  657d53:	vpclmullqlqdq xmm6,xmm6,xmm8
  657d59:	vpshufd xmm7,xmm5,0xee
  657d5e:	vpxor  xmm3,xmm7,xmm3
  657d62:	vpclmullqlqdq xmm3,xmm3,xmm8
  657d68:	vpextrq rdx,xmm6,0x1
  657d6e:	lea    rsi,[rdx+rdx*1]
  657d72:	shl    rdx,0x2
  657d76:	xor    rdx,rsi
  657d79:	vpxor  xmm7,xmm4,xmm3
  657d7d:	vmovq  rsi,xmm7
  657d82:	xor    rsi,rdx
  657d85:	vpshufd xmm4,xmm4,0xee
  657d8a:	vpsllq xmm7,xmm6,0x7
  657d8f:	vpxor  xmm4,xmm6,xmm4
  657d93:	vpbroadcastq xmm5,xmm5
  657d98:	vpinsrq xmm5,xmm5,rsi,0x0
  657d9e:	vpalignr xmm4,xmm4,xmm7,0x8
  657da4:	vmovdqa xmm7,XMMWORD PTR [rbx+rcx*1]
  657da9:	vpblendd xmm3,xmm7,xmm3,0xc
  657daf:	vpunpckhqdq xmm6,xmm6,xmm7
  657db3:	vpxor  xmm4,xmm4,xmm5
  657db7:	vpternlogq xmm6,xmm4,xmm3,0x96
  657dbe:	vmovdqa XMMWORD PTR [rbx+rcx*1],xmm6
  657dc3:	add    rcx,0x10
  657dc7:	inc    rax
  657dca:	cmp    rcx,0xff0
  657dd1:	jne    657c70 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x700>
  657dd7:	cmp    r14,r13
  657dda:	jne    657a80 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x510>
  657de0:	mov    edi,0xff0
  657de5:	vzeroupper 
  657de8:	call   QWORD PTR [rip+0xa0422]        # 6f8210 <malloc@GLIBC_2.2.5>
  657dee:	test   rax,rax
  657df1:	je     65b307 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3d97>
  657df7:	mov    r12,rax
  657dfa:	mov    rdi,QWORD PTR [rip+0xa238f]        # 6fa190 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.16073103944965275176+0x18>
  657e01:	cmp    rdi,0x10
  657e05:	jb     65b319 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3da9>
  657e0b:	mov    rax,QWORD PTR [rip+0xa2376]        # 6fa188 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.16073103944965275176+0x10>
  657e12:	lea    rdx,[rax+0xd000]
  657e19:	lea    rsi,[rax+0xe000]
  657e20:	lea    rdi,[rax+0xf000]
  657e27:	xor    r8d,r8d
  657e2a:	nop    WORD PTR [rax+rax*1+0x0]
  657e30:	mov    r9,QWORD PTR [rbx+r8*1]
  657e34:	mov    r14,r9
  657e37:	mov    r11,r9
  657e3a:	mov    r15,r9
  657e3d:	mov    r13,r9
  657e40:	mov    r10d,r9d
  657e43:	shr    r10d,0x4
  657e47:	and    r10d,0xff0
  657e4e:	lea    rcx,[rax+0x1000]
  657e55:	vmovdqa xmm0,XMMWORD PTR [rcx+r10*1]
  657e5b:	movzx  r10d,r9b
  657e5f:	shl    r10d,0x4
  657e63:	vpxor  xmm0,xmm0,XMMWORD PTR [rax+r10*1]
  657e69:	mov    r10d,r9d
  657e6c:	shr    r10d,0xc
  657e70:	and    r10d,0xff0
  657e77:	shr    r9d,0x14
  657e7b:	and    r9d,0xfffffff0
  657e7f:	lea    rcx,[rax+0x3000]
  657e86:	vmovdqa xmm1,XMMWORD PTR [rcx+r9*1]
  657e8c:	lea    rcx,[rax+0x2000]
  657e93:	vpternlogq xmm1,xmm0,XMMWORD PTR [rcx+r10*1],0x96
  657e9b:	mov    r10,QWORD PTR [rbx+r8*1+0x8]
  657ea0:	mov    r9,r10
  657ea3:	shr    r15,0x28
  657ea7:	movzx  r15d,r15b
  657eab:	shl    r15d,0x4
  657eaf:	lea    rcx,[rax+0x5000]
  657eb6:	vmovdqa xmm0,XMMWORD PTR [rcx+r15*1]
  657ebc:	mov    r15,r10
  657ebf:	shr    r13,0x20
  657ec3:	movzx  r13d,r13b
  657ec7:	shl    r13d,0x4
  657ecb:	lea    rcx,[rax+0x4000]
  657ed2:	vpternlogq xmm0,xmm1,XMMWORD PTR [rcx+r13*1],0x96
  657eda:	mov    r13,r10
  657edd:	shr    r14,0x38
  657ee1:	shl    r14d,0x4
  657ee5:	lea    rcx,[rax+0x7000]
  657eec:	vmovdqa xmm1,XMMWORD PTR [rcx+r14*1]
  657ef2:	mov    r14,r10
  657ef5:	shr    r11,0x30
  657ef9:	movzx  r11d,r11b
  657efd:	shl    r11d,0x4
  657f01:	lea    rcx,[rax+0x6000]
  657f08:	vpternlogq xmm1,xmm0,XMMWORD PTR [rcx+r11*1],0x96
  657f10:	mov    r11d,r10d
  657f13:	shr    r11d,0x4
  657f17:	and    r11d,0xff0
  657f1e:	lea    rcx,[rax+0x9000]
  657f25:	vmovdqa xmm0,XMMWORD PTR [rcx+r11*1]
  657f2b:	movzx  r11d,r10b
  657f2f:	shl    r11d,0x4
  657f33:	lea    rcx,[rax+0x8000]
  657f3a:	vpternlogq xmm0,xmm1,XMMWORD PTR [rcx+r11*1],0x96
  657f42:	mov    r11d,r10d
  657f45:	shr    r10d,0x14
  657f49:	and    r10d,0xfffffff0
  657f4d:	lea    rcx,[rax+0xb000]
  657f54:	vmovdqa xmm1,XMMWORD PTR [rcx+r10*1]
  657f5a:	shr    r11d,0xc
  657f5e:	and    r11d,0xff0
  657f65:	lea    rcx,[rax+0xa000]
  657f6c:	vpternlogq xmm1,xmm0,XMMWORD PTR [rcx+r11*1],0x96
  657f74:	shr    r13,0x28
  657f78:	movzx  r10d,r13b
  657f7c:	shl    r10d,0x4
  657f80:	vmovdqa xmm0,XMMWORD PTR [rdx+r10*1]
  657f86:	shr    r14,0x20
  657f8a:	movzx  r10d,r14b
  657f8e:	shl    r10d,0x4
  657f92:	lea    rcx,[rax+0xc000]
  657f99:	vpternlogq xmm0,xmm1,XMMWORD PTR [rcx+r10*1],0x96
  657fa1:	shr    r9,0x38
  657fa5:	shl    r9d,0x4
  657fa9:	vmovdqa xmm1,XMMWORD PTR [rdi+r9*1]
  657faf:	shr    r15,0x30
  657fb3:	movzx  r9d,r15b
  657fb7:	shl    r9d,0x4
  657fbb:	vpternlogq xmm1,xmm0,XMMWORD PTR [rsi+r9*1],0x96
  657fc3:	vmovdqu XMMWORD PTR [r12+r8*1],xmm1
  657fc9:	add    r8,0x10
  657fcd:	cmp    r8,0xff0
  657fd4:	jne    657e30 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x8c0>
  657fda:	mov    rdi,rbx
  657fdd:	call   QWORD PTR [rip+0xa023d]        # 6f8220 <free@GLIBC_2.2.5>
  657fe3:	jmp    657850 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2e0>
  657fe8:	nop    DWORD PTR [rax+rax*1+0x0]
  657ff0:	mov    r15,rcx
  657ff3:	mov    eax,DWORD PTR [rip+0xa2aaf]        # 6faaa8 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.16073103944965275176+0x930>
  657ff9:	test   eax,eax
  657ffb:	jne    65aef6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3986>
  658001:	mov    eax,DWORD PTR [rip+0xa3bc9]        # 6fbbd0 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.16073103944965275176+0x1120>
  658007:	test   eax,eax
  658009:	jne    65af00 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3990>
  65800f:	mov    edi,0xff0
  658014:	mov    esi,0x1
  658019:	call   QWORD PTR [rip+0xa0219]        # 6f8238 <calloc@GLIBC_2.2.5>
  65801f:	test   rax,rax
  658022:	je     65b300 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3d90>
  658028:	mov    QWORD PTR [rsp+0x38],rax
  65802d:	mov    edx,0x1000
  658032:	lea    rdi,[rsp+0x2400]
  65803a:	xor    esi,esi
  65803c:	call   QWORD PTR [rip+0xa01fe]        # 6f8240 <memset@GLIBC_2.2.5>
  658042:	mov    rcx,QWORD PTR [rsp+0x58]
  658047:	shr    rcx,0x4
  65804b:	mov    rax,QWORD PTR [rsp+0x18]
  658050:	shr    rax,0x4
  658054:	cmp    rax,rcx
  658057:	cmovae rax,rcx
  65805b:	mov    rcx,QWORD PTR [rsp+0x20]
  658060:	cmp    rcx,rax
  658063:	cmovb  rax,rcx
  658067:	mov    QWORD PTR [rsp+0x18],rax
  65806c:	test   rax,rax
  65806f:	je     6589c2 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1452>
  658075:	xor    ebx,ebx
  658077:	nop    WORD PTR [rax+rax*1+0x0]
  658080:	mov    r15,rbx
  658083:	shl    r15,0x4
  658087:	mov    rax,QWORD PTR [rsp+0x108]
  65808f:	mov    r14,QWORD PTR [rax+r15*1]
  658093:	mov    r12,QWORD PTR [rax+r15*1+0x8]
  658098:	mov    rax,QWORD PTR [rsp+0x100]
  6580a0:	lea    rsi,[rax+r15*1]
  6580a4:	lea    rdi,[rsp+0x1000]
  6580ac:	vzeroupper 
  6580af:	call   667e30 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate6domainNtB2_6Domain8eval_row>
  6580b4:	add    r15,QWORD PTR [rsp+0xf8]
  6580bc:	lea    rdi,[rsp+0x440]
  6580c4:	mov    rsi,r15
  6580c7:	call   667e30 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate6domainNtB2_6Domain8eval_row>
  6580cc:	mov    rsi,QWORD PTR [rip+0xa20bd]        # 6fa190 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.16073103944965275176+0x18>
  6580d3:	cmp    rsi,0x10
  6580d7:	lea    r13,[rsp+0x1400]
  6580df:	jb     65b2ca <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3d5a>
  6580e5:	inc    rbx
  6580e8:	mov    QWORD PTR [rsp+0x140],rbx
  6580f0:	mov    rax,r12
  6580f3:	shr    rax,0x38
  6580f7:	mov    rcx,r12
  6580fa:	shr    rcx,0x30
  6580fe:	mov    rsi,r12
  658101:	shr    rsi,0x28
  658105:	mov    rdi,r12
  658108:	mov    r8,r14
  65810b:	shr    r8,0x38
  65810f:	mov    r9,r14
  658112:	shr    r9,0x30
  658116:	mov    r10,r14
  658119:	shr    r10,0x28
  65811d:	mov    r11,r14
  658120:	mov    rdx,QWORD PTR [rip+0xa2061]        # 6fa188 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.16073103944965275176+0x10>
  658127:	movzx  ebx,r14b
  65812b:	shl    ebx,0x4
  65812e:	mov    r15d,r14d
  658131:	shr    r15d,0x4
  658135:	and    r15d,0xff0
  65813c:	vmovdqa xmm0,XMMWORD PTR [rdx+r15*1+0x1000]
  658146:	vpxor  xmm0,xmm0,XMMWORD PTR [rdx+rbx*1]
  65814b:	mov    ebx,r14d
  65814e:	shr    ebx,0xc
  658151:	and    ebx,0xff0
  658157:	shr    r14d,0x14
  65815b:	and    r14d,0xfffffff0
  65815f:	vmovdqa xmm1,XMMWORD PTR [rdx+r14*1+0x3000]
  658169:	vpternlogq xmm1,xmm0,XMMWORD PTR [rdx+rbx*1+0x2000],0x96
  658175:	shr    r11,0x20
  658179:	movzx  r11d,r11b
  65817d:	movzx  r10d,r10b
  658181:	shl    r10d,0x4
  658185:	vmovdqa xmm0,XMMWORD PTR [rdx+r10*1+0x5000]
  65818f:	shl    r11d,0x4
  658193:	vpternlogq xmm0,xmm1,XMMWORD PTR [rdx+r11*1+0x4000],0x96
  65819f:	movzx  r9d,r9b
  6581a3:	shl    r9d,0x4
  6581a7:	shl    r8d,0x4
  6581ab:	vmovdqa xmm1,XMMWORD PTR [rdx+r8*1+0x7000]
  6581b5:	vpternlogq xmm1,xmm0,XMMWORD PTR [rdx+r9*1+0x6000],0x96
  6581c1:	movzx  r8d,r12b
  6581c5:	shl    r8d,0x4
  6581c9:	mov    r9d,r12d
  6581cc:	shr    r9d,0x4
  6581d0:	and    r9d,0xff0
  6581d7:	vmovdqa xmm0,XMMWORD PTR [rdx+r9*1+0x9000]
  6581e1:	vpternlogq xmm0,xmm1,XMMWORD PTR [rdx+r8*1+0x8000],0x96
  6581ed:	shr    rdi,0x20
  6581f1:	mov    r8d,r12d
  6581f4:	shr    r8d,0xc
  6581f8:	shr    r12d,0x14
  6581fc:	and    r12d,0xfffffff0
  658200:	vmovdqa xmm1,XMMWORD PTR [rdx+r12*1+0xb000]
  65820a:	and    r8d,0xff0
  658211:	vpternlogq xmm1,xmm0,XMMWORD PTR [rdx+r8*1+0xa000],0x96
  65821d:	movzx  edi,dil
  658221:	shl    edi,0x4
  658224:	movzx  esi,sil
  658228:	shl    esi,0x4
  65822b:	vmovdqa xmm0,XMMWORD PTR [rdx+rsi*1+0xd000]
  658234:	vpternlogq xmm0,xmm1,XMMWORD PTR [rdx+rdi*1+0xc000],0x96
  658240:	movzx  ecx,cl
  658243:	shl    ecx,0x4
  658246:	shl    eax,0x4
  658249:	vmovdqa xmm1,XMMWORD PTR [rdx+rax*1+0xf000]
  658252:	vpternlogq xmm1,xmm0,XMMWORD PTR [rdx+rcx*1+0xe000],0x96
  65825e:	vpextrq rcx,xmm1,0x1
  658264:	vmovq  rdx,xmm1
  658269:	mov    eax,edx
  65826b:	shl    eax,0x8
  65826e:	movzx  eax,ax
  658271:	mov    QWORD PTR [rsp+0xc8],rax
  658279:	mov    eax,edx
  65827b:	and    eax,0xff00
  658280:	mov    QWORD PTR [rsp+0xc0],rax
  658288:	mov    eax,edx
  65828a:	shr    eax,0x8
  65828d:	and    eax,0xff00
  658292:	mov    QWORD PTR [rsp+0xb8],rax
  65829a:	mov    eax,edx
  65829c:	shr    eax,0x10
  65829f:	and    eax,0xff00
  6582a4:	mov    QWORD PTR [rsp+0xb0],rax
  6582ac:	mov    rax,rdx
  6582af:	shr    rax,0x18
  6582b3:	and    eax,0xff00
  6582b8:	mov    QWORD PTR [rsp+0xa8],rax
  6582c0:	mov    rax,rdx
  6582c3:	shr    rax,0x20
  6582c7:	and    eax,0xff00
  6582cc:	mov    QWORD PTR [rsp+0xa0],rax
  6582d4:	mov    rax,rdx
  6582d7:	shr    rax,0x28
  6582db:	and    eax,0xff00
  6582e0:	mov    QWORD PTR [rsp+0x98],rax
  6582e8:	shr    rdx,0x30
  6582ec:	and    edx,0xff00
  6582f2:	mov    QWORD PTR [rsp+0x90],rdx
  6582fa:	mov    eax,ecx
  6582fc:	shl    eax,0x8
  6582ff:	movzx  eax,ax
  658302:	mov    QWORD PTR [rsp+0x128],rax
  65830a:	mov    eax,ecx
  65830c:	and    eax,0xff00
  658311:	mov    QWORD PTR [rsp+0x120],rax
  658319:	mov    eax,ecx
  65831b:	shr    eax,0x8
  65831e:	and    eax,0xff00
  658323:	mov    QWORD PTR [rsp+0x118],rax
  65832b:	mov    eax,ecx
  65832d:	shr    eax,0x10
  658330:	and    eax,0xff00
  658335:	mov    QWORD PTR [rsp+0x88],rax
  65833d:	mov    rax,rcx
  658340:	shr    rax,0x18
  658344:	and    eax,0xff00
  658349:	mov    QWORD PTR [rsp+0x400],rax
  658351:	mov    rax,rcx
  658354:	shr    rax,0x20
  658358:	and    eax,0xff00
  65835d:	mov    QWORD PTR [rsp+0x110],rax
  658365:	mov    rax,rcx
  658368:	shr    rax,0x28
  65836c:	and    eax,0xff00
  658371:	mov    QWORD PTR [rsp+0x20],rax
  658376:	shr    rcx,0x30
  65837a:	and    ecx,0xff00
  658380:	mov    QWORD PTR [rsp+0x150],rcx
  658388:	mov    rdi,QWORD PTR [rip+0xa1df1]        # 6fa180 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.16073103944965275176+0x8>
  65838f:	mov    r11,QWORD PTR [rip+0xa1de2]        # 6fa178 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.16073103944965275176>
  658396:	mov    rbx,r13
  658399:	xor    ecx,ecx
  65839b:	mov    QWORD PTR [rsp+0x1c0],rdi
  6583a3:	mov    QWORD PTR [rsp+0x148],r11
  6583ab:	nop    DWORD PTR [rax+rax*1+0x0]
  6583b0:	mov    al,0x1
  6583b2:	shl    al,cl
  6583b4:	movzx  r9d,al
  6583b8:	mov    rsi,QWORD PTR [rsp+0xc8]
  6583c0:	or     rsi,r9
  6583c3:	cmp    rsi,rdi
  6583c6:	jae    65b24d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cdd>
  6583cc:	mov    r12,QWORD PTR [rsp+0xc0]
  6583d4:	or     r12,r9
  6583d7:	cmp    r12,rdi
  6583da:	jae    65b245 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cd5>
  6583e0:	mov    rax,QWORD PTR [rsp+0xb8]
  6583e8:	or     rax,r9
  6583eb:	cmp    rax,rdi
  6583ee:	jae    65b222 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cb2>
  6583f4:	mov    r13,QWORD PTR [rsp+0xb0]
  6583fc:	or     r13,r9
  6583ff:	cmp    r13,rdi
  658402:	jae    65b21d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cad>
  658408:	mov    r14,QWORD PTR [rsp+0xa8]
  658410:	or     r14,r9
  658413:	cmp    r14,rdi
  658416:	jae    65b240 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cd0>
  65841c:	mov    r15,QWORD PTR [rsp+0xa0]
  658424:	or     r15,r9
  658427:	cmp    r15,rdi
  65842a:	jae    65b218 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3ca8>
  658430:	mov    r10,QWORD PTR [rsp+0x98]
  658438:	or     r10,r9
  65843b:	cmp    r10,rdi
  65843e:	jae    65b227 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cb7>
  658444:	mov    r11,QWORD PTR [rsp+0x90]
  65844c:	or     r11,r9
  65844f:	cmp    r11,rdi
  658452:	jae    65b231 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cc1>
  658458:	mov    rdx,QWORD PTR [rsp+0x128]
  658460:	or     rdx,r9
  658463:	cmp    rdx,rdi
  658466:	jae    65b236 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cc6>
  65846c:	mov    r8,QWORD PTR [rsp+0x120]
  658474:	or     r8,r9
  658477:	cmp    r8,rdi
  65847a:	jae    65b24a <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cda>
  658480:	mov    QWORD PTR [rsp+0x1a0],r15
  658488:	mov    r15,QWORD PTR [rsp+0x118]
  658490:	or     r15,r9
  658493:	cmp    r15,rdi
  658496:	jae    65b218 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3ca8>
  65849c:	mov    QWORD PTR [rsp+0x280],r14
  6584a4:	mov    QWORD PTR [rsp+0x60],rcx
  6584a9:	mov    r14,rbx
  6584ac:	mov    rbx,QWORD PTR [rsp+0x88]
  6584b4:	or     rbx,r9
  6584b7:	cmp    rbx,rdi
  6584ba:	jae    65b23b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3ccb>
  6584c0:	mov    QWORD PTR [rsp+0x30],r13
  6584c5:	mov    r13,QWORD PTR [rsp+0x400]
  6584cd:	or     r13,r9
  6584d0:	cmp    r13,rdi
  6584d3:	jae    65b21d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cad>
  6584d9:	mov    QWORD PTR [rsp+0x28],rax
  6584de:	mov    rax,QWORD PTR [rsp+0x110]
  6584e6:	or     rax,r9
  6584e9:	cmp    rax,rdi
  6584ec:	jae    65b222 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cb2>
  6584f2:	mov    QWORD PTR [rsp+0xd0],rsi
  6584fa:	mov    rsi,QWORD PTR [rsp+0x20]
  6584ff:	or     rsi,r9
  658502:	cmp    rsi,rdi
  658505:	jae    65b24d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cdd>
  65850b:	or     r9,QWORD PTR [rsp+0x150]
  658513:	cmp    r9,rdi
  658516:	jae    65b22c <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cbc>
  65851c:	mov    rdi,QWORD PTR [rsp+0x148]
  658524:	movzx  ecx,BYTE PTR [rdi+r8*1]
  658529:	shl    ecx,0x8
  65852c:	movzx  edx,BYTE PTR [rdi+rdx*1]
  658530:	or     rdx,rcx
  658533:	movzx  ecx,BYTE PTR [rdi+r15*1]
  658538:	shl    ecx,0x10
  65853b:	or     rdx,rcx
  65853e:	movzx  ecx,BYTE PTR [rdi+rbx*1]
  658542:	shl    ecx,0x18
  658545:	or     rdx,rcx
  658548:	movzx  ecx,BYTE PTR [rdi+r13*1]
  65854d:	shl    rcx,0x20
  658551:	or     rdx,rcx
  658554:	movzx  eax,BYTE PTR [rdi+rax*1]
  658558:	shl    rax,0x28
  65855c:	or     rdx,rax
  65855f:	movzx  eax,BYTE PTR [rdi+rsi*1]
  658563:	shl    rax,0x30
  658567:	or     rdx,rax
  65856a:	movzx  eax,BYTE PTR [rdi+r9*1]
  65856f:	shl    rax,0x38
  658573:	or     rdx,rax
  658576:	movzx  ecx,BYTE PTR [rdi+r12*1]
  65857b:	shl    ecx,0x8
  65857e:	mov    rax,QWORD PTR [rsp+0xd0]
  658586:	movzx  eax,BYTE PTR [rdi+rax*1]
  65858a:	or     rax,rcx
  65858d:	mov    rcx,QWORD PTR [rsp+0x28]
  658592:	movzx  ecx,BYTE PTR [rdi+rcx*1]
  658596:	shl    ecx,0x10
  658599:	or     rax,rcx
  65859c:	mov    rcx,QWORD PTR [rsp+0x30]
  6585a1:	movzx  ecx,BYTE PTR [rdi+rcx*1]
  6585a5:	shl    ecx,0x18
  6585a8:	or     rax,rcx
  6585ab:	mov    rcx,QWORD PTR [rsp+0x280]
  6585b3:	movzx  ecx,BYTE PTR [rdi+rcx*1]
  6585b7:	shl    rcx,0x20
  6585bb:	or     rax,rcx
  6585be:	mov    rcx,QWORD PTR [rsp+0x1a0]
  6585c6:	movzx  ecx,BYTE PTR [rdi+rcx*1]
  6585ca:	shl    rcx,0x28
  6585ce:	or     rax,rcx
  6585d1:	movzx  ecx,BYTE PTR [rdi+r10*1]
  6585d6:	shl    rcx,0x30
  6585da:	or     rax,rcx
  6585dd:	movzx  ecx,BYTE PTR [rdi+r11*1]
  6585e2:	mov    r11,rdi
  6585e5:	shl    rcx,0x38
  6585e9:	or     rax,rcx
  6585ec:	mov    QWORD PTR [r14+0x8],rdx
  6585f0:	mov    QWORD PTR [r14],rax
  6585f3:	mov    rcx,QWORD PTR [rsp+0x60]
  6585f8:	inc    rcx
  6585fb:	add    r14,0x10
  6585ff:	cmp    rcx,0x8
  658603:	movabs r13,0x7fffffffffffffc0
  65860d:	mov    rdi,QWORD PTR [rsp+0x1c0]
  658615:	mov    rbx,r14
  658618:	jne    6583b0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0xe40>
  65861e:	vmovups ymm0,YMMWORD PTR [rsp+0x1400]
  658627:	vmovups ymm1,YMMWORD PTR [rsp+0x1420]
  658630:	vmovups ymm2,YMMWORD PTR [rsp+0x1440]
  658639:	vmovdqu ymm3,YMMWORD PTR [rsp+0x1460]
  658642:	vmovdqa YMMWORD PTR [rsp+0x8a0],ymm3
  65864b:	vmovaps YMMWORD PTR [rsp+0x880],ymm2
  658654:	vmovaps YMMWORD PTR [rsp+0x860],ymm1
  65865d:	vmovaps YMMWORD PTR [rsp+0x840],ymm0
  658666:	lea    rcx,[rsp+0x2430]
  65866e:	xor    edx,edx
  658670:	mov    r15,QWORD PTR [rsp+0xf0]
  658678:	nop    DWORD PTR [rax+rax*1+0x0]
  658680:	lea    rsi,[rdx+0x1]
  658684:	lea    rax,[rdx+0x2]
  658688:	lea    rdi,[r13+rdx*1+0x40]
  65868d:	and    rdi,rsi
  658690:	shl    rdi,0x4
  658694:	tzcnt  r8,rsi
  658699:	shl    r8d,0x4
  65869d:	vmovaps xmm0,XMMWORD PTR [rsp+r8*1+0x840]
  6586a7:	vxorps xmm0,xmm0,XMMWORD PTR [rsp+rdi*1+0x2400]
  6586b0:	vmovaps XMMWORD PTR [rcx-0x20],xmm0
  6586b5:	and    rsi,rax
  6586b8:	shl    rsi,0x4
  6586bc:	vmovq  xmm0,QWORD PTR [rsp+rsi*1+0x2408]
  6586c5:	add    rdx,0x3
  6586c9:	vmovq  xmm1,QWORD PTR [rsp+rsi*1+0x2400]
  6586d2:	vpunpcklqdq xmm0,xmm1,xmm0
  6586d6:	tzcnt  rsi,rax
  6586db:	shl    esi,0x4
  6586de:	vmovq  xmm1,QWORD PTR [rsp+rsi*1+0x848]
  6586e7:	vmovq  xmm2,QWORD PTR [rsp+rsi*1+0x840]
  6586f0:	vpunpcklqdq xmm1,xmm2,xmm1
  6586f4:	vpxor  xmm0,xmm1,xmm0
  6586f8:	vpextrq QWORD PTR [rcx-0x8],xmm0,0x1
  6586ff:	vmovq  QWORD PTR [rcx-0x10],xmm0
  658704:	and    rax,rdx
  658707:	shl    rax,0x4
  65870b:	vmovq  xmm0,QWORD PTR [rsp+rax*1+0x2408]
  658714:	vmovq  xmm1,QWORD PTR [rsp+rax*1+0x2400]
  65871d:	vpunpcklqdq xmm0,xmm1,xmm0
  658721:	tzcnt  rax,rdx
  658726:	shl    eax,0x4
  658729:	vmovq  xmm1,QWORD PTR [rsp+rax*1+0x848]
  658732:	vmovq  xmm2,QWORD PTR [rsp+rax*1+0x840]
  65873b:	vpunpcklqdq xmm1,xmm2,xmm1
  65873f:	vpxor  xmm0,xmm1,xmm0
  658743:	vmovdqa XMMWORD PTR [rcx],xmm0
  658747:	add    rcx,0x30
  65874b:	cmp    rdx,0xff
  658752:	jne    658680 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1110>
  658758:	vmovups ymm0,YMMWORD PTR [rsp+0x10df]
  658761:	lea    rcx,[rsp+0x1430]
  658769:	vmovups YMMWORD PTR [rcx+0xcf],ymm0
  658771:	vmovups ymm0,YMMWORD PTR [rsp+0x10c0]
  65877a:	vmovups YMMWORD PTR [rcx+0xb0],ymm0
  658782:	vmovups ymm0,YMMWORD PTR [rsp+0x10a0]
  65878b:	vmovups YMMWORD PTR [rcx+0x90],ymm0
  658793:	vmovups ymm0,YMMWORD PTR [rsp+0x1080]
  65879c:	vmovups YMMWORD PTR [rcx+0x70],ymm0
  6587a1:	vmovups ymm0,YMMWORD PTR [rsp+0x1000]
  6587aa:	vmovups ymm1,YMMWORD PTR [rsp+0x1020]
  6587b3:	vmovups ymm2,YMMWORD PTR [rsp+0x1040]
  6587bc:	vmovups ymm3,YMMWORD PTR [rsp+0x1060]
  6587c5:	vmovups YMMWORD PTR [rcx+0x50],ymm3
  6587ca:	vmovups YMMWORD PTR [rcx+0x30],ymm2
  6587cf:	vmovups YMMWORD PTR [rcx+0x10],ymm1
  6587d4:	vmovups YMMWORD PTR [rcx-0x10],ymm0
  6587d9:	lea    rax,[rsp+0x2f8]
  6587e1:	vpxor  xmm4,xmm4,xmm4
  6587e5:	vmovdqu XMMWORD PTR [rax],xmm4
  6587e9:	vmovups ymm0,YMMWORD PTR [rsp+0x51f]
  6587f2:	vmovups YMMWORD PTR [rcx+0x1ef],ymm0
  6587fa:	vmovups ymm0,YMMWORD PTR [rsp+0x500]
  658803:	vmovups YMMWORD PTR [rcx+0x1d0],ymm0
  65880b:	vmovups ymm0,YMMWORD PTR [rsp+0x4e0]
  658814:	vmovups YMMWORD PTR [rcx+0x1b0],ymm0
  65881c:	vmovups ymm0,YMMWORD PTR [rsp+0x4c0]
  658825:	vmovups YMMWORD PTR [rcx+0x190],ymm0
  65882d:	vmovups ymm0,YMMWORD PTR [rsp+0x440]
  658836:	vmovdqu ymm1,YMMWORD PTR [rsp+0x460]
  65883f:	vmovdqu ymm2,YMMWORD PTR [rsp+0x480]
  658848:	vmovdqu ymm3,YMMWORD PTR [rsp+0x4a0]
  658851:	vmovdqu YMMWORD PTR [rcx+0x170],ymm3
  658859:	vmovdqu YMMWORD PTR [rcx+0x150],ymm2
  658861:	vmovdqu YMMWORD PTR [rcx+0x130],ymm1
  658869:	vmovups YMMWORD PTR [rcx+0x110],ymm0
  658871:	movzx  eax,BYTE PTR [rsp+0x307]
  658879:	vmovups xmm0,XMMWORD PTR [rsp+0x2f7]
  658882:	vmovups XMMWORD PTR [rcx+0xef],xmm0
  65888a:	mov    BYTE PTR [rcx+0xff],al
  658890:	lea    rax,[rsp+0x2e7]
  658898:	vmovdqu XMMWORD PTR [rax],xmm4
  65889c:	mov    QWORD PTR [rsp+0x1530],0x0
  6588a8:	mov    QWORD PTR [rsp+0x1538],0xff
  6588b4:	movzx  eax,BYTE PTR [rsp+0x2f6]
  6588bc:	mov    BYTE PTR [rcx+0x21f],al
  6588c2:	vmovdqu xmm0,XMMWORD PTR [rsp+0x2e6]
  6588cb:	vmovdqu XMMWORD PTR [rcx+0x20f],xmm0
  6588d3:	lea    rcx,[rsp+0x1542]
  6588db:	xor    edx,edx
  6588dd:	mov    rbx,QWORD PTR [rsp+0x38]
  6588e2:	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  6588f0:	movzx  edi,BYTE PTR [rcx-0x122]
  6588f7:	movzx  eax,BYTE PTR [rcx-0x2]
  6588fb:	shl    edi,0x8
  6588fe:	or     rdi,rax
  658901:	mov    rsi,QWORD PTR [rsp+0x1c0]
  658909:	cmp    rdi,rsi
  65890c:	jae    65b1fb <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3c8b>
  658912:	movzx  eax,BYTE PTR [r11+rdi*1]
  658917:	shl    eax,0x4
  65891a:	vmovdqa xmm0,XMMWORD PTR [rbx+rdx*1]
  65891f:	vpxor  xmm0,xmm0,XMMWORD PTR [rsp+rax*1+0x2400]
  658928:	vmovdqa XMMWORD PTR [rbx+rdx*1],xmm0
  65892d:	movzx  edi,BYTE PTR [rcx-0x121]
  658934:	movzx  eax,BYTE PTR [rcx-0x1]
  658938:	shl    edi,0x8
  65893b:	or     rdi,rax
  65893e:	cmp    rdi,rsi
  658941:	jae    65b1fb <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3c8b>
  658947:	movzx  eax,BYTE PTR [r11+rdi*1]
  65894c:	shl    eax,0x4
  65894f:	vmovdqa xmm0,XMMWORD PTR [rbx+rdx*1+0x10]
  658955:	vpxor  xmm0,xmm0,XMMWORD PTR [rsp+rax*1+0x2400]
  65895e:	vmovdqa XMMWORD PTR [rbx+rdx*1+0x10],xmm0
  658964:	movzx  edi,BYTE PTR [rcx-0x120]
  65896b:	movzx  eax,BYTE PTR [rcx]
  65896e:	shl    edi,0x8
  658971:	or     rdi,rax
  658974:	cmp    rdi,rsi
  658977:	jae    65b1fb <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3c8b>
  65897d:	movzx  eax,BYTE PTR [r11+rdi*1]
  658982:	shl    eax,0x4
  658985:	vmovdqa xmm0,XMMWORD PTR [rbx+rdx*1+0x20]
  65898b:	vpxor  xmm0,xmm0,XMMWORD PTR [rsp+rax*1+0x2400]
  658994:	vmovdqa XMMWORD PTR [rbx+rdx*1+0x20],xmm0
  65899a:	add    rdx,0x30
  65899e:	add    rcx,0x3
  6589a2:	cmp    rdx,0xff0
  6589a9:	jne    6588f0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1380>
  6589af:	mov    rbx,QWORD PTR [rsp+0x140]
  6589b7:	cmp    rbx,QWORD PTR [rsp+0x18]
  6589bc:	jne    658080 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0xb10>
  6589c2:	mov    edi,0xff0
  6589c7:	vzeroupper 
  6589ca:	call   QWORD PTR [rip+0x9f840]        # 6f8210 <malloc@GLIBC_2.2.5>
  6589d0:	test   rax,rax
  6589d3:	je     65b33d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3dcd>
  6589d9:	mov    r12,rax
  6589dc:	mov    edx,0xff0
  6589e1:	mov    rdi,rax
  6589e4:	mov    rbx,QWORD PTR [rsp+0x38]
  6589e9:	mov    rsi,rbx
  6589ec:	call   QWORD PTR [rip+0x9f83e]        # 6f8230 <memcpy@GLIBC_2.14>
  6589f2:	mov    rdi,rbx
  6589f5:	call   QWORD PTR [rip+0x9f825]        # 6f8220 <free@GLIBC_2.2.5>
  6589fb:	jmp    657862 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2f2>
  658a00:	mov    al,0x2
  658a02:	mov    DWORD PTR [rsp+0x44],eax
  658a06:	jmp    658a18 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x14a8>
  658a08:	mov    al,0x1
  658a0a:	mov    DWORD PTR [rsp+0x44],eax
  658a0e:	jmp    658a18 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x14a8>
  658a10:	mov    DWORD PTR [rsp+0x44],0x0
  658a18:	mov    eax,DWORD PTR [rip+0xa31b2]        # 6fbbd0 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.16073103944965275176+0x1120>
  658a1e:	test   eax,eax
  658a20:	jne    65af0a <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x399a>
  658a26:	mov    eax,DWORD PTR [rip+0xa207c]        # 6faaa8 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.16073103944965275176+0x930>
  658a2c:	test   eax,eax
  658a2e:	jne    65af14 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x39a4>
  658a34:	mov    edx,0x1000
  658a39:	lea    rdi,[rsp+0x1400]
  658a41:	xor    esi,esi
  658a43:	mov    r15,QWORD PTR [rip+0x9f7f6]        # 6f8240 <memset@GLIBC_2.2.5>
  658a4a:	call   r15
  658a4d:	mov    edx,0x400
  658a52:	lea    rdi,[rsp+0x840]
  658a5a:	xor    esi,esi
  658a5c:	call   r15
  658a5f:	mov    edx,0x400
  658a64:	lea    rdi,[rsp+0x1000]
  658a6c:	xor    esi,esi
  658a6e:	call   r15
  658a71:	cmp    r12,r14
  658a74:	jne    658a87 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1517>
  658a76:	xor    eax,eax
  658a78:	cmp    QWORD PTR [rsp+0x130],rbx
  658a80:	jne    658abb <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x154b>
  658a82:	jmp    65a700 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3190>
  658a87:	mov    rax,QWORD PTR [rsp+0x58]
  658a8c:	lea    rcx,[rax+0x3ff]
  658a93:	shr    rcx,0xa
  658a97:	mov    rax,QWORD PTR [rsp+0x18]
  658a9c:	add    rax,0x3ff
  658aa2:	shr    rax,0xa
  658aa6:	cmp    rax,rcx
  658aa9:	cmovae rax,rcx
  658aad:	cmp    QWORD PTR [rsp+0x130],rbx
  658ab5:	je     65a700 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3190>
  658abb:	mov    rcx,QWORD PTR [rsp+0x20]
  658ac0:	add    rcx,0x3f
  658ac4:	shr    rcx,0x6
  658ac8:	cmp    rcx,rax
  658acb:	cmovae rcx,rax
  658acf:	mov    QWORD PTR [rsp+0x250],rcx
  658ad7:	test   rcx,rcx
  658ada:	je     65a700 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3190>
  658ae0:	mov    r14,QWORD PTR [rip+0xa16a1]        # 6fa188 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.16073103944965275176+0x10>
  658ae7:	mov    rax,QWORD PTR [rip+0xa16a2]        # 6fa190 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.16073103944965275176+0x18>
  658aee:	mov    QWORD PTR [rsp+0x48],rax
  658af3:	lea    r12,[r14+0x1000]
  658afa:	lea    rcx,[r14+0x2000]
  658b01:	lea    rsi,[r14+0x3000]
  658b08:	lea    rax,[r14+0x4000]
  658b0f:	mov    QWORD PTR [rsp+0x280],rax
  658b17:	lea    rax,[r14+0x5000]
  658b1e:	mov    QWORD PTR [rsp+0x30],rax
  658b23:	lea    rax,[r14+0x6000]
  658b2a:	mov    QWORD PTR [rsp+0x28],rax
  658b2f:	lea    rax,[r14+0x7000]
  658b36:	mov    QWORD PTR [rsp+0xd0],rax
  658b3e:	lea    rax,[r14+0x8000]
  658b45:	mov    QWORD PTR [rsp+0xc8],rax
  658b4d:	lea    rax,[r14+0x9000]
  658b54:	mov    QWORD PTR [rsp+0xc0],rax
  658b5c:	lea    rax,[r14+0xa000]
  658b63:	mov    QWORD PTR [rsp+0xb8],rax
  658b6b:	mov    r15,QWORD PTR [rip+0xa2f46]        # 6fbab8 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.16073103944965275176+0x1008>
  658b72:	add    r15,QWORD PTR [rip+0xa2f7f]        # 6fbaf8 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.16073103944965275176+0x1048>
  658b79:	lea    rax,[r14+0xb000]
  658b80:	mov    QWORD PTR [rsp+0xb0],rax
  658b88:	lea    rax,[r14+0xc000]
  658b8f:	mov    QWORD PTR [rsp+0xa8],rax
  658b97:	lea    rax,[r14+0xd000]
  658b9e:	mov    QWORD PTR [rsp+0xa0],rax
  658ba6:	lea    rax,[r14+0xe000]
  658bad:	mov    QWORD PTR [rsp+0x98],rax
  658bb5:	lea    rax,[r14+0xf000]
  658bbc:	mov    QWORD PTR [rsp+0x90],rax
  658bc4:	mov    rax,QWORD PTR [rsp+0x168]
  658bcc:	mov    QWORD PTR [rsp+0x198],rax
  658bd4:	mov    rax,QWORD PTR [rsp+0x160]
  658bdc:	mov    QWORD PTR [rsp+0x190],rax
  658be4:	mov    r8,QWORD PTR [rsp+0x138]
  658bec:	mov    r9,QWORD PTR [rsp+0x18]
  658bf1:	mov    r10,QWORD PTR [rsp+0x58]
  658bf6:	mov    r11,QWORD PTR [rsp+0x20]
  658bfb:	xor    edx,edx
  658bfd:	mov    QWORD PTR [rsp+0x60],rcx
  658c02:	mov    QWORD PTR [rsp+0x88],rsi
  658c0a:	mov    QWORD PTR [rsp+0x248],r14
  658c12:	mov    QWORD PTR [rsp+0x240],r12
  658c1a:	jmp    658d61 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x17f1>
  658c1f:	nop
  658c20:	inc    QWORD PTR [rsp+0x110]
  658c28:	lea    rdi,[rsp+0x1400]
  658c30:	lea    rsi,[rsp+0x2400]
  658c38:	mov    rbx,QWORD PTR [rsp+0x38]
  658c3d:	mov    rdx,rbx
  658c40:	lea    rcx,[rsp+0x840]
  658c48:	mov    r8,rbx
  658c4b:	lea    r9,[rsp+0x1000]
  658c53:	push   0x0
  658c55:	push   rbx
  658c56:	vzeroupper 
  658c59:	call   6708e0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled12consume_four>
  658c5e:	add    rsp,0x10
  658c62:	lea    r14,[rsp+0x1400]
  658c6a:	mov    rdi,r14
  658c6d:	lea    r12,[rsp+0x2400]
  658c75:	mov    rsi,r12
  658c78:	mov    rdx,rbx
  658c7b:	lea    rcx,[rsp+0x840]
  658c83:	mov    r8,rbx
  658c86:	lea    r9,[rsp+0x1000]
  658c8e:	push   0x4
  658c90:	push   rbx
  658c91:	call   6708e0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled12consume_four>
  658c96:	add    rsp,0x10
  658c9a:	mov    rdi,r14
  658c9d:	mov    rsi,r12
  658ca0:	mov    rdx,rbx
  658ca3:	lea    rcx,[rsp+0x840]
  658cab:	mov    r8,rbx
  658cae:	lea    r9,[rsp+0x1000]
  658cb6:	push   0x8
  658cb8:	push   rbx
  658cb9:	call   6708e0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled12consume_four>
  658cbe:	add    rsp,0x10
  658cc2:	mov    rdi,r14
  658cc5:	mov    rsi,r12
  658cc8:	mov    rdx,rbx
  658ccb:	lea    rcx,[rsp+0x840]
  658cd3:	mov    r8,rbx
  658cd6:	lea    r9,[rsp+0x1000]
  658cde:	push   0xc
  658ce0:	push   rbx
  658ce1:	call   6708e0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled12consume_four>
  658ce6:	mov    rdx,QWORD PTR [rsp+0x120]
  658cee:	add    rsp,0x10
  658cf2:	mov    r11,QWORD PTR [rsp+0x140]
  658cfa:	add    r11,0xffffffffffffffc0
  658cfe:	mov    r10,QWORD PTR [rsp+0x148]
  658d06:	add    r10,0xfffffffffffffc00
  658d0d:	mov    r9,QWORD PTR [rsp+0x150]
  658d15:	add    r9,0xfffffffffffffc00
  658d1c:	mov    r8,QWORD PTR [rsp+0x118]
  658d24:	add    r8,0x400
  658d2b:	add    QWORD PTR [rsp+0x190],0x400
  658d37:	add    QWORD PTR [rsp+0x198],0x400
  658d43:	cmp    rdx,QWORD PTR [rsp+0x250]
  658d4b:	mov    r14,QWORD PTR [rsp+0x248]
  658d53:	mov    r12,QWORD PTR [rsp+0x240]
  658d5b:	je     65a700 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3190>
  658d61:	cmp    r11,0x40
  658d65:	mov    eax,0x40
  658d6a:	cmovb  rax,r11
  658d6e:	mov    ecx,eax
  658d70:	and    ecx,0x40
  658d73:	shl    ecx,0x4
  658d76:	lea    rsi,[rsp+rcx*1+0x840]
  658d7e:	mov    QWORD PTR [rsp+0x120],rsi
  658d86:	add    rcx,r8
  658d89:	mov    QWORD PTR [rsp+0x188],rcx
  658d91:	and    eax,0x3f
  658d94:	shl    eax,0x4
  658d97:	mov    QWORD PTR [rsp+0x128],rax
  658d9f:	cmp    r11,0x1
  658da3:	mov    rax,r11
  658da6:	adc    rax,0x0
  658daa:	cmp    rax,0x40
  658dae:	mov    ecx,0x40
  658db3:	cmovae rax,rcx
  658db7:	cmp    r10,0x400
  658dbe:	mov    ecx,0x400
  658dc3:	cmovb  rcx,r10
  658dc7:	shr    ecx,0x4
  658dca:	cmp    r9,0x400
  658dd1:	mov    esi,0x400
  658dd6:	cmovb  rsi,r9
  658dda:	shr    esi,0x4
  658ddd:	cmp    rcx,rsi
  658de0:	cmovb  rsi,rcx
  658de4:	mov    QWORD PTR [rsp+0x1a0],rsi
  658dec:	mov    rdi,rdx
  658def:	shl    rdi,0xa
  658df3:	mov    rsi,QWORD PTR [rsp+0x58]
  658df8:	sub    rsi,rdi
  658dfb:	cmp    rsi,0x400
  658e02:	mov    ecx,0x400
  658e07:	cmovae rsi,rcx
  658e0b:	mov    QWORD PTR [rsp+0x260],rsi
  658e13:	mov    rsi,QWORD PTR [rsp+0x18]
  658e18:	sub    rsi,rdi
  658e1b:	cmp    rsi,0x400
  658e22:	cmovae rsi,rcx
  658e26:	mov    QWORD PTR [rsp+0x258],rsi
  658e2e:	mov    QWORD PTR [rsp+0x110],rdx
  658e36:	shl    rdx,0x6
  658e3a:	mov    rsi,QWORD PTR [rsp+0x20]
  658e3f:	sub    rsi,rdx
  658e42:	cmp    rsi,0x40
  658e46:	mov    ebx,0x40
  658e4b:	cmovb  rbx,rsi
  658e4f:	mov    rcx,QWORD PTR [rsp+0x108]
  658e57:	mov    QWORD PTR [rsp+0x1c0],rdi
  658e5f:	add    rcx,rdi
  658e62:	mov    edi,DWORD PTR [rsp+0x44]
  658e66:	cmp    dil,0x1
  658e6a:	mov    QWORD PTR [rsp+0x118],r8
  658e72:	mov    QWORD PTR [rsp+0x150],r9
  658e7a:	mov    QWORD PTR [rsp+0x148],r10
  658e82:	mov    QWORD PTR [rsp+0x140],r11
  658e8a:	mov    QWORD PTR [rsp+0x38],rbx
  658e8f:	je     659080 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1b10>
  658e95:	movzx  edi,dil
  658e99:	test   edi,edi
  658e9b:	jne    6595f0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2080>
  658ea1:	cmp    QWORD PTR [rsp+0x20],rdx
  658ea6:	je     65a2fc <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2d8c>
  658eac:	cmp    QWORD PTR [rsp+0x48],0x10
  658eb2:	jb     65b2e6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3d76>
  658eb8:	mov    rbx,r12
  658ebb:	shl    eax,0x4
  658ebe:	xor    ecx,ecx
  658ec0:	mov    r11,QWORD PTR [rsp+0x118]
  658ec8:	mov    rdx,QWORD PTR [r11+rcx*1]
  658ecc:	mov    r8,rdx
  658ecf:	mov    rdi,rdx
  658ed2:	mov    rsi,rdx
  658ed5:	mov    r10,rdx
  658ed8:	mov    r9d,edx
  658edb:	shr    r9d,0x4
  658edf:	and    r9d,0xff0
  658ee6:	vmovdqa xmm0,XMMWORD PTR [rbx+r9*1]
  658eec:	movzx  r9d,dl
  658ef0:	shl    r9d,0x4
  658ef4:	vpxor  xmm0,xmm0,XMMWORD PTR [r14+r9*1]
  658efa:	mov    r9d,edx
  658efd:	shr    r9d,0xc
  658f01:	and    r9d,0xff0
  658f08:	shr    edx,0x14
  658f0b:	and    edx,0xfffffff0
  658f0e:	mov    r12,QWORD PTR [rsp+0x88]
  658f16:	vmovdqa xmm1,XMMWORD PTR [r12+rdx*1]
  658f1c:	mov    rdx,QWORD PTR [rsp+0x60]
  658f21:	vpternlogq xmm1,xmm0,XMMWORD PTR [rdx+r9*1],0x96
  658f29:	mov    r9,QWORD PTR [r11+rcx*1+0x8]
  658f2e:	mov    rdx,r9
  658f31:	shr    rsi,0x28
  658f35:	movzx  esi,sil
  658f39:	shl    esi,0x4
  658f3c:	mov    r11,QWORD PTR [rsp+0x30]
  658f41:	vmovdqa xmm0,XMMWORD PTR [r11+rsi*1]
  658f47:	mov    rsi,r9
  658f4a:	shr    r10,0x20
  658f4e:	movzx  r10d,r10b
  658f52:	shl    r10d,0x4
  658f56:	mov    r11,QWORD PTR [rsp+0x280]
  658f5e:	vpternlogq xmm0,xmm1,XMMWORD PTR [r11+r10*1],0x96
  658f66:	mov    r10,r9
  658f69:	shr    r8,0x38
  658f6d:	shl    r8d,0x4
  658f71:	mov    r11,QWORD PTR [rsp+0xd0]
  658f79:	vmovdqa xmm1,XMMWORD PTR [r11+r8*1]
  658f7f:	mov    r8,r9
  658f82:	shr    rdi,0x30
  658f86:	movzx  edi,dil
  658f8a:	shl    edi,0x4
  658f8d:	mov    r11,QWORD PTR [rsp+0x28]
  658f92:	vpternlogq xmm1,xmm0,XMMWORD PTR [r11+rdi*1],0x96
  658f9a:	mov    edi,r9d
  658f9d:	shr    edi,0x4
  658fa0:	and    edi,0xff0
  658fa6:	mov    r11,QWORD PTR [rsp+0xc0]
  658fae:	vmovdqa xmm0,XMMWORD PTR [r11+rdi*1]
  658fb4:	movzx  edi,r9b
  658fb8:	shl    edi,0x4
  658fbb:	mov    r11,QWORD PTR [rsp+0xc8]
  658fc3:	vpternlogq xmm0,xmm1,XMMWORD PTR [r11+rdi*1],0x96
  658fcb:	mov    edi,r9d
  658fce:	shr    r9d,0x14
  658fd2:	and    r9d,0xfffffff0
  658fd6:	mov    r11,QWORD PTR [rsp+0xb0]
  658fde:	vmovdqa xmm1,XMMWORD PTR [r11+r9*1]
  658fe4:	shr    edi,0xc
  658fe7:	and    edi,0xff0
  658fed:	mov    r9,QWORD PTR [rsp+0xb8]
  658ff5:	vpternlogq xmm1,xmm0,XMMWORD PTR [r9+rdi*1],0x96
  658ffd:	shr    r10,0x28
  659001:	movzx  edi,r10b
  659005:	shl    edi,0x4
  659008:	mov    r9,QWORD PTR [rsp+0xa0]
  659010:	vmovdqa xmm0,XMMWORD PTR [r9+rdi*1]
  659016:	shr    r8,0x20
  65901a:	movzx  edi,r8b
  65901e:	shl    edi,0x4
  659021:	mov    r8,QWORD PTR [rsp+0xa8]
  659029:	vpternlogq xmm0,xmm1,XMMWORD PTR [r8+rdi*1],0x96
  659031:	shr    rdx,0x38
  659035:	shl    edx,0x4
  659038:	mov    rdi,QWORD PTR [rsp+0x90]
  659040:	vmovdqa xmm1,XMMWORD PTR [rdi+rdx*1]
  659045:	shr    rsi,0x30
  659049:	movzx  edx,sil
  65904d:	shl    edx,0x4
  659050:	mov    rsi,QWORD PTR [rsp+0x98]
  659058:	vpternlogq xmm1,xmm0,XMMWORD PTR [rsi+rdx*1],0x96
  659060:	vmovdqu XMMWORD PTR [rsp+rcx*1+0x840],xmm1
  659069:	add    rcx,0x10
  65906d:	cmp    rax,rcx
  659070:	jne    658ec0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1950>
  659076:	jmp    65a2fc <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2d8c>
  65907b:	nop    DWORD PTR [rax+rax*1+0x0]
  659080:	cmp    rsi,0x40
  659084:	jb     659cbc <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x274c>
  65908a:	vmovdqu64 zmm0,ZMMWORD PTR [rcx]
  659090:	vmovdqu64 zmm1,ZMMWORD PTR [rcx+0x40]
  659097:	vmovdqu64 zmm2,ZMMWORD PTR [rcx+0x80]
  65909e:	vmovdqu64 zmm3,ZMMWORD PTR [rcx+0xc0]
  6590a5:	vmovdqu64 zmm4,ZMMWORD PTR [rcx+0x100]
  6590ac:	vmovdqu64 zmm5,ZMMWORD PTR [rcx+0x140]
  6590b3:	vmovdqu64 zmm6,ZMMWORD PTR [rcx+0x180]
  6590ba:	vmovdqu64 zmm7,ZMMWORD PTR [rcx+0x1c0]
  6590c1:	vmovdqu64 zmm8,ZMMWORD PTR [rcx+0x200]
  6590c8:	vmovdqu64 zmm9,ZMMWORD PTR [rcx+0x240]
  6590cf:	vmovdqu64 zmm10,ZMMWORD PTR [rcx+0x280]
  6590d6:	vmovdqu64 zmm11,ZMMWORD PTR [rcx+0x2c0]
  6590dd:	vmovdqu64 zmm12,ZMMWORD PTR [rcx+0x300]
  6590e4:	vmovdqu64 zmm13,ZMMWORD PTR [rcx+0x340]
  6590eb:	vmovdqu64 zmm14,ZMMWORD PTR [rcx+0x380]
  6590f2:	vmovdqu64 zmm15,ZMMWORD PTR [rcx+0x3c0]
  6590f9:	vpunpcklbw zmm16,zmm0,zmm1
  6590ff:	vpunpckhbw zmm0,zmm0,zmm1
  659105:	vpunpcklbw zmm1,zmm2,zmm3
  65910b:	vpunpckhbw zmm2,zmm2,zmm3
  659111:	vpunpcklbw zmm3,zmm4,zmm5
  659117:	vpunpckhbw zmm4,zmm4,zmm5
  65911d:	vpunpcklbw zmm5,zmm6,zmm7
  659123:	vpunpckhbw zmm6,zmm6,zmm7
  659129:	vpunpcklbw zmm7,zmm8,zmm9
  65912f:	vpunpckhbw zmm8,zmm8,zmm9
  659135:	vpunpcklbw zmm9,zmm10,zmm11
  65913b:	vpunpckhbw zmm10,zmm10,zmm11
  659141:	vpunpcklbw zmm11,zmm12,zmm13
  659147:	vpunpckhbw zmm12,zmm12,zmm13
  65914d:	vpunpcklbw zmm13,zmm14,zmm15
  659153:	vpunpckhbw zmm14,zmm14,zmm15
  659159:	vpunpcklwd zmm15,zmm16,zmm1
  65915f:	vpunpckhwd zmm1,zmm16,zmm1
  659165:	vpunpcklwd zmm16,zmm0,zmm2
  65916b:	vpunpckhwd zmm0,zmm0,zmm2
  659171:	vpunpcklwd zmm2,zmm3,zmm5
  659177:	vpunpckhwd zmm3,zmm3,zmm5
  65917d:	vpunpcklwd zmm5,zmm4,zmm6
  659183:	vpunpckhwd zmm4,zmm4,zmm6
  659189:	vpunpcklwd zmm6,zmm7,zmm9
  65918f:	vpunpckhwd zmm7,zmm7,zmm9
  659195:	vpunpcklwd zmm9,zmm8,zmm10
  65919b:	vpunpckhwd zmm8,zmm8,zmm10
  6591a1:	vpunpcklwd zmm10,zmm11,zmm13
  6591a7:	vpunpckhwd zmm11,zmm11,zmm13
  6591ad:	vpunpcklwd zmm13,zmm12,zmm14
  6591b3:	vpunpckhwd zmm12,zmm12,zmm14
  6591b9:	vpunpckldq zmm14,zmm15,zmm2
  6591bf:	vpunpckhdq zmm2,zmm15,zmm2
  6591c5:	vpunpckldq zmm15,zmm1,zmm3
  6591cb:	vpunpckhdq zmm1,zmm1,zmm3
  6591d1:	vpunpckldq zmm3,zmm16,zmm5
  6591d7:	vpunpckhdq zmm5,zmm16,zmm5
  6591dd:	vpunpckldq zmm16,zmm0,zmm4
  6591e3:	vpunpckhdq zmm0,zmm0,zmm4
  6591e9:	vpunpckldq zmm4,zmm6,zmm10
  6591ef:	vpunpckhdq zmm6,zmm6,zmm10
  6591f5:	vpunpckldq zmm10,zmm7,zmm11
  6591fb:	vpunpckhdq zmm7,zmm7,zmm11
  659201:	vpunpckldq zmm11,zmm9,zmm13
  659207:	vpunpckhdq zmm9,zmm9,zmm13
  65920d:	vpunpckldq zmm13,zmm8,zmm12
  659213:	vpunpckhdq zmm8,zmm8,zmm12
  659219:	vpunpcklqdq zmm12,zmm14,zmm4
  65921f:	vmovdqa64 ZMMWORD PTR [rsp+0xfc0],zmm12
  659227:	vpunpckhqdq zmm4,zmm14,zmm4
  65922d:	vmovdqa64 ZMMWORD PTR [rsp+0xf80],zmm4
  659235:	vpunpcklqdq zmm4,zmm2,zmm6
  65923b:	vmovdqa64 ZMMWORD PTR [rsp+0xf40],zmm4
  659243:	vpunpckhqdq zmm2,zmm2,zmm6
  659249:	vmovdqa64 ZMMWORD PTR [rsp+0xf00],zmm2
  659251:	vpunpcklqdq zmm2,zmm15,zmm10
  659257:	vmovdqa64 ZMMWORD PTR [rsp+0xec0],zmm2
  65925f:	vpunpckhqdq zmm2,zmm15,zmm10
  659265:	vmovdqa64 ZMMWORD PTR [rsp+0xe80],zmm2
  65926d:	vpunpcklqdq zmm2,zmm1,zmm7
  659273:	vmovdqa64 ZMMWORD PTR [rsp+0xe40],zmm2
  65927b:	vpunpckhqdq zmm1,zmm1,zmm7
  659281:	vmovdqa64 ZMMWORD PTR [rsp+0xe00],zmm1
  659289:	vpunpcklqdq zmm1,zmm3,zmm11
  65928f:	vmovdqa64 ZMMWORD PTR [rsp+0xdc0],zmm1
  659297:	vpunpckhqdq zmm1,zmm3,zmm11
  65929d:	vmovdqa64 ZMMWORD PTR [rsp+0xd80],zmm1
  6592a5:	vpunpcklqdq zmm1,zmm5,zmm9
  6592ab:	vmovdqa64 ZMMWORD PTR [rsp+0xd40],zmm1
  6592b3:	vpunpckhqdq zmm1,zmm5,zmm9
  6592b9:	vmovdqa64 ZMMWORD PTR [rsp+0xd00],zmm1
  6592c1:	vpunpcklqdq zmm1,zmm16,zmm13
  6592c7:	vmovdqa64 ZMMWORD PTR [rsp+0xcc0],zmm1
  6592cf:	vpunpckhqdq zmm1,zmm16,zmm13
  6592d5:	vmovdqa64 ZMMWORD PTR [rsp+0xc80],zmm1
  6592dd:	vpunpcklqdq zmm1,zmm0,zmm8
  6592e3:	vmovdqa64 ZMMWORD PTR [rsp+0xc40],zmm1
  6592eb:	vpunpckhqdq zmm0,zmm0,zmm8
  6592f1:	vmovdqa64 ZMMWORD PTR [rsp+0x400],zmm0
  6592f9:	mov    edx,0x400
  6592fe:	lea    rdi,[rsp+0x440]
  659306:	xor    esi,esi
  659308:	vzeroupper 
  65930b:	call   QWORD PTR [rip+0x9ef2f]        # 6f8240 <memset@GLIBC_2.2.5>
  659311:	vmovdqa64 zmm16,ZMMWORD PTR [rsp+0xc40]
  659319:	vmovdqa64 zmm15,ZMMWORD PTR [rsp+0xc80]
  659321:	vmovdqa64 zmm14,ZMMWORD PTR [rsp+0xcc0]
  659329:	vmovdqa64 zmm13,ZMMWORD PTR [rsp+0xd00]
  659331:	vmovdqa64 zmm12,ZMMWORD PTR [rsp+0xd40]
  659339:	vmovdqa64 zmm11,ZMMWORD PTR [rsp+0xd80]
  659341:	vmovdqa64 zmm10,ZMMWORD PTR [rsp+0xdc0]
  659349:	vmovdqa64 zmm9,ZMMWORD PTR [rsp+0xe00]
  659351:	vmovdqa64 zmm8,ZMMWORD PTR [rsp+0xe40]
  659359:	vmovdqa64 zmm7,ZMMWORD PTR [rsp+0xe80]
  659361:	vmovdqa64 zmm6,ZMMWORD PTR [rsp+0xec0]
  659369:	vmovdqa64 zmm5,ZMMWORD PTR [rsp+0xf00]
  659371:	vmovdqa64 zmm4,ZMMWORD PTR [rsp+0xf40]
  659379:	vmovdqa64 zmm3,ZMMWORD PTR [rsp+0xf80]
  659381:	vmovdqa64 zmm2,ZMMWORD PTR [rsp+0xfc0]
  659389:	xor    eax,eax
  65938b:	lea    rdx,[rip+0xa0de6]        # 6fa178 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.16073103944965275176>
  659392:	jmp    6593b8 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1e48>
  659394:	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  6593a0:	vmovdqa64 ZMMWORD PTR [rsp+rax*1+0x440],zmm0
  6593a8:	add    rax,0x40
  6593ac:	cmp    rax,0x400
  6593b2:	je     659a3c <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x24cc>
  6593b8:	mov    rcx,QWORD PTR [rdx+rax*2+0x130]
  6593c0:	test   rcx,rcx
  6593c3:	je     6595d0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2060>
  6593c9:	vpbroadcastq zmm0,rcx
  6593cf:	vgf2p8affineqb zmm0,zmm2,zmm0,0x0
  6593d6:	mov    rcx,QWORD PTR [rdx+rax*2+0x138]
  6593de:	test   rcx,rcx
  6593e1:	je     6593f6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1e86>
  6593e3:	vpbroadcastq zmm1,rcx
  6593e9:	vgf2p8affineqb zmm1,zmm3,zmm1,0x0
  6593f0:	vpxord zmm0,zmm0,zmm1
  6593f6:	mov    rcx,QWORD PTR [rdx+rax*2+0x140]
  6593fe:	test   rcx,rcx
  659401:	je     659416 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1ea6>
  659403:	vpbroadcastq zmm1,rcx
  659409:	vgf2p8affineqb zmm1,zmm4,zmm1,0x0
  659410:	vpxord zmm0,zmm0,zmm1
  659416:	mov    rcx,QWORD PTR [rdx+rax*2+0x148]
  65941e:	test   rcx,rcx
  659421:	je     659436 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1ec6>
  659423:	vpbroadcastq zmm1,rcx
  659429:	vgf2p8affineqb zmm1,zmm5,zmm1,0x0
  659430:	vpxord zmm0,zmm0,zmm1
  659436:	mov    rcx,QWORD PTR [rdx+rax*2+0x150]
  65943e:	test   rcx,rcx
  659441:	je     659456 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1ee6>
  659443:	vpbroadcastq zmm1,rcx
  659449:	vgf2p8affineqb zmm1,zmm6,zmm1,0x0
  659450:	vpxord zmm0,zmm0,zmm1
  659456:	mov    rcx,QWORD PTR [rdx+rax*2+0x158]
  65945e:	test   rcx,rcx
  659461:	je     659476 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1f06>
  659463:	vpbroadcastq zmm1,rcx
  659469:	vgf2p8affineqb zmm1,zmm7,zmm1,0x0
  659470:	vpxord zmm0,zmm0,zmm1
  659476:	mov    rcx,QWORD PTR [rdx+rax*2+0x160]
  65947e:	test   rcx,rcx
  659481:	je     659496 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1f26>
  659483:	vpbroadcastq zmm1,rcx
  659489:	vgf2p8affineqb zmm1,zmm8,zmm1,0x0
  659490:	vpxord zmm0,zmm0,zmm1
  659496:	mov    rcx,QWORD PTR [rdx+rax*2+0x168]
  65949e:	test   rcx,rcx
  6594a1:	je     6594b6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1f46>
  6594a3:	vpbroadcastq zmm1,rcx
  6594a9:	vgf2p8affineqb zmm1,zmm9,zmm1,0x0
  6594b0:	vpxord zmm0,zmm0,zmm1
  6594b6:	mov    rcx,QWORD PTR [rdx+rax*2+0x170]
  6594be:	test   rcx,rcx
  6594c1:	je     6594d6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1f66>
  6594c3:	vpbroadcastq zmm1,rcx
  6594c9:	vgf2p8affineqb zmm1,zmm10,zmm1,0x0
  6594d0:	vpxord zmm0,zmm0,zmm1
  6594d6:	mov    rcx,QWORD PTR [rdx+rax*2+0x178]
  6594de:	test   rcx,rcx
  6594e1:	je     6594f6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1f86>
  6594e3:	vpbroadcastq zmm1,rcx
  6594e9:	vgf2p8affineqb zmm1,zmm11,zmm1,0x0
  6594f0:	vpxord zmm0,zmm0,zmm1
  6594f6:	mov    rcx,QWORD PTR [rdx+rax*2+0x180]
  6594fe:	test   rcx,rcx
  659501:	je     659516 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1fa6>
  659503:	vpbroadcastq zmm1,rcx
  659509:	vgf2p8affineqb zmm1,zmm12,zmm1,0x0
  659510:	vpxord zmm0,zmm0,zmm1
  659516:	mov    rcx,QWORD PTR [rdx+rax*2+0x188]
  65951e:	test   rcx,rcx
  659521:	je     659536 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1fc6>
  659523:	vpbroadcastq zmm1,rcx
  659529:	vgf2p8affineqb zmm1,zmm13,zmm1,0x0
  659530:	vpxord zmm0,zmm0,zmm1
  659536:	mov    rcx,QWORD PTR [rdx+rax*2+0x190]
  65953e:	test   rcx,rcx
  659541:	je     659556 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1fe6>
  659543:	vpbroadcastq zmm1,rcx
  659549:	vgf2p8affineqb zmm1,zmm14,zmm1,0x0
  659550:	vpxord zmm0,zmm0,zmm1
  659556:	mov    rcx,QWORD PTR [rdx+rax*2+0x198]
  65955e:	test   rcx,rcx
  659561:	je     659576 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2006>
  659563:	vpbroadcastq zmm1,rcx
  659569:	vgf2p8affineqb zmm1,zmm15,zmm1,0x0
  659570:	vpxord zmm0,zmm0,zmm1
  659576:	mov    rcx,QWORD PTR [rdx+rax*2+0x1a0]
  65957e:	test   rcx,rcx
  659581:	je     659596 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2026>
  659583:	vpbroadcastq zmm1,rcx
  659589:	vgf2p8affineqb zmm1,zmm16,zmm1,0x0
  659590:	vpxord zmm0,zmm0,zmm1
  659596:	mov    rcx,QWORD PTR [rdx+rax*2+0x1a8]
  65959e:	test   rcx,rcx
  6595a1:	je     6593a0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1e30>
  6595a7:	vpbroadcastq zmm1,rcx
  6595ad:	vmovdqa64 zmm17,ZMMWORD PTR [rsp+0x400]
  6595b5:	vgf2p8affineqb zmm1,zmm17,zmm1,0x0
  6595bc:	vpxord zmm0,zmm0,zmm1
  6595c2:	jmp    6593a0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1e30>
  6595c7:	nop    WORD PTR [rax+rax*1+0x0]
  6595d0:	vpxor  xmm0,xmm0,xmm0
  6595d4:	mov    rcx,QWORD PTR [rdx+rax*2+0x138]
  6595dc:	test   rcx,rcx
  6595df:	jne    6593e3 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1e73>
  6595e5:	jmp    6593f6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x1e86>
  6595ea:	nop    WORD PTR [rax+rax*1+0x0]
  6595f0:	cmp    rsi,0x40
  6595f4:	jb     65a121 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2bb1>
  6595fa:	vmovdqu64 zmm0,ZMMWORD PTR [rcx]
  659600:	vmovdqu64 zmm1,ZMMWORD PTR [rcx+0x40]
  659607:	vmovdqu64 zmm2,ZMMWORD PTR [rcx+0x80]
  65960e:	vmovdqu64 zmm3,ZMMWORD PTR [rcx+0xc0]
  659615:	vmovdqu64 zmm4,ZMMWORD PTR [rcx+0x100]
  65961c:	vmovdqu64 zmm5,ZMMWORD PTR [rcx+0x140]
  659623:	vmovdqu64 zmm6,ZMMWORD PTR [rcx+0x180]
  65962a:	vmovdqu64 zmm7,ZMMWORD PTR [rcx+0x1c0]
  659631:	vmovdqu64 zmm8,ZMMWORD PTR [rcx+0x200]
  659638:	vmovdqu64 zmm9,ZMMWORD PTR [rcx+0x240]
  65963f:	vmovdqu64 zmm10,ZMMWORD PTR [rcx+0x280]
  659646:	vmovdqu64 zmm11,ZMMWORD PTR [rcx+0x2c0]
  65964d:	vmovdqu64 zmm12,ZMMWORD PTR [rcx+0x300]
  659654:	vmovdqu64 zmm13,ZMMWORD PTR [rcx+0x340]
  65965b:	vmovdqu64 zmm14,ZMMWORD PTR [rcx+0x380]
  659662:	vmovdqu64 zmm15,ZMMWORD PTR [rcx+0x3c0]
  659669:	vpunpcklbw zmm16,zmm0,zmm1
  65966f:	vpunpckhbw zmm0,zmm0,zmm1
  659675:	vpunpcklbw zmm1,zmm2,zmm3
  65967b:	vpunpckhbw zmm2,zmm2,zmm3
  659681:	vpunpcklbw zmm3,zmm4,zmm5
  659687:	vpunpckhbw zmm4,zmm4,zmm5
  65968d:	vpunpcklbw zmm5,zmm6,zmm7
  659693:	vpunpckhbw zmm6,zmm6,zmm7
  659699:	vpunpcklbw zmm7,zmm8,zmm9
  65969f:	vpunpckhbw zmm8,zmm8,zmm9
  6596a5:	vpunpcklbw zmm9,zmm10,zmm11
  6596ab:	vpunpckhbw zmm10,zmm10,zmm11
  6596b1:	vpunpcklbw zmm11,zmm12,zmm13
  6596b7:	vpunpckhbw zmm12,zmm12,zmm13
  6596bd:	vpunpcklbw zmm13,zmm14,zmm15
  6596c3:	vpunpckhbw zmm14,zmm14,zmm15
  6596c9:	vpunpcklwd zmm15,zmm16,zmm1
  6596cf:	vpunpckhwd zmm1,zmm16,zmm1
  6596d5:	vpunpcklwd zmm16,zmm0,zmm2
  6596db:	vpunpckhwd zmm0,zmm0,zmm2
  6596e1:	vpunpcklwd zmm2,zmm3,zmm5
  6596e7:	vpunpckhwd zmm3,zmm3,zmm5
  6596ed:	vpunpcklwd zmm5,zmm4,zmm6
  6596f3:	vpunpckhwd zmm4,zmm4,zmm6
  6596f9:	vpunpcklwd zmm6,zmm7,zmm9
  6596ff:	vpunpckhwd zmm7,zmm7,zmm9
  659705:	vpunpcklwd zmm9,zmm8,zmm10
  65970b:	vpunpckhwd zmm8,zmm8,zmm10
  659711:	vpunpcklwd zmm10,zmm11,zmm13
  659717:	vpunpckhwd zmm11,zmm11,zmm13
  65971d:	vpunpcklwd zmm13,zmm12,zmm14
  659723:	vpunpckhwd zmm12,zmm12,zmm14
  659729:	vpunpckldq zmm14,zmm15,zmm2
  65972f:	vpunpckhdq zmm15,zmm15,zmm2
  659735:	vpunpckldq zmm17,zmm1,zmm3
  65973b:	vpunpckhdq zmm18,zmm1,zmm3
  659741:	vpunpckldq zmm19,zmm16,zmm5
  659747:	vpunpckhdq zmm16,zmm16,zmm5
  65974d:	vpunpckldq zmm20,zmm0,zmm4
  659753:	vpunpckhdq zmm21,zmm0,zmm4
  659759:	vpunpckldq zmm1,zmm6,zmm10
  65975f:	vpunpckhdq zmm3,zmm6,zmm10
  659765:	vpunpckldq zmm5,zmm7,zmm11
  65976b:	vpunpckhdq zmm7,zmm7,zmm11
  659771:	vpunpckldq zmm10,zmm9,zmm13
  659777:	vpunpckhdq zmm11,zmm9,zmm13
  65977d:	vpunpckldq zmm13,zmm8,zmm12
  659783:	vpunpckhdq zmm22,zmm8,zmm12
  659789:	vpunpcklqdq zmm0,zmm14,zmm1
  65978f:	vpunpckhqdq zmm1,zmm14,zmm1
  659795:	vpunpcklqdq zmm2,zmm15,zmm3
  65979b:	vpunpckhqdq zmm3,zmm15,zmm3
  6597a1:	vpunpcklqdq zmm4,zmm17,zmm5
  6597a7:	vpunpckhqdq zmm5,zmm17,zmm5
  6597ad:	vpunpcklqdq zmm6,zmm18,zmm7
  6597b3:	vpunpckhqdq zmm7,zmm18,zmm7
  6597b9:	vpunpcklqdq zmm8,zmm19,zmm10
  6597bf:	vpunpckhqdq zmm9,zmm19,zmm10
  6597c5:	vpunpcklqdq zmm10,zmm16,zmm11
  6597cb:	vpunpckhqdq zmm11,zmm16,zmm11
  6597d1:	vpunpcklqdq zmm12,zmm20,zmm13
  6597d7:	vpunpckhqdq zmm13,zmm20,zmm13
  6597dd:	vpunpcklqdq zmm14,zmm21,zmm22
  6597e3:	vpunpckhqdq zmm15,zmm21,zmm22
  6597e9:	xor    eax,eax
  6597eb:	lea    rdx,[rip+0xa0986]        # 6fa178 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.16073103944965275176>
  6597f2:	jmp    659818 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x22a8>
  6597f4:	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  659800:	vmovdqa64 ZMMWORD PTR [rsp+rax*1+0x440],zmm16
  659808:	add    rax,0x40
  65980c:	cmp    rax,0x400
  659812:	je     659ea1 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2931>
  659818:	mov    rcx,QWORD PTR [rdx+rax*2+0x130]
  659820:	test   rcx,rcx
  659823:	je     659a20 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x24b0>
  659829:	vpbroadcastq zmm16,rcx
  65982f:	vgf2p8affineqb zmm16,zmm0,zmm16,0x0
  659836:	mov    rcx,QWORD PTR [rdx+rax*2+0x138]
  65983e:	test   rcx,rcx
  659841:	je     659856 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x22e6>
  659843:	vpbroadcastq zmm17,rcx
  659849:	vgf2p8affineqb zmm17,zmm1,zmm17,0x0
  659850:	vpxord zmm16,zmm16,zmm17
  659856:	mov    rcx,QWORD PTR [rdx+rax*2+0x140]
  65985e:	test   rcx,rcx
  659861:	je     659876 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2306>
  659863:	vpbroadcastq zmm17,rcx
  659869:	vgf2p8affineqb zmm17,zmm2,zmm17,0x0
  659870:	vpxord zmm16,zmm16,zmm17
  659876:	mov    rcx,QWORD PTR [rdx+rax*2+0x148]
  65987e:	test   rcx,rcx
  659881:	je     659896 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2326>
  659883:	vpbroadcastq zmm17,rcx
  659889:	vgf2p8affineqb zmm17,zmm3,zmm17,0x0
  659890:	vpxord zmm16,zmm16,zmm17
  659896:	mov    rcx,QWORD PTR [rdx+rax*2+0x150]
  65989e:	test   rcx,rcx
  6598a1:	je     6598b6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2346>
  6598a3:	vpbroadcastq zmm17,rcx
  6598a9:	vgf2p8affineqb zmm17,zmm4,zmm17,0x0
  6598b0:	vpxord zmm16,zmm16,zmm17
  6598b6:	mov    rcx,QWORD PTR [rdx+rax*2+0x158]
  6598be:	test   rcx,rcx
  6598c1:	je     6598d6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2366>
  6598c3:	vpbroadcastq zmm17,rcx
  6598c9:	vgf2p8affineqb zmm17,zmm5,zmm17,0x0
  6598d0:	vpxord zmm16,zmm16,zmm17
  6598d6:	mov    rcx,QWORD PTR [rdx+rax*2+0x160]
  6598de:	test   rcx,rcx
  6598e1:	je     6598f6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2386>
  6598e3:	vpbroadcastq zmm17,rcx
  6598e9:	vgf2p8affineqb zmm17,zmm6,zmm17,0x0
  6598f0:	vpxord zmm16,zmm16,zmm17
  6598f6:	mov    rcx,QWORD PTR [rdx+rax*2+0x168]
  6598fe:	test   rcx,rcx
  659901:	je     659916 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x23a6>
  659903:	vpbroadcastq zmm17,rcx
  659909:	vgf2p8affineqb zmm17,zmm7,zmm17,0x0
  659910:	vpxord zmm16,zmm16,zmm17
  659916:	mov    rcx,QWORD PTR [rdx+rax*2+0x170]
  65991e:	test   rcx,rcx
  659921:	je     659936 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x23c6>
  659923:	vpbroadcastq zmm17,rcx
  659929:	vgf2p8affineqb zmm17,zmm8,zmm17,0x0
  659930:	vpxord zmm16,zmm16,zmm17
  659936:	mov    rcx,QWORD PTR [rdx+rax*2+0x178]
  65993e:	test   rcx,rcx
  659941:	je     659956 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x23e6>
  659943:	vpbroadcastq zmm17,rcx
  659949:	vgf2p8affineqb zmm17,zmm9,zmm17,0x0
  659950:	vpxord zmm16,zmm16,zmm17
  659956:	mov    rcx,QWORD PTR [rdx+rax*2+0x180]
  65995e:	test   rcx,rcx
  659961:	je     659976 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2406>
  659963:	vpbroadcastq zmm17,rcx
  659969:	vgf2p8affineqb zmm17,zmm10,zmm17,0x0
  659970:	vpxord zmm16,zmm16,zmm17
  659976:	mov    rcx,QWORD PTR [rdx+rax*2+0x188]
  65997e:	test   rcx,rcx
  659981:	je     659996 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2426>
  659983:	vpbroadcastq zmm17,rcx
  659989:	vgf2p8affineqb zmm17,zmm11,zmm17,0x0
  659990:	vpxord zmm16,zmm16,zmm17
  659996:	mov    rcx,QWORD PTR [rdx+rax*2+0x190]
  65999e:	test   rcx,rcx
  6599a1:	je     6599b6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2446>
  6599a3:	vpbroadcastq zmm17,rcx
  6599a9:	vgf2p8affineqb zmm17,zmm12,zmm17,0x0
  6599b0:	vpxord zmm16,zmm16,zmm17
  6599b6:	mov    rcx,QWORD PTR [rdx+rax*2+0x198]
  6599be:	test   rcx,rcx
  6599c1:	je     6599d6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2466>
  6599c3:	vpbroadcastq zmm17,rcx
  6599c9:	vgf2p8affineqb zmm17,zmm13,zmm17,0x0
  6599d0:	vpxord zmm16,zmm16,zmm17
  6599d6:	mov    rcx,QWORD PTR [rdx+rax*2+0x1a0]
  6599de:	test   rcx,rcx
  6599e1:	je     6599f6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2486>
  6599e3:	vpbroadcastq zmm17,rcx
  6599e9:	vgf2p8affineqb zmm17,zmm14,zmm17,0x0
  6599f0:	vpxord zmm16,zmm16,zmm17
  6599f6:	mov    rcx,QWORD PTR [rdx+rax*2+0x1a8]
  6599fe:	test   rcx,rcx
  659a01:	je     659800 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2290>
  659a07:	vpbroadcastq zmm17,rcx
  659a0d:	vgf2p8affineqb zmm17,zmm15,zmm17,0x0
  659a14:	vpxord zmm16,zmm16,zmm17
  659a1a:	jmp    659800 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2290>
  659a1f:	nop
  659a20:	vpxord xmm16,xmm16,xmm16
  659a26:	mov    rcx,QWORD PTR [rdx+rax*2+0x138]
  659a2e:	test   rcx,rcx
  659a31:	jne    659843 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x22d3>
  659a37:	jmp    659856 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x22e6>
  659a3c:	vmovdqa64 zmm0,ZMMWORD PTR [rsp+0x440]
  659a44:	vmovdqa64 zmm1,ZMMWORD PTR [rsp+0x480]
  659a4c:	vmovdqa64 zmm2,ZMMWORD PTR [rsp+0x4c0]
  659a54:	vmovdqa64 zmm3,ZMMWORD PTR [rsp+0x500]
  659a5c:	vmovdqa64 zmm4,ZMMWORD PTR [rsp+0x540]
  659a64:	vmovdqa64 zmm5,ZMMWORD PTR [rsp+0x580]
  659a6c:	vmovdqa64 zmm6,ZMMWORD PTR [rsp+0x5c0]
  659a74:	vmovdqa64 zmm7,ZMMWORD PTR [rsp+0x600]
  659a7c:	vmovdqa64 zmm8,ZMMWORD PTR [rsp+0x640]
  659a84:	vmovdqa64 zmm9,ZMMWORD PTR [rsp+0x680]
  659a8c:	vmovdqa64 zmm10,ZMMWORD PTR [rsp+0x6c0]
  659a94:	vmovdqa64 zmm11,ZMMWORD PTR [rsp+0x700]
  659a9c:	vmovdqa64 zmm12,ZMMWORD PTR [rsp+0x740]
  659aa4:	vmovdqa64 zmm13,ZMMWORD PTR [rsp+0x780]
  659aac:	vmovdqa64 zmm14,ZMMWORD PTR [rsp+0x7c0]
  659ab4:	vmovdqa64 zmm15,ZMMWORD PTR [rsp+0x800]
  659abc:	vpunpcklbw zmm16,zmm0,zmm1
  659ac2:	vpunpckhbw zmm0,zmm0,zmm1
  659ac8:	vpunpcklbw zmm1,zmm2,zmm3
  659ace:	vpunpckhbw zmm2,zmm2,zmm3
  659ad4:	vpunpcklbw zmm3,zmm4,zmm5
  659ada:	vpunpckhbw zmm4,zmm4,zmm5
  659ae0:	vpunpcklbw zmm5,zmm6,zmm7
  659ae6:	vpunpckhbw zmm6,zmm6,zmm7
  659aec:	vpunpcklbw zmm7,zmm8,zmm9
  659af2:	vpunpckhbw zmm8,zmm8,zmm9
  659af8:	vpunpcklbw zmm9,zmm10,zmm11
  659afe:	vpunpckhbw zmm10,zmm10,zmm11
  659b04:	vpunpcklbw zmm11,zmm12,zmm13
  659b0a:	vpunpckhbw zmm12,zmm12,zmm13
  659b10:	vpunpcklbw zmm13,zmm14,zmm15
  659b16:	vpunpckhbw zmm14,zmm14,zmm15
  659b1c:	vpunpcklwd zmm15,zmm16,zmm1
  659b22:	vpunpckhwd zmm1,zmm16,zmm1
  659b28:	vpunpcklwd zmm16,zmm0,zmm2
  659b2e:	vpunpckhwd zmm0,zmm0,zmm2
  659b34:	vpunpcklwd zmm2,zmm3,zmm5
  659b3a:	vpunpckhwd zmm3,zmm3,zmm5
  659b40:	vpunpcklwd zmm5,zmm4,zmm6
  659b46:	vpunpckhwd zmm4,zmm4,zmm6
  659b4c:	vpunpcklwd zmm6,zmm7,zmm9
  659b52:	vpunpckhwd zmm7,zmm7,zmm9
  659b58:	vpunpcklwd zmm9,zmm8,zmm10
  659b5e:	vpunpckhwd zmm8,zmm8,zmm10
  659b64:	vpunpcklwd zmm10,zmm11,zmm13
  659b6a:	vpunpckhwd zmm11,zmm11,zmm13
  659b70:	vpunpcklwd zmm13,zmm12,zmm14
  659b76:	vpunpckhwd zmm12,zmm12,zmm14
  659b7c:	vpunpckldq zmm14,zmm15,zmm2
  659b82:	vpunpckhdq zmm2,zmm15,zmm2
  659b88:	vpunpckldq zmm15,zmm1,zmm3
  659b8e:	vpunpckhdq zmm1,zmm1,zmm3
  659b94:	vpunpckldq zmm3,zmm16,zmm5
  659b9a:	vpunpckhdq zmm5,zmm16,zmm5
  659ba0:	vpunpckldq zmm16,zmm0,zmm4
  659ba6:	vpunpckhdq zmm0,zmm0,zmm4
  659bac:	vpunpckldq zmm4,zmm6,zmm10
  659bb2:	vpunpckhdq zmm6,zmm6,zmm10
  659bb8:	vpunpckldq zmm10,zmm7,zmm11
  659bbe:	vpunpckhdq zmm7,zmm7,zmm11
  659bc4:	vpunpckldq zmm11,zmm9,zmm13
  659bca:	vpunpckhdq zmm9,zmm9,zmm13
  659bd0:	vpunpckldq zmm13,zmm8,zmm12
  659bd6:	vpunpckhdq zmm8,zmm8,zmm12
  659bdc:	vpunpcklqdq zmm12,zmm14,zmm4
  659be2:	vpunpckhqdq zmm4,zmm14,zmm4
  659be8:	vpunpcklqdq zmm14,zmm2,zmm6
  659bee:	vpunpckhqdq zmm2,zmm2,zmm6
  659bf4:	vpunpcklqdq zmm6,zmm15,zmm10
  659bfa:	vpunpckhqdq zmm10,zmm15,zmm10
  659c00:	vpunpcklqdq zmm15,zmm1,zmm7
  659c06:	vpunpckhqdq zmm1,zmm1,zmm7
  659c0c:	vpunpcklqdq zmm7,zmm3,zmm11
  659c12:	vpunpckhqdq zmm3,zmm3,zmm11
  659c18:	vpunpcklqdq zmm11,zmm5,zmm9
  659c1e:	vpunpckhqdq zmm5,zmm5,zmm9
  659c24:	vpunpcklqdq zmm9,zmm16,zmm13
  659c2a:	vpunpckhqdq zmm13,zmm16,zmm13
  659c30:	vpunpcklqdq zmm16,zmm0,zmm8
  659c36:	vpunpckhqdq zmm0,zmm0,zmm8
  659c3c:	vmovdqu64 ZMMWORD PTR [rsp+0x840],zmm12
  659c44:	vmovdqu64 ZMMWORD PTR [rsp+0x880],zmm4
  659c4c:	vmovdqu64 ZMMWORD PTR [rsp+0x8c0],zmm14
  659c54:	vmovdqu64 ZMMWORD PTR [rsp+0x900],zmm2
  659c5c:	vmovdqu64 ZMMWORD PTR [rsp+0x940],zmm6
  659c64:	vmovdqu64 ZMMWORD PTR [rsp+0x980],zmm10
  659c6c:	vmovdqu64 ZMMWORD PTR [rsp+0x9c0],zmm15
  659c74:	vmovdqu64 ZMMWORD PTR [rsp+0xa00],zmm1
  659c7c:	vmovdqu64 ZMMWORD PTR [rsp+0xa40],zmm7
  659c84:	vmovdqu64 ZMMWORD PTR [rsp+0xa80],zmm3
  659c8c:	vmovdqu64 ZMMWORD PTR [rsp+0xac0],zmm11
  659c94:	vmovdqu64 ZMMWORD PTR [rsp+0xb00],zmm5
  659c9c:	vmovdqu64 ZMMWORD PTR [rsp+0xb40],zmm9
  659ca4:	vmovdqu64 ZMMWORD PTR [rsp+0xb80],zmm13
  659cac:	vmovdqu64 ZMMWORD PTR [rsp+0xbc0],zmm16
  659cb4:	vmovdqu64 ZMMWORD PTR [rsp+0xc00],zmm0
  659cbc:	test   bl,0x3f
  659cbf:	je     65a2fc <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2d8c>
  659cc5:	cmp    QWORD PTR [rsp+0x48],0x10
  659ccb:	jb     65b2e6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3d76>
  659cd1:	mov    rbx,r12
  659cd4:	xor    eax,eax
  659cd6:	mov    r11,QWORD PTR [rsp+0x188]
  659cde:	mov    r10,QWORD PTR [rsp+0x88]
  659ce6:	cs nop WORD PTR [rax+rax*1+0x0]
  659cf0:	mov    rcx,QWORD PTR [r11+rax*1]
  659cf4:	mov    rdi,rcx
  659cf7:	mov    rsi,rcx
  659cfa:	mov    rdx,rcx
  659cfd:	mov    r9,rcx
  659d00:	mov    r8d,ecx
  659d03:	shr    r8d,0x4
  659d07:	and    r8d,0xff0
  659d0e:	vmovdqa xmm0,XMMWORD PTR [rbx+r8*1]
  659d14:	movzx  r8d,cl
  659d18:	shl    r8d,0x4
  659d1c:	vpxor  xmm0,xmm0,XMMWORD PTR [r14+r8*1]
  659d22:	mov    r8d,ecx
  659d25:	shr    r8d,0xc
  659d29:	and    r8d,0xff0
  659d30:	shr    ecx,0x14
  659d33:	and    ecx,0xfffffff0
  659d36:	vmovdqa xmm1,XMMWORD PTR [r10+rcx*1]
  659d3c:	mov    rcx,QWORD PTR [rsp+0x60]
  659d41:	vpternlogq xmm1,xmm0,XMMWORD PTR [rcx+r8*1],0x96
  659d49:	mov    r8,QWORD PTR [r11+rax*1+0x8]
  659d4e:	mov    rcx,r8
  659d51:	shr    rdx,0x28
  659d55:	movzx  edx,dl
  659d58:	shl    edx,0x4
  659d5b:	mov    r12,QWORD PTR [rsp+0x30]
  659d60:	vmovdqa xmm0,XMMWORD PTR [r12+rdx*1]
  659d66:	mov    rdx,r8
  659d69:	shr    r9,0x20
  659d6d:	movzx  r9d,r9b
  659d71:	shl    r9d,0x4
  659d75:	mov    r12,QWORD PTR [rsp+0x280]
  659d7d:	vpternlogq xmm0,xmm1,XMMWORD PTR [r12+r9*1],0x96
  659d85:	mov    r9,r8
  659d88:	shr    rdi,0x38
  659d8c:	shl    edi,0x4
  659d8f:	mov    r12,QWORD PTR [rsp+0xd0]
  659d97:	vmovdqa xmm1,XMMWORD PTR [r12+rdi*1]
  659d9d:	mov    rdi,r8
  659da0:	shr    rsi,0x30
  659da4:	movzx  esi,sil
  659da8:	shl    esi,0x4
  659dab:	mov    r12,QWORD PTR [rsp+0x28]
  659db0:	vpternlogq xmm1,xmm0,XMMWORD PTR [r12+rsi*1],0x96
  659db8:	mov    esi,r8d
  659dbb:	shr    esi,0x4
  659dbe:	and    esi,0xff0
  659dc4:	mov    r12,QWORD PTR [rsp+0xc0]
  659dcc:	vmovdqa xmm0,XMMWORD PTR [r12+rsi*1]
  659dd2:	movzx  esi,r8b
  659dd6:	shl    esi,0x4
  659dd9:	mov    r12,QWORD PTR [rsp+0xc8]
  659de1:	vpternlogq xmm0,xmm1,XMMWORD PTR [r12+rsi*1],0x96
  659de9:	mov    esi,r8d
  659dec:	shr    r8d,0x14
  659df0:	and    r8d,0xfffffff0
  659df4:	mov    r12,QWORD PTR [rsp+0xb0]
  659dfc:	vmovdqa xmm1,XMMWORD PTR [r12+r8*1]
  659e02:	shr    esi,0xc
  659e05:	and    esi,0xff0
  659e0b:	mov    r8,QWORD PTR [rsp+0xb8]
  659e13:	vpternlogq xmm1,xmm0,XMMWORD PTR [r8+rsi*1],0x96
  659e1b:	shr    r9,0x28
  659e1f:	movzx  esi,r9b
  659e23:	shl    esi,0x4
  659e26:	mov    r8,QWORD PTR [rsp+0xa0]
  659e2e:	vmovdqa xmm0,XMMWORD PTR [r8+rsi*1]
  659e34:	shr    rdi,0x20
  659e38:	movzx  esi,dil
  659e3c:	shl    esi,0x4
  659e3f:	mov    rdi,QWORD PTR [rsp+0xa8]
  659e47:	vpternlogq xmm0,xmm1,XMMWORD PTR [rdi+rsi*1],0x96
  659e4f:	shr    rcx,0x38
  659e53:	shl    ecx,0x4
  659e56:	mov    rsi,QWORD PTR [rsp+0x90]
  659e5e:	vmovdqa xmm1,XMMWORD PTR [rsi+rcx*1]
  659e63:	shr    rdx,0x30
  659e67:	movzx  ecx,dl
  659e6a:	shl    ecx,0x4
  659e6d:	mov    rdx,QWORD PTR [rsp+0x98]
  659e75:	vpternlogq xmm1,xmm0,XMMWORD PTR [rdx+rcx*1],0x96
  659e7d:	mov    rcx,QWORD PTR [rsp+0x120]
  659e85:	vmovdqu XMMWORD PTR [rcx+rax*1],xmm1
  659e8a:	add    rax,0x10
  659e8e:	cmp    QWORD PTR [rsp+0x128],rax
  659e96:	jne    659cf0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2780>
  659e9c:	jmp    65a2fc <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2d8c>
  659ea1:	vmovdqa64 zmm0,ZMMWORD PTR [rsp+0x440]
  659ea9:	vmovdqa64 zmm1,ZMMWORD PTR [rsp+0x480]
  659eb1:	vmovdqa64 zmm2,ZMMWORD PTR [rsp+0x4c0]
  659eb9:	vmovdqa64 zmm3,ZMMWORD PTR [rsp+0x500]
  659ec1:	vmovdqa64 zmm4,ZMMWORD PTR [rsp+0x540]
  659ec9:	vmovdqa64 zmm5,ZMMWORD PTR [rsp+0x580]
  659ed1:	vmovdqa64 zmm6,ZMMWORD PTR [rsp+0x5c0]
  659ed9:	vmovdqa64 zmm7,ZMMWORD PTR [rsp+0x600]
  659ee1:	vmovdqa64 zmm8,ZMMWORD PTR [rsp+0x640]
  659ee9:	vmovdqa64 zmm9,ZMMWORD PTR [rsp+0x680]
  659ef1:	vmovdqa64 zmm10,ZMMWORD PTR [rsp+0x6c0]
  659ef9:	vmovdqa64 zmm11,ZMMWORD PTR [rsp+0x700]
  659f01:	vmovdqa64 zmm12,ZMMWORD PTR [rsp+0x740]
  659f09:	vmovdqa64 zmm13,ZMMWORD PTR [rsp+0x780]
  659f11:	vmovdqa64 zmm14,ZMMWORD PTR [rsp+0x7c0]
  659f19:	vmovdqa64 zmm15,ZMMWORD PTR [rsp+0x800]
  659f21:	vpunpcklbw zmm16,zmm0,zmm1
  659f27:	vpunpckhbw zmm0,zmm0,zmm1
  659f2d:	vpunpcklbw zmm1,zmm2,zmm3
  659f33:	vpunpckhbw zmm2,zmm2,zmm3
  659f39:	vpunpcklbw zmm3,zmm4,zmm5
  659f3f:	vpunpckhbw zmm4,zmm4,zmm5
  659f45:	vpunpcklbw zmm5,zmm6,zmm7
  659f4b:	vpunpckhbw zmm6,zmm6,zmm7
  659f51:	vpunpcklbw zmm7,zmm8,zmm9
  659f57:	vpunpckhbw zmm8,zmm8,zmm9
  659f5d:	vpunpcklbw zmm9,zmm10,zmm11
  659f63:	vpunpckhbw zmm10,zmm10,zmm11
  659f69:	vpunpcklbw zmm11,zmm12,zmm13
  659f6f:	vpunpckhbw zmm12,zmm12,zmm13
  659f75:	vpunpcklbw zmm13,zmm14,zmm15
  659f7b:	vpunpckhbw zmm14,zmm14,zmm15
  659f81:	vpunpcklwd zmm15,zmm16,zmm1
  659f87:	vpunpckhwd zmm1,zmm16,zmm1
  659f8d:	vpunpcklwd zmm16,zmm0,zmm2
  659f93:	vpunpckhwd zmm0,zmm0,zmm2
  659f99:	vpunpcklwd zmm2,zmm3,zmm5
  659f9f:	vpunpckhwd zmm3,zmm3,zmm5
  659fa5:	vpunpcklwd zmm5,zmm4,zmm6
  659fab:	vpunpckhwd zmm4,zmm4,zmm6
  659fb1:	vpunpcklwd zmm6,zmm7,zmm9
  659fb7:	vpunpckhwd zmm7,zmm7,zmm9
  659fbd:	vpunpcklwd zmm9,zmm8,zmm10
  659fc3:	vpunpckhwd zmm8,zmm8,zmm10
  659fc9:	vpunpcklwd zmm10,zmm11,zmm13
  659fcf:	vpunpckhwd zmm11,zmm11,zmm13
  659fd5:	vpunpcklwd zmm13,zmm12,zmm14
  659fdb:	vpunpckhwd zmm12,zmm12,zmm14
  659fe1:	vpunpckldq zmm14,zmm15,zmm2
  659fe7:	vpunpckhdq zmm2,zmm15,zmm2
  659fed:	vpunpckldq zmm15,zmm1,zmm3
  659ff3:	vpunpckhdq zmm1,zmm1,zmm3
  659ff9:	vpunpckldq zmm3,zmm16,zmm5
  659fff:	vpunpckhdq zmm5,zmm16,zmm5
  65a005:	vpunpckldq zmm16,zmm0,zmm4
  65a00b:	vpunpckhdq zmm0,zmm0,zmm4
  65a011:	vpunpckldq zmm4,zmm6,zmm10
  65a017:	vpunpckhdq zmm6,zmm6,zmm10
  65a01d:	vpunpckldq zmm10,zmm7,zmm11
  65a023:	vpunpckhdq zmm7,zmm7,zmm11
  65a029:	vpunpckldq zmm11,zmm9,zmm13
  65a02f:	vpunpckhdq zmm9,zmm9,zmm13
  65a035:	vpunpckldq zmm13,zmm8,zmm12
  65a03b:	vpunpckhdq zmm8,zmm8,zmm12
  65a041:	vpunpcklqdq zmm12,zmm14,zmm4
  65a047:	vpunpckhqdq zmm4,zmm14,zmm4
  65a04d:	vpunpcklqdq zmm14,zmm2,zmm6
  65a053:	vpunpckhqdq zmm2,zmm2,zmm6
  65a059:	vpunpcklqdq zmm6,zmm15,zmm10
  65a05f:	vpunpckhqdq zmm10,zmm15,zmm10
  65a065:	vpunpcklqdq zmm15,zmm1,zmm7
  65a06b:	vpunpckhqdq zmm1,zmm1,zmm7
  65a071:	vpunpcklqdq zmm7,zmm3,zmm11
  65a077:	vpunpckhqdq zmm3,zmm3,zmm11
  65a07d:	vpunpcklqdq zmm11,zmm5,zmm9
  65a083:	vpunpckhqdq zmm5,zmm5,zmm9
  65a089:	vpunpcklqdq zmm9,zmm16,zmm13
  65a08f:	vpunpckhqdq zmm13,zmm16,zmm13
  65a095:	vpunpcklqdq zmm16,zmm0,zmm8
  65a09b:	vpunpckhqdq zmm0,zmm0,zmm8
  65a0a1:	vmovdqu64 ZMMWORD PTR [rsp+0x840],zmm12
  65a0a9:	vmovdqu64 ZMMWORD PTR [rsp+0x880],zmm4
  65a0b1:	vmovdqu64 ZMMWORD PTR [rsp+0x8c0],zmm14
  65a0b9:	vmovdqu64 ZMMWORD PTR [rsp+0x900],zmm2
  65a0c1:	vmovdqu64 ZMMWORD PTR [rsp+0x940],zmm6
  65a0c9:	vmovdqu64 ZMMWORD PTR [rsp+0x980],zmm10
  65a0d1:	vmovdqu64 ZMMWORD PTR [rsp+0x9c0],zmm15
  65a0d9:	vmovdqu64 ZMMWORD PTR [rsp+0xa00],zmm1
  65a0e1:	vmovdqu64 ZMMWORD PTR [rsp+0xa40],zmm7
  65a0e9:	vmovdqu64 ZMMWORD PTR [rsp+0xa80],zmm3
  65a0f1:	vmovdqu64 ZMMWORD PTR [rsp+0xac0],zmm11
  65a0f9:	vmovdqu64 ZMMWORD PTR [rsp+0xb00],zmm5
  65a101:	vmovdqu64 ZMMWORD PTR [rsp+0xb40],zmm9
  65a109:	vmovdqu64 ZMMWORD PTR [rsp+0xb80],zmm13
  65a111:	vmovdqu64 ZMMWORD PTR [rsp+0xbc0],zmm16
  65a119:	vmovdqu64 ZMMWORD PTR [rsp+0xc00],zmm0
  65a121:	test   bl,0x3f
  65a124:	je     65a2fc <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2d8c>
  65a12a:	cmp    QWORD PTR [rsp+0x48],0x10
  65a130:	jb     65b2e6 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3d76>
  65a136:	mov    rbx,r12
  65a139:	xor    eax,eax
  65a13b:	mov    r10,QWORD PTR [rsp+0x188]
  65a143:	mov    r11,QWORD PTR [rsp+0x88]
  65a14b:	nop    DWORD PTR [rax+rax*1+0x0]
  65a150:	mov    rcx,QWORD PTR [r10+rax*1]
  65a154:	mov    rdi,rcx
  65a157:	mov    rsi,rcx
  65a15a:	mov    rdx,rcx
  65a15d:	mov    r9,rcx
  65a160:	mov    r8d,ecx
  65a163:	shr    r8d,0x4
  65a167:	and    r8d,0xff0
  65a16e:	vmovdqa xmm0,XMMWORD PTR [rbx+r8*1]
  65a174:	movzx  r8d,cl
  65a178:	shl    r8d,0x4
  65a17c:	vpxor  xmm0,xmm0,XMMWORD PTR [r14+r8*1]
  65a182:	mov    r8d,ecx
  65a185:	shr    r8d,0xc
  65a189:	and    r8d,0xff0
  65a190:	shr    ecx,0x14
  65a193:	and    ecx,0xfffffff0
  65a196:	vmovdqa xmm1,XMMWORD PTR [r11+rcx*1]
  65a19c:	mov    rcx,QWORD PTR [rsp+0x60]
  65a1a1:	vpternlogq xmm1,xmm0,XMMWORD PTR [rcx+r8*1],0x96
  65a1a9:	mov    r8,QWORD PTR [r10+rax*1+0x8]
  65a1ae:	mov    rcx,r8
  65a1b1:	shr    rdx,0x28
  65a1b5:	movzx  edx,dl
  65a1b8:	shl    edx,0x4
  65a1bb:	mov    r12,QWORD PTR [rsp+0x30]
  65a1c0:	vmovdqa xmm0,XMMWORD PTR [r12+rdx*1]
  65a1c6:	mov    rdx,r8
  65a1c9:	shr    r9,0x20
  65a1cd:	movzx  r9d,r9b
  65a1d1:	shl    r9d,0x4
  65a1d5:	mov    r12,QWORD PTR [rsp+0x280]
  65a1dd:	vpternlogq xmm0,xmm1,XMMWORD PTR [r12+r9*1],0x96
  65a1e5:	mov    r9,r8
  65a1e8:	shr    rdi,0x38
  65a1ec:	shl    edi,0x4
  65a1ef:	mov    r12,QWORD PTR [rsp+0xd0]
  65a1f7:	vmovdqa xmm1,XMMWORD PTR [r12+rdi*1]
  65a1fd:	mov    rdi,r8
  65a200:	shr    rsi,0x30
  65a204:	movzx  esi,sil
  65a208:	shl    esi,0x4
  65a20b:	mov    r12,QWORD PTR [rsp+0x28]
  65a210:	vpternlogq xmm1,xmm0,XMMWORD PTR [r12+rsi*1],0x96
  65a218:	mov    esi,r8d
  65a21b:	shr    esi,0x4
  65a21e:	and    esi,0xff0
  65a224:	mov    r12,QWORD PTR [rsp+0xc0]
  65a22c:	vmovdqa xmm0,XMMWORD PTR [r12+rsi*1]
  65a232:	movzx  esi,r8b
  65a236:	shl    esi,0x4
  65a239:	mov    r12,QWORD PTR [rsp+0xc8]
  65a241:	vpternlogq xmm0,xmm1,XMMWORD PTR [r12+rsi*1],0x96
  65a249:	mov    esi,r8d
  65a24c:	shr    r8d,0x14
  65a250:	and    r8d,0xfffffff0
  65a254:	mov    r12,QWORD PTR [rsp+0xb0]
  65a25c:	vmovdqa xmm1,XMMWORD PTR [r12+r8*1]
  65a262:	shr    esi,0xc
  65a265:	and    esi,0xff0
  65a26b:	mov    r8,QWORD PTR [rsp+0xb8]
  65a273:	vpternlogq xmm1,xmm0,XMMWORD PTR [r8+rsi*1],0x96
  65a27b:	shr    r9,0x28
  65a27f:	movzx  esi,r9b
  65a283:	shl    esi,0x4
  65a286:	mov    r8,QWORD PTR [rsp+0xa0]
  65a28e:	vmovdqa xmm0,XMMWORD PTR [r8+rsi*1]
  65a294:	shr    rdi,0x20
  65a298:	movzx  esi,dil
  65a29c:	shl    esi,0x4
  65a29f:	mov    rdi,QWORD PTR [rsp+0xa8]
  65a2a7:	vpternlogq xmm0,xmm1,XMMWORD PTR [rdi+rsi*1],0x96
  65a2af:	shr    rcx,0x38
  65a2b3:	shl    ecx,0x4
  65a2b6:	mov    rsi,QWORD PTR [rsp+0x90]
  65a2be:	vmovdqa xmm1,XMMWORD PTR [rsi+rcx*1]
  65a2c3:	shr    rdx,0x30
  65a2c7:	movzx  ecx,dl
  65a2ca:	shl    ecx,0x4
  65a2cd:	mov    rdx,QWORD PTR [rsp+0x98]
  65a2d5:	vpternlogq xmm1,xmm0,XMMWORD PTR [rdx+rcx*1],0x96
  65a2dd:	mov    rcx,QWORD PTR [rsp+0x120]
  65a2e5:	vmovdqu XMMWORD PTR [rcx+rax*1],xmm1
  65a2ea:	add    rax,0x10
  65a2ee:	cmp    QWORD PTR [rsp+0x128],rax
  65a2f6:	jne    65a150 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2be0>
  65a2fc:	mov    rax,QWORD PTR [rsp+0x260]
  65a304:	shr    rax,0x4
  65a308:	mov    rcx,QWORD PTR [rsp+0x258]
  65a310:	shr    rcx,0x4
  65a314:	cmp    rcx,rax
  65a317:	cmovae rcx,rax
  65a31b:	test   rcx,rcx
  65a31e:	je     658c20 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x16b0>
  65a324:	mov    rax,QWORD PTR [rsp+0x100]
  65a32c:	mov    rcx,QWORD PTR [rsp+0x1c0]
  65a334:	add    rax,rcx
  65a337:	add    rcx,QWORD PTR [rsp+0xf8]
  65a33f:	mov    QWORD PTR [rsp+0x1c0],rcx
  65a347:	mov    rcx,QWORD PTR [rsp+0x198]
  65a34f:	mov    rdx,QWORD PTR [rsp+0x190]
  65a357:	xor    esi,esi
  65a359:	jmp    65a39f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2e2f>
  65a35b:	nop    DWORD PTR [rax+rax*1+0x0]
  65a360:	shl    rsi,0x7
  65a364:	vgf2p8mulb zmm1,zmm1,zmm3
  65a36a:	vmovdqa64 ZMMWORD PTR [rsp+rsi*1+0x2400],zmm1
  65a375:	vgf2p8mulb zmm0,zmm0,zmm2
  65a37b:	vmovdqa64 ZMMWORD PTR [rsp+rsi*1+0x2440],zmm0
  65a386:	add    rdx,0x10
  65a38a:	add    rcx,0x10
  65a38e:	mov    rsi,rdi
  65a391:	cmp    rdi,QWORD PTR [rsp+0x1a0]
  65a399:	je     658c20 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x16b0>
  65a39f:	lea    rdi,[rsi+0x1]
  65a3a3:	mov    r9,rsi
  65a3a6:	shl    r9,0x4
  65a3aa:	mov    r11,QWORD PTR [rsp+0x1c0]
  65a3b2:	mov    r8,QWORD PTR [r11+r9*1]
  65a3b6:	vmovups xmm0,XMMWORD PTR [rax+r9*1]
  65a3bc:	mov    r10d,DWORD PTR [rax+r9*1]
  65a3c0:	vandps xmm0,xmm0,XMMWORD PTR [r11+r9*1]
  65a3c6:	vmovups XMMWORD PTR [rsp+r9*1+0x1000],xmm0
  65a3d0:	movzx  r9d,r10b
  65a3d4:	shl    r9d,0x7
  65a3d8:	vmovdqu64 zmm1,ZMMWORD PTR [r15+r9*1]
  65a3df:	vmovdqu64 zmm0,ZMMWORD PTR [r15+r9*1+0x40]
  65a3e7:	mov    r9d,0x1
  65a3ed:	mov    r10d,0x8
  65a3f3:	jmp    65a43b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2ecb>
  65a3f5:	vmovdqa64 zmm3,ZMMWORD PTR [rip+0xffffffffff9b6d81]        # 11180 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x132>
  65a3ff:	vpermq zmm2,zmm3,zmm2
  65a405:	xor    r14,0x40
  65a409:	vmovdqu64 zmm3,ZMMWORD PTR [r11+r14*1]
  65a410:	vshufi64x2 zmm3,zmm3,zmm3,0x4e
  65a417:	vpxord zmm1,zmm2,zmm1
  65a41d:	vpshufd zmm2,zmm3,0x4e
  65a424:	vpxord zmm0,zmm2,zmm0
  65a42a:	inc    r9
  65a42d:	add    r10,0x8
  65a431:	cmp    r9,0x10
  65a435:	je     65a560 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2ff0>
  65a43b:	movzx  r11d,BYTE PTR [rdx+r9*1]
  65a440:	shl    r11d,0x7
  65a444:	add    r11,r15
  65a447:	mov    r12d,r9d
  65a44a:	shr    r12d,1
  65a44d:	and    r12d,0x3
  65a451:	mov    r14,r10
  65a454:	and    r14,r13
  65a457:	vmovdqu64 zmm2,ZMMWORD PTR [r14+r11*1]
  65a45e:	test   r9b,0x1
  65a462:	jne    65a490 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2f20>
  65a464:	lea    rbx,[rip+0xffffffffff9b8141]        # 125ac <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x155e>
  65a46b:	movsxd r12,DWORD PTR [rbx+r12*4]
  65a46f:	add    r12,rbx
  65a472:	jmp    r12
  65a475:	xor    r14,0x40
  65a479:	vmovdqu64 zmm3,ZMMWORD PTR [r11+r14*1]
  65a480:	jmp    65a528 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2fb8>
  65a485:	data16 cs nop WORD PTR [rax+rax*1+0x0]
  65a490:	lea    rbx,[rip+0xffffffffff9b8105]        # 1259c <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x154e>
  65a497:	movsxd r12,DWORD PTR [rbx+r12*4]
  65a49b:	add    r12,rbx
  65a49e:	jmp    r12
  65a4a1:	vpshufd zmm2,zmm2,0x4e
  65a4a8:	xor    r14,0x40
  65a4ac:	vmovdqu64 zmm3,ZMMWORD PTR [r11+r14*1]
  65a4b3:	jmp    65a417 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2ea7>
  65a4b8:	vshufi64x2 zmm2,zmm2,zmm2,0x4e
  65a4bf:	xor    r14,0x40
  65a4c3:	vmovdqu64 zmm3,ZMMWORD PTR [r11+r14*1]
  65a4ca:	vshufi64x2 zmm3,zmm3,zmm3,0x4e
  65a4d1:	jmp    65a528 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2fb8>
  65a4d3:	vshufi64x2 zmm2,zmm2,zmm2,0x1b
  65a4da:	xor    r14,0x40
  65a4de:	vmovdqu64 zmm3,ZMMWORD PTR [r11+r14*1]
  65a4e5:	vshufi64x2 zmm3,zmm3,zmm3,0x1b
  65a4ec:	jmp    65a528 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2fb8>
  65a4ee:	vmovdqa64 zmm3,ZMMWORD PTR [rip+0xffffffffff9b6c48]        # 11140 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xf2>
  65a4f8:	vpermq zmm2,zmm3,zmm2
  65a4fe:	xor    r14,0x40
  65a502:	vmovdqu64 zmm3,ZMMWORD PTR [r11+r14*1]
  65a509:	vshufi64x2 zmm3,zmm3,zmm3,0x1b
  65a510:	jmp    65a417 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2ea7>
  65a515:	vpermq zmm2,zmm2,0x4e
  65a51c:	xor    r14,0x40
  65a520:	vpermq zmm3,ZMMWORD PTR [r11+r14*1],0x4e
  65a528:	vpxord zmm1,zmm1,zmm2
  65a52e:	vpxord zmm0,zmm0,zmm3
  65a534:	inc    r9
  65a537:	add    r10,0x8
  65a53b:	cmp    r9,0x10
  65a53f:	jne    65a43b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2ecb>
  65a545:	jmp    65a560 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2ff0>
  65a547:	vpermq zmm2,zmm2,0x1b
  65a54e:	xor    r14,0x40
  65a552:	vpermq zmm3,ZMMWORD PTR [r11+r14*1],0x4e
  65a55a:	jmp    65a417 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2ea7>
  65a55f:	nop
  65a560:	movzx  r8d,r8b
  65a564:	shl    r8d,0x7
  65a568:	vmovdqu64 zmm3,ZMMWORD PTR [r15+r8*1]
  65a56f:	vmovdqu64 zmm2,ZMMWORD PTR [r15+r8*1+0x40]
  65a577:	mov    r8d,0x1
  65a57d:	mov    r9d,0x8
  65a583:	jmp    65a5cb <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x305b>
  65a585:	vmovdqa64 zmm5,ZMMWORD PTR [rip+0xffffffffff9b6bf1]        # 11180 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x132>
  65a58f:	vpermq zmm4,zmm5,zmm4
  65a595:	xor    r11,0x40
  65a599:	vmovdqu64 zmm5,ZMMWORD PTR [r10+r11*1]
  65a5a0:	vshufi64x2 zmm5,zmm5,zmm5,0x4e
  65a5a7:	vpxord zmm3,zmm4,zmm3
  65a5ad:	vpshufd zmm4,zmm5,0x4e
  65a5b4:	vpxord zmm2,zmm4,zmm2
  65a5ba:	inc    r8
  65a5bd:	add    r9,0x8
  65a5c1:	cmp    r8,0x10
  65a5c5:	je     65a360 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2df0>
  65a5cb:	movzx  r10d,BYTE PTR [rcx+r8*1]
  65a5d0:	shl    r10d,0x7
  65a5d4:	add    r10,r15
  65a5d7:	mov    r14d,r8d
  65a5da:	shr    r14d,1
  65a5dd:	and    r14d,0x3
  65a5e1:	mov    r11,r9
  65a5e4:	and    r11,r13
  65a5e7:	vmovdqu64 zmm4,ZMMWORD PTR [r11+r10*1]
  65a5ee:	test   r8b,0x1
  65a5f2:	jne    65a620 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x30b0>
  65a5f4:	lea    rbx,[rip+0xffffffffff9b7fd1]        # 125cc <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x157e>
  65a5fb:	movsxd r14,DWORD PTR [rbx+r14*4]
  65a5ff:	add    r14,rbx
  65a602:	jmp    r14
  65a605:	xor    r11,0x40
  65a609:	vmovdqu64 zmm5,ZMMWORD PTR [r10+r11*1]
  65a610:	jmp    65a6b8 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3148>
  65a615:	data16 cs nop WORD PTR [rax+rax*1+0x0]
  65a620:	lea    rbx,[rip+0xffffffffff9b7f95]        # 125bc <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x156e>
  65a627:	movsxd r14,DWORD PTR [rbx+r14*4]
  65a62b:	add    r14,rbx
  65a62e:	jmp    r14
  65a631:	vpshufd zmm4,zmm4,0x4e
  65a638:	xor    r11,0x40
  65a63c:	vmovdqu64 zmm5,ZMMWORD PTR [r10+r11*1]
  65a643:	jmp    65a5a7 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3037>
  65a648:	vshufi64x2 zmm4,zmm4,zmm4,0x4e
  65a64f:	xor    r11,0x40
  65a653:	vmovdqu64 zmm5,ZMMWORD PTR [r10+r11*1]
  65a65a:	vshufi64x2 zmm5,zmm5,zmm5,0x4e
  65a661:	jmp    65a6b8 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3148>
  65a663:	vshufi64x2 zmm4,zmm4,zmm4,0x1b
  65a66a:	xor    r11,0x40
  65a66e:	vmovdqu64 zmm5,ZMMWORD PTR [r10+r11*1]
  65a675:	vshufi64x2 zmm5,zmm5,zmm5,0x1b
  65a67c:	jmp    65a6b8 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3148>
  65a67e:	vmovdqa64 zmm5,ZMMWORD PTR [rip+0xffffffffff9b6ab8]        # 11140 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0xf2>
  65a688:	vpermq zmm4,zmm5,zmm4
  65a68e:	xor    r11,0x40
  65a692:	vmovdqu64 zmm5,ZMMWORD PTR [r10+r11*1]
  65a699:	vshufi64x2 zmm5,zmm5,zmm5,0x1b
  65a6a0:	jmp    65a5a7 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3037>
  65a6a5:	vpermq zmm4,zmm4,0x4e
  65a6ac:	xor    r11,0x40
  65a6b0:	vpermq zmm5,ZMMWORD PTR [r10+r11*1],0x4e
  65a6b8:	vpxord zmm3,zmm3,zmm4
  65a6be:	vpxord zmm2,zmm2,zmm5
  65a6c4:	inc    r8
  65a6c7:	add    r9,0x8
  65a6cb:	cmp    r8,0x10
  65a6cf:	jne    65a5cb <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x305b>
  65a6d5:	jmp    65a360 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2df0>
  65a6da:	vpermq zmm4,zmm4,0x1b
  65a6e1:	xor    r11,0x40
  65a6e5:	vpermq zmm5,ZMMWORD PTR [r10+r11*1],0x4e
  65a6ed:	jmp    65a5a7 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3037>
  65a6f2:	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  65a700:	mov    edi,0xff0
  65a705:	call   QWORD PTR [rip+0x9db05]        # 6f8210 <malloc@GLIBC_2.2.5>
  65a70b:	test   rax,rax
  65a70e:	je     65b32b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3dbb>
  65a714:	mov    r12,rax
  65a717:	mov    eax,0xf
  65a71c:	lea    rcx,[rsp+0x2300]
  65a724:	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  65a730:	movzx  edx,BYTE PTR [rcx-0xf00]
  65a737:	mov    BYTE PTR [rsp+0x1a0],dl
  65a73e:	movzx  esi,BYTE PTR [rcx-0xe00]
  65a745:	movzx  edi,BYTE PTR [rcx-0xd00]
  65a74c:	movzx  r8d,BYTE PTR [rcx-0xc00]
  65a754:	movzx  r9d,BYTE PTR [rcx-0xb00]
  65a75c:	movzx  r10d,BYTE PTR [rcx-0xa00]
  65a764:	movzx  r11d,BYTE PTR [rcx-0x900]
  65a76c:	movzx  ebx,BYTE PTR [rcx-0x800]
  65a773:	movzx  r14d,BYTE PTR [rcx-0x700]
  65a77b:	movzx  r15d,BYTE PTR [rcx-0x600]
  65a783:	movzx  r13d,BYTE PTR [rcx-0x500]
  65a78b:	movzx  edx,BYTE PTR [rcx-0x400]
  65a792:	mov    BYTE PTR [rsp+0x1c0],dl
  65a799:	movzx  edx,BYTE PTR [rcx-0x300]
  65a7a0:	mov    BYTE PTR [rsp+0x60],dl
  65a7a4:	movzx  edx,BYTE PTR [rcx-0x200]
  65a7ab:	mov    BYTE PTR [rsp+0x280],dl
  65a7b2:	movzx  edx,BYTE PTR [rcx-0x100]
  65a7b9:	mov    BYTE PTR [rsp+0x30],dl
  65a7bd:	movzx  edx,BYTE PTR [rcx]
  65a7c0:	mov    BYTE PTR [rsp+0x28],dl
  65a7c4:	movzx  edx,BYTE PTR [rsp+0x1a0]
  65a7cc:	mov    BYTE PTR [r12+rax*1-0xf],dl
  65a7d1:	mov    BYTE PTR [r12+rax*1-0xe],sil
  65a7d6:	mov    BYTE PTR [r12+rax*1-0xd],dil
  65a7db:	mov    BYTE PTR [r12+rax*1-0xc],r8b
  65a7e0:	mov    BYTE PTR [r12+rax*1-0xb],r9b
  65a7e5:	mov    BYTE PTR [r12+rax*1-0xa],r10b
  65a7ea:	mov    BYTE PTR [r12+rax*1-0x9],r11b
  65a7ef:	mov    BYTE PTR [r12+rax*1-0x8],bl
  65a7f4:	mov    BYTE PTR [r12+rax*1-0x7],r14b
  65a7f9:	mov    BYTE PTR [r12+rax*1-0x6],r15b
  65a7fe:	mov    BYTE PTR [r12+rax*1-0x5],r13b
  65a803:	movzx  edx,BYTE PTR [rsp+0x1c0]
  65a80b:	mov    BYTE PTR [r12+rax*1-0x4],dl
  65a810:	movzx  edx,BYTE PTR [rsp+0x60]
  65a815:	mov    BYTE PTR [r12+rax*1-0x3],dl
  65a81a:	movzx  edx,BYTE PTR [rsp+0x280]
  65a822:	mov    BYTE PTR [r12+rax*1-0x2],dl
  65a827:	movzx  edx,BYTE PTR [rsp+0x30]
  65a82c:	mov    BYTE PTR [r12+rax*1-0x1],dl
  65a831:	movzx  edx,BYTE PTR [rsp+0x28]
  65a836:	mov    BYTE PTR [r12+rax*1],dl
  65a83a:	add    rax,0x10
  65a83e:	inc    rcx
  65a841:	cmp    rax,0xfff
  65a847:	jne    65a730 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x31c0>
  65a84d:	jmp    657850 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2e0>
  65a852:	mov    eax,DWORD PTR [rip+0xa0250]        # 6faaa8 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.16073103944965275176+0x930>
  65a858:	test   eax,eax
  65a85a:	jne    65af1e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x39ae>
  65a860:	mov    eax,DWORD PTR [rip+0xa136a]        # 6fbbd0 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.16073103944965275176+0x1120>
  65a866:	test   eax,eax
  65a868:	jne    65af28 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x39b8>
  65a86e:	mov    rax,QWORD PTR [rsp+0x58]
  65a873:	shr    rax,0x4
  65a877:	mov    rbx,QWORD PTR [rsp+0x18]
  65a87c:	shr    rbx,0x4
  65a880:	cmp    rbx,rax
  65a883:	cmovae rbx,rax
  65a887:	mov    rax,QWORD PTR [rsp+0x20]
  65a88c:	cmp    rax,rbx
  65a88f:	cmovb  rbx,rax
  65a893:	mov    edx,0x1000
  65a898:	lea    rdi,[rsp+0x2400]
  65a8a0:	xor    esi,esi
  65a8a2:	call   QWORD PTR [rip+0x9d998]        # 6f8240 <memset@GLIBC_2.2.5>
  65a8a8:	mov    QWORD PTR [rsp+0x18],rbx
  65a8ad:	test   rbx,rbx
  65a8b0:	lea    r13,[rsp+0x1400]
  65a8b8:	je     65ad90 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3820>
  65a8be:	xor    ebx,ebx
  65a8c0:	mov    r14,rbx
  65a8c3:	shl    r14,0x4
  65a8c7:	mov    rax,QWORD PTR [rsp+0x100]
  65a8cf:	lea    r12,[rax+r14*1]
  65a8d3:	mov    rdi,r13
  65a8d6:	mov    rsi,r12
  65a8d9:	vzeroupper 
  65a8dc:	call   667860 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate6domainNtB2_6Domain15eval_row_native>
  65a8e1:	vmovups zmm0,ZMMWORD PTR [rsp+0x1480]
  65a8e9:	vmovaps ZMMWORD PTR [rsp+0x1c0],zmm0
  65a8f1:	mov    rax,QWORD PTR [rsp+0xf8]
  65a8f9:	lea    r15,[rax+r14*1]
  65a8fd:	vmovups xmm0,XMMWORD PTR [rsp+0x14e0]
  65a906:	vmovaps YMMWORD PTR [rsp+0x1a0],ymm0
  65a90f:	mov    rax,QWORD PTR [rsp+0x14f0]
  65a917:	mov    ecx,eax
  65a919:	shr    ecx,0x8
  65a91c:	vmovq  xmm0,rax
  65a921:	vpinsrd xmm0,xmm0,DWORD PTR [rsp+0x14f8],0x2
  65a92c:	vpinsrw xmm1,xmm0,WORD PTR [rsp+0x14fc],0x6
  65a936:	vpinsrb xmm2,xmm1,ecx,0x1
  65a93c:	mov    ecx,eax
  65a93e:	shr    ecx,0x10
  65a941:	vpinsrb xmm2,xmm2,ecx,0x2
  65a947:	mov    ecx,eax
  65a949:	shr    ecx,0x18
  65a94c:	vpinsrb xmm2,xmm2,ecx,0x3
  65a952:	mov    rcx,rax
  65a955:	shr    rcx,0x20
  65a959:	vpinsrb xmm2,xmm2,ecx,0x4
  65a95f:	mov    rcx,rax
  65a962:	shr    rcx,0x28
  65a966:	vpinsrb xmm2,xmm2,ecx,0x5
  65a96c:	mov    rcx,rax
  65a96f:	shr    rcx,0x30
  65a973:	vpinsrb xmm2,xmm2,ecx,0x6
  65a979:	shr    rax,0x38
  65a97d:	vpinsrb xmm2,xmm2,eax,0x7
  65a983:	vpblendd xmm0,xmm2,xmm0,0x4
  65a989:	vpblendw xmm0,xmm0,xmm1,0x40
  65a98f:	vpinsrb xmm0,xmm0,BYTE PTR [rsp+0x14fe],0xe
  65a99a:	vmovdqa XMMWORD PTR [rsp+0x60],xmm0
  65a9a0:	vmovdqu ymm0,YMMWORD PTR [rsp+0x14c0]
  65a9a9:	vmovdqa64 ZMMWORD PTR [rsp+0x280],zmm0
  65a9b1:	mov    rdi,r13
  65a9b4:	mov    rsi,r15
  65a9b7:	vzeroupper 
  65a9ba:	call   667860 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate6domainNtB2_6Domain15eval_row_native>
  65a9bf:	mov    rdi,QWORD PTR [rip+0x9f7ca]        # 6fa190 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.16073103944965275176+0x18>
  65a9c6:	cmp    rdi,0x10
  65a9ca:	jb     65b2eb <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3d7b>
  65a9d0:	add    r14,QWORD PTR [rsp+0x108]
  65a9d8:	mov    rcx,QWORD PTR [r14]
  65a9db:	mov    rax,QWORD PTR [rip+0x9f7a6]        # 6fa188 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.16073103944965275176+0x10>
  65a9e2:	mov    rdi,rcx
  65a9e5:	mov    rsi,rcx
  65a9e8:	mov    rdx,rcx
  65a9eb:	mov    r9,rcx
  65a9ee:	mov    r8d,ecx
  65a9f1:	shr    r8d,0x4
  65a9f5:	and    r8d,0xff0
  65a9fc:	vmovdqa xmm0,XMMWORD PTR [rax+r8*1+0x1000]
  65aa06:	movzx  r8d,cl
  65aa0a:	shl    r8d,0x4
  65aa0e:	vpxor  xmm0,xmm0,XMMWORD PTR [rax+r8*1]
  65aa14:	mov    r8d,ecx
  65aa17:	shr    ecx,0x14
  65aa1a:	and    ecx,0xfffffff0
  65aa1d:	vmovdqa xmm1,XMMWORD PTR [rax+rcx*1+0x3000]
  65aa26:	shr    r8d,0xc
  65aa2a:	and    r8d,0xff0
  65aa31:	vpternlogq xmm1,xmm0,XMMWORD PTR [rax+r8*1+0x2000],0x96
  65aa3d:	mov    r8,QWORD PTR [r14+0x8]
  65aa41:	mov    rcx,r8
  65aa44:	shr    rdx,0x28
  65aa48:	movzx  edx,dl
  65aa4b:	shl    edx,0x4
  65aa4e:	vmovdqa xmm0,XMMWORD PTR [rax+rdx*1+0x5000]
  65aa57:	mov    rdx,r8
  65aa5a:	shr    r9,0x20
  65aa5e:	movzx  r9d,r9b
  65aa62:	shl    r9d,0x4
  65aa66:	vpternlogq xmm0,xmm1,XMMWORD PTR [rax+r9*1+0x4000],0x96
  65aa72:	mov    r9,r8
  65aa75:	shr    rdi,0x38
  65aa79:	shl    edi,0x4
  65aa7c:	vmovdqa xmm1,XMMWORD PTR [rax+rdi*1+0x7000]
  65aa85:	mov    rdi,r8
  65aa88:	shr    rsi,0x30
  65aa8c:	movzx  esi,sil
  65aa90:	shl    esi,0x4
  65aa93:	vpternlogq xmm1,xmm0,XMMWORD PTR [rax+rsi*1+0x6000],0x96
  65aa9f:	mov    esi,r8d
  65aaa2:	shr    esi,0x4
  65aaa5:	and    esi,0xff0
  65aaab:	vmovdqa xmm0,XMMWORD PTR [rax+rsi*1+0x9000]
  65aab4:	movzx  esi,r8b
  65aab8:	shl    esi,0x4
  65aabb:	vpternlogq xmm0,xmm1,XMMWORD PTR [rax+rsi*1+0x8000],0x96
  65aac7:	mov    esi,r8d
  65aaca:	shr    r8d,0x14
  65aace:	and    r8d,0xfffffff0
  65aad2:	vmovdqa xmm1,XMMWORD PTR [rax+r8*1+0xb000]
  65aadc:	shr    esi,0xc
  65aadf:	and    esi,0xff0
  65aae5:	vpternlogq xmm1,xmm0,XMMWORD PTR [rax+rsi*1+0xa000],0x96
  65aaf1:	shr    r9,0x28
  65aaf5:	movzx  esi,r9b
  65aaf9:	shl    esi,0x4
  65aafc:	vmovdqa xmm0,XMMWORD PTR [rax+rsi*1+0xd000]
  65ab05:	shr    rdi,0x20
  65ab09:	movzx  esi,dil
  65ab0d:	shl    esi,0x4
  65ab10:	vpternlogq xmm0,xmm1,XMMWORD PTR [rax+rsi*1+0xc000],0x96
  65ab1c:	shr    rcx,0x38
  65ab20:	shl    ecx,0x4
  65ab23:	vmovdqa xmm1,XMMWORD PTR [rax+rcx*1+0xf000]
  65ab2c:	shr    rdx,0x30
  65ab30:	movzx  ecx,dl
  65ab33:	shl    ecx,0x4
  65ab36:	vpternlogq xmm1,xmm0,XMMWORD PTR [rax+rcx*1+0xe000],0x96
  65ab42:	mov    rax,QWORD PTR [rsp+0x14f0]
  65ab4a:	mov    ecx,eax
  65ab4c:	shr    ecx,0x8
  65ab4f:	vmovq  xmm0,rax
  65ab54:	vpinsrd xmm0,xmm0,DWORD PTR [rsp+0x14f8],0x2
  65ab5f:	vpinsrw xmm2,xmm0,WORD PTR [rsp+0x14fc],0x6
  65ab69:	vpinsrb xmm3,xmm2,ecx,0x1
  65ab6f:	mov    ecx,eax
  65ab71:	shr    ecx,0x10
  65ab74:	vpinsrb xmm3,xmm3,ecx,0x2
  65ab7a:	mov    ecx,eax
  65ab7c:	shr    ecx,0x18
  65ab7f:	vpinsrb xmm3,xmm3,ecx,0x3
  65ab85:	mov    rcx,rax
  65ab88:	shr    rcx,0x20
  65ab8c:	vpinsrb xmm3,xmm3,ecx,0x4
  65ab92:	mov    rcx,rax
  65ab95:	shr    rcx,0x28
  65ab99:	vpinsrb xmm3,xmm3,ecx,0x5
  65ab9f:	mov    rcx,rax
  65aba2:	shr    rcx,0x30
  65aba6:	vpinsrb xmm3,xmm3,ecx,0x6
  65abac:	vmovdqu64 zmm4,ZMMWORD PTR [rsp+0x1480]
  65abb4:	shr    rax,0x38
  65abb8:	vpinsrb xmm3,xmm3,eax,0x7
  65abbe:	vpblendd xmm0,xmm3,xmm0,0x4
  65abc4:	vmovdqu xmm3,XMMWORD PTR [rsp+0x14e0]
  65abcd:	vpblendw xmm0,xmm0,xmm2,0x40
  65abd3:	vpinsrb xmm2,xmm0,BYTE PTR [rsp+0x14fe],0xe
  65abde:	vmovdqu ymm5,YMMWORD PTR [rsp+0x14c0]
  65abe7:	vmovdqu XMMWORD PTR [rsp+0x1420],xmm1
  65abf0:	mov    rax,QWORD PTR [r15]
  65abf3:	mov    rcx,QWORD PTR [r15+0x8]
  65abf7:	and    rax,QWORD PTR [r12]
  65abfb:	and    rcx,QWORD PTR [r12+0x8]
  65ac00:	vmovdqa ymm0,YMMWORD PTR [rsp+0x1a0]
  65ac09:	vinserti128 ymm0,ymm0,XMMWORD PTR [rsp+0x60],0x1
  65ac11:	vmovdqa64 zmm1,ZMMWORD PTR [rsp+0x280]
  65ac19:	vinserti64x4 zmm1,zmm1,ymm0,0x1
  65ac20:	inc    rbx
  65ac23:	vgf2p8mulb zmm0,zmm4,ZMMWORD PTR [rsp+0x1c0]
  65ac2b:	vinserti128 ymm2,ymm3,xmm2,0x1
  65ac31:	vinserti64x4 zmm2,zmm5,ymm2,0x1
  65ac38:	vmovdqa64 zmm3,ZMMWORD PTR [rip+0xffffffffff9b657e]        # 111c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x172>
  65ac42:	vpandq zmm2,zmm2,zmm3
  65ac48:	vpandq zmm1,zmm1,zmm3
  65ac4e:	vgf2p8mulb zmm1,zmm1,zmm2
  65ac54:	lea    rdx,[rsp+0x1430]
  65ac5c:	vpxor  xmm2,xmm2,xmm2
  65ac60:	vmovdqu XMMWORD PTR [rdx],xmm2
  65ac64:	kmovq  k1,rax
  65ac69:	kmovq  k2,rcx
  65ac6e:	lea    rax,[rsp+0x1421]
  65ac76:	xor    ecx,ecx
  65ac78:	nop    DWORD PTR [rax+rax*1+0x0]
  65ac80:	vpbroadcastb zmm2,BYTE PTR [rax-0x1]
  65ac87:	vmovdqu8 zmm3{k1}{z},zmm2
  65ac8d:	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x2400]
  65ac98:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x2400],zmm3
  65aca3:	vmovdqu8 zmm3{k2}{z},zmm2
  65aca9:	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x2440]
  65acb4:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x2440],zmm3
  65acbf:	vgf2p8mulb zmm3,zmm0,zmm2
  65acc5:	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x2480]
  65acd0:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x2480],zmm3
  65acdb:	vgf2p8mulb zmm2,zmm1,zmm2
  65ace1:	vpxord zmm2,zmm2,ZMMWORD PTR [rsp+rcx*1+0x24c0]
  65acec:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x24c0],zmm2
  65acf7:	vpbroadcastb zmm2,BYTE PTR [rax]
  65acfd:	vmovdqu8 zmm3{k1}{z},zmm2
  65ad03:	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x2500]
  65ad0e:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x2500],zmm3
  65ad19:	vmovdqu8 zmm3{k2}{z},zmm2
  65ad1f:	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x2540]
  65ad2a:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x2540],zmm3
  65ad35:	vgf2p8mulb zmm3,zmm0,zmm2
  65ad3b:	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x2580]
  65ad46:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x2580],zmm3
  65ad51:	vgf2p8mulb zmm2,zmm1,zmm2
  65ad57:	vpxord zmm2,zmm2,ZMMWORD PTR [rsp+rcx*1+0x25c0]
  65ad62:	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x25c0],zmm2
  65ad6d:	add    rcx,0x200
  65ad74:	add    rax,0x2
  65ad78:	cmp    rcx,0x1000
  65ad7f:	jne    65ac80 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3710>
  65ad85:	cmp    rbx,QWORD PTR [rsp+0x18]
  65ad8a:	jne    65a8c0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3350>
  65ad90:	mov    edi,0xff0
  65ad95:	vzeroupper 
  65ad98:	call   QWORD PTR [rip+0x9d472]        # 6f8210 <malloc@GLIBC_2.2.5>
  65ad9e:	test   rax,rax
  65ada1:	je     65b32b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3dbb>
  65ada7:	mov    r12,rax
  65adaa:	mov    eax,0xf
  65adaf:	lea    rcx,[rsp+0x3300]
  65adb7:	nop    WORD PTR [rax+rax*1+0x0]
  65adc0:	movzx  edx,BYTE PTR [rcx-0xf00]
  65adc7:	mov    BYTE PTR [rsp+0x1a0],dl
  65adce:	movzx  esi,BYTE PTR [rcx-0xe00]
  65add5:	movzx  edi,BYTE PTR [rcx-0xd00]
  65addc:	movzx  r8d,BYTE PTR [rcx-0xc00]
  65ade4:	movzx  r9d,BYTE PTR [rcx-0xb00]
  65adec:	movzx  r10d,BYTE PTR [rcx-0xa00]
  65adf4:	movzx  r11d,BYTE PTR [rcx-0x900]
  65adfc:	movzx  ebx,BYTE PTR [rcx-0x800]
  65ae03:	movzx  r14d,BYTE PTR [rcx-0x700]
  65ae0b:	movzx  r15d,BYTE PTR [rcx-0x600]
  65ae13:	movzx  r13d,BYTE PTR [rcx-0x500]
  65ae1b:	movzx  edx,BYTE PTR [rcx-0x400]
  65ae22:	mov    BYTE PTR [rsp+0x1c0],dl
  65ae29:	movzx  edx,BYTE PTR [rcx-0x300]
  65ae30:	mov    BYTE PTR [rsp+0x60],dl
  65ae34:	movzx  edx,BYTE PTR [rcx-0x200]
  65ae3b:	mov    BYTE PTR [rsp+0x280],dl
  65ae42:	movzx  edx,BYTE PTR [rcx-0x100]
  65ae49:	mov    BYTE PTR [rsp+0x30],dl
  65ae4d:	movzx  edx,BYTE PTR [rcx]
  65ae50:	mov    BYTE PTR [rsp+0x28],dl
  65ae54:	movzx  edx,BYTE PTR [rsp+0x1a0]
  65ae5c:	mov    BYTE PTR [r12+rax*1-0xf],dl
  65ae61:	mov    BYTE PTR [r12+rax*1-0xe],sil
  65ae66:	mov    BYTE PTR [r12+rax*1-0xd],dil
  65ae6b:	mov    BYTE PTR [r12+rax*1-0xc],r8b
  65ae70:	mov    BYTE PTR [r12+rax*1-0xb],r9b
  65ae75:	mov    BYTE PTR [r12+rax*1-0xa],r10b
  65ae7a:	mov    BYTE PTR [r12+rax*1-0x9],r11b
  65ae7f:	mov    BYTE PTR [r12+rax*1-0x8],bl
  65ae84:	mov    BYTE PTR [r12+rax*1-0x7],r14b
  65ae89:	mov    BYTE PTR [r12+rax*1-0x6],r15b
  65ae8e:	mov    BYTE PTR [r12+rax*1-0x5],r13b
  65ae93:	movzx  edx,BYTE PTR [rsp+0x1c0]
  65ae9b:	mov    BYTE PTR [r12+rax*1-0x4],dl
  65aea0:	movzx  edx,BYTE PTR [rsp+0x60]
  65aea5:	mov    BYTE PTR [r12+rax*1-0x3],dl
  65aeaa:	movzx  edx,BYTE PTR [rsp+0x280]
  65aeb2:	mov    BYTE PTR [r12+rax*1-0x2],dl
  65aeb7:	movzx  edx,BYTE PTR [rsp+0x30]
  65aebc:	mov    BYTE PTR [r12+rax*1-0x1],dl
  65aec1:	movzx  edx,BYTE PTR [rsp+0x28]
  65aec6:	mov    BYTE PTR [r12+rax*1],dl
  65aeca:	add    rax,0x10
  65aece:	inc    rcx
  65aed1:	cmp    rax,0xfff
  65aed7:	jne    65adc0 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3850>
  65aedd:	jmp    657850 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x2e0>
  65aee2:	call   654734 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.16073103944965275176>
  65aee7:	jmp    657a12 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x4a2>
  65aeec:	call   654786 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate6domain6DomainE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.16073103944965275176>
  65aef1:	jmp    657a20 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x4b0>
  65aef6:	call   654734 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.16073103944965275176>
  65aefb:	jmp    658001 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0xa91>
  65af00:	call   654786 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate6domain6DomainE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.16073103944965275176>
  65af05:	jmp    65800f <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0xa9f>
  65af0a:	call   654786 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate6domain6DomainE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.16073103944965275176>
  65af0f:	jmp    658a26 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x14b6>
  65af14:	call   654734 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.16073103944965275176>
  65af19:	jmp    658a34 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x14c4>
  65af1e:	call   654734 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.16073103944965275176>
  65af23:	jmp    65a860 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x32f0>
  65af28:	call   654786 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate6domain6DomainE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.16073103944965275176>
  65af2d:	jmp    65a86e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x32fe>
  65af32:	mov    eax,0xff
  65af37:	mov    QWORD PTR [rsp+0x80],rax
  65af3f:	mov    QWORD PTR [rsp+0x50],rax
  65af44:	mov    rcx,QWORD PTR [rsp+0xd8]
  65af4c:	mov    rax,QWORD PTR [rsp+0x50]
  65af51:	mov    QWORD PTR [rcx],rax
  65af54:	mov    rax,QWORD PTR [rsp+0xe8]
  65af5c:	mov    QWORD PTR [rcx+0x8],rax
  65af60:	mov    rax,QWORD PTR [rsp+0x80]
  65af68:	mov    QWORD PTR [rcx+0x10],rax
  65af6c:	jmp    65b1ec <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3c7c>
  65af71:	mov    r14,rsi
  65af74:	mov    r12,r9
  65af77:	mov    r15,rcx
  65af7a:	call   QWORD PTR [rip+0x9d660]        # 6f85e0 <_DYNAMIC+0x5c8>
  65af80:	mov    rcx,r15
  65af83:	mov    r9,r12
  65af86:	mov    rsi,r14
  65af89:	mov    rax,QWORD PTR [rax]
  65af8c:	mov    rax,QWORD PTR [rax+0x208]
  65af93:	shr    rcx,1
  65af96:	cmp    rcx,rax
  65af99:	cmovbe rcx,rax
  65af9d:	mov    QWORD PTR [rsp+0x270],rcx
  65afa5:	mov    QWORD PTR [rsp+0x440],rsi
  65afad:	mov    rax,QWORD PTR [r9]
  65afb0:	mov    r11,QWORD PTR [r9+0x8]
  65afb4:	mov    rcx,QWORD PTR [r9+0x10]
  65afb8:	mov    rdx,QWORD PTR [r9+0x18]
  65afbc:	mov    r10,QWORD PTR [r9+0x20]
  65afc0:	mov    rdi,QWORD PTR [r9+0x28]
  65afc4:	mov    r8,rcx
  65afc7:	imul   r8,rsi
  65afcb:	cmp    r11,r8
  65afce:	cmovb  r8,r11
  65afd2:	lea    r14,[rax+r8*1]
  65afd6:	sub    r11,r8
  65afd9:	lea    r15,[rsp+0x268]
  65afe1:	mov    QWORD PTR [rsp+0x308],r15
  65afe9:	lea    r15,[rsp+0x440]
  65aff1:	mov    QWORD PTR [rsp+0x310],r15
  65aff9:	lea    r12,[rsp+0x270]
  65b001:	mov    QWORD PTR [rsp+0x318],r12
  65b009:	mov    QWORD PTR [rsp+0x320],r14
  65b011:	mov    r14,rdi
  65b014:	imul   r14,rsi
  65b018:	cmp    r10,r14
  65b01b:	cmovb  r14,r10
  65b01f:	mov    QWORD PTR [rsp+0x328],r11
  65b027:	lea    r11,[rdx+r14*1]
  65b02b:	mov    QWORD PTR [rsp+0x330],rcx
  65b033:	mov    QWORD PTR [rsp+0x338],r11
  65b03b:	mov    r11,QWORD PTR [r9+0x30]
  65b03f:	sub    r10,r14
  65b042:	mov    QWORD PTR [rsp+0x340],r10
  65b04a:	mov    r10,QWORD PTR [r9+0x38]
  65b04e:	mov    r9,QWORD PTR [r9+0x40]
  65b052:	imul   rsi,r9
  65b056:	cmp    r10,rsi
  65b059:	cmovb  rsi,r10
  65b05d:	mov    r13,rsi
  65b060:	shl    r13,0x4
  65b064:	add    r13,r11
  65b067:	mov    QWORD PTR [rsp+0x348],rdi
  65b06f:	mov    QWORD PTR [rsp+0x350],r13
  65b077:	sub    r10,rsi
  65b07a:	mov    QWORD PTR [rsp+0x358],r10
  65b082:	mov    r10,QWORD PTR [rbx]
  65b085:	mov    r13,QWORD PTR [rbx+0x8]
  65b089:	mov    rbx,QWORD PTR [rbx+0x10]
  65b08d:	mov    QWORD PTR [rsp+0x360],r9
  65b095:	mov    QWORD PTR [rsp+0x368],r10
  65b09d:	mov    QWORD PTR [rsp+0x370],r13
  65b0a5:	mov    QWORD PTR [rsp+0x378],rbx
  65b0ad:	mov    QWORD PTR [rsp+0x380],r15
  65b0b5:	mov    QWORD PTR [rsp+0x388],r12
  65b0bd:	mov    QWORD PTR [rsp+0x390],rax
  65b0c5:	mov    QWORD PTR [rsp+0x398],r8
  65b0cd:	mov    QWORD PTR [rsp+0x3a0],rcx
  65b0d5:	mov    QWORD PTR [rsp+0x3a8],rdx
  65b0dd:	mov    QWORD PTR [rsp+0x3b0],r14
  65b0e5:	mov    QWORD PTR [rsp+0x3b8],rdi
  65b0ed:	mov    QWORD PTR [rsp+0x3c0],r11
  65b0f5:	mov    QWORD PTR [rsp+0x3c8],rsi
  65b0fd:	mov    QWORD PTR [rsp+0x3d0],r9
  65b105:	mov    QWORD PTR [rsp+0x3d8],r10
  65b10d:	mov    QWORD PTR [rsp+0x3e0],r13
  65b115:	mov    QWORD PTR [rsp+0x3e8],rbx
  65b11d:	mov    rbx,0xffffffffffffffa8
  65b124:	mov    rdx,QWORD PTR fs:[rbx]
  65b128:	test   rdx,rdx
  65b12b:	je     65b14e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3bde>
  65b12d:	lea    rdi,[rsp+0x1400]
  65b135:	lea    rsi,[rsp+0x308]
  65b13d:	xor    ecx,ecx
  65b13f:	call   661b80 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_>
  65b144:	mov    rdi,QWORD PTR [rsp+0xd8]
  65b14c:	jmp    65b193 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3c23>
  65b14e:	call   QWORD PTR [rip+0x9d48c]        # 6f85e0 <_DYNAMIC+0x5c8>
  65b154:	mov    rsi,QWORD PTR [rax]
  65b157:	mov    rdx,QWORD PTR fs:[rbx]
  65b15b:	test   rdx,rdx
  65b15e:	je     65b28e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3d1e>
  65b164:	cmp    QWORD PTR [rdx+0x110],rsi
  65b16b:	mov    rbx,QWORD PTR [rsp+0xd8]
  65b173:	jne    65b2ac <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3d3c>
  65b179:	lea    rdi,[rsp+0x1400]
  65b181:	lea    rsi,[rsp+0x308]
  65b189:	xor    ecx,ecx
  65b18b:	call   661b80 <_RNCINvNtCsiKyBw8IgE1b_10rayon_core4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtBY_3zip11ZipProducerIB26_INtNtNtB10_5slice6chunks14ChunksProducerhEB2z_EIB2A_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtBY_3map11MapConsumerINtNtBY_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB55_s_0ENCB55_0EE0NCBR_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB6m_E0B59_>
  65b190:	mov    rdi,rbx
  65b193:	mov    rax,QWORD PTR [rsp+0x1410]
  65b19b:	mov    QWORD PTR [rsp+0x2410],rax
  65b1a3:	vmovups xmm0,XMMWORD PTR [rsp+0x1400]
  65b1ac:	vmovaps XMMWORD PTR [rsp+0x2400],xmm0
  65b1b5:	lea    rdx,[rsp+0x2418]
  65b1bd:	vmovdqu xmm0,XMMWORD PTR [rsp+0x1418]
  65b1c6:	vmovdqu XMMWORD PTR [rsp+0x2418],xmm0
  65b1cf:	mov    rax,QWORD PTR [rsp+0x1428]
  65b1d7:	mov    QWORD PTR [rsp+0x2428],rax
  65b1df:	lea    rsi,[rsp+0x2400]
  65b1e7:	call   6643c0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0B5_>
  65b1ec:	lea    rsp,[rbp-0x28]
  65b1f0:	pop    rbx
  65b1f1:	pop    r12
  65b1f3:	pop    r13
  65b1f5:	pop    r14
  65b1f7:	pop    r15
  65b1f9:	pop    rbp
  65b1fa:	ret    
  65b1fb:	lea    rdx,[rip+0x99c56]        # 6f4e58 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0xd8>
  65b202:	mov    rsi,QWORD PTR [rsp+0x1c0]
  65b20a:	vzeroupper 
  65b20d:	call   QWORD PTR [rip+0x9d055]        # 6f8268 <_DYNAMIC+0x250>
  65b213:	jmp    65b34d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3ddd>
  65b218:	mov    rsi,r15
  65b21b:	jmp    65b24d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cdd>
  65b21d:	mov    rsi,r13
  65b220:	jmp    65b24d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cdd>
  65b222:	mov    rsi,rax
  65b225:	jmp    65b24d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cdd>
  65b227:	mov    rsi,r10
  65b22a:	jmp    65b24d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cdd>
  65b22c:	mov    rsi,r9
  65b22f:	jmp    65b24d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cdd>
  65b231:	mov    rsi,r11
  65b234:	jmp    65b24d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cdd>
  65b236:	mov    rsi,rdx
  65b239:	jmp    65b24d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cdd>
  65b23b:	mov    rsi,rbx
  65b23e:	jmp    65b24d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cdd>
  65b240:	mov    rsi,r14
  65b243:	jmp    65b24d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cdd>
  65b245:	mov    rsi,r12
  65b248:	jmp    65b24d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cdd>
  65b24a:	mov    rsi,r8
  65b24d:	mov    rdi,rsi
  65b250:	mov    rsi,QWORD PTR [rsp+0x1c0]
  65b258:	mov    rdx,QWORD PTR [rsp+0x158]
  65b260:	mov    rbx,QWORD PTR [rsp+0x38]
  65b265:	call   QWORD PTR [rip+0x9cffd]        # 6f8268 <_DYNAMIC+0x250>
  65b26b:	jmp    65b34d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3ddd>
  65b270:	lea    rdi,[rip+0xffffffffff9b748d]        # 12704 <anon.0e5291c933a25680de2bf032d375ada2.76.llvm.16073103944965275176+0x1>
  65b277:	lea    rdx,[rip+0x9a3a2]        # 6f5620 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x8a0>
  65b27e:	mov    esi,0x37
  65b283:	call   QWORD PTR [rip+0x9d047]        # 6f82d0 <_DYNAMIC+0x2b8>
  65b289:	jmp    65b34d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3ddd>
  65b28e:	sub    rsi,0xffffffffffffff80
  65b292:	lea    rdi,[rsp+0x1400]
  65b29a:	lea    rdx,[rsp+0x308]
  65b2a2:	call   654890 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1N_3zip11ZipProducerIB2V_INtNtNtB1P_5slice6chunks14ChunksProducerhEB3p_EIB3q_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1N_3map11MapConsumerINtNtB1N_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5X_s_0ENCB5X_0EE0NCB1G_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7f_E0TB7f_B7f_EEB61_>
  65b2a7:	jmp    65b144 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3bd4>
  65b2ac:	sub    rsi,0xffffffffffffff80
  65b2b0:	lea    rdi,[rsp+0x1400]
  65b2b8:	lea    rcx,[rsp+0x308]
  65b2c0:	call   655700 <_RINvMs4_NtCsiKyBw8IgE1b_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1O_3zip11ZipProducerIB2W_INtNtNtB1Q_5slice6chunks14ChunksProducerhEB3q_EIB3r_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB1O_3map11MapConsumerINtNtB1O_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB5Y_s_0ENCB5Y_0EE0NCB1H_s_0INtNtCs3KfQW3d0ssT_5alloc3vec3VecAhj10_EB7g_E0TB7g_B7g_EEB62_>
  65b2c5:	jmp    65b190 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3c20>
  65b2ca:	mov    QWORD PTR [rsp+0x1c0],rsi
  65b2d2:	lea    rax,[rip+0x9a107]        # 6f53e0 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x660>
  65b2d9:	mov    QWORD PTR [rsp+0x158],rax
  65b2e1:	jmp    65b24d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3cdd>
  65b2e6:	mov    rdi,QWORD PTR [rsp+0x48]
  65b2eb:	lea    rdx,[rip+0x9a0ee]        # 6f53e0 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x660>
  65b2f2:	mov    rsi,rdi
  65b2f5:	vzeroupper 
  65b2f8:	call   QWORD PTR [rip+0x9cf6a]        # 6f8268 <_DYNAMIC+0x250>
  65b2fe:	jmp    65b34d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3ddd>
  65b300:	mov    edi,0x10
  65b305:	jmp    65b330 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3dc0>
  65b307:	mov    edi,0x1
  65b30c:	mov    esi,0xff0
  65b311:	call   QWORD PTR [rip+0x9cf41]        # 6f8258 <_DYNAMIC+0x240>
  65b317:	jmp    65b34d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3ddd>
  65b319:	lea    rdx,[rip+0x9a0c0]        # 6f53e0 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x660>
  65b320:	mov    rsi,rdi
  65b323:	call   QWORD PTR [rip+0x9cf3f]        # 6f8268 <_DYNAMIC+0x250>
  65b329:	jmp    65b34d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3ddd>
  65b32b:	mov    edi,0x1
  65b330:	mov    esi,0xff0
  65b335:	call   QWORD PTR [rip+0x9cf1d]        # 6f8258 <_DYNAMIC+0x240>
  65b33b:	jmp    65b34d <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3ddd>
  65b33d:	mov    edi,0x1
  65b342:	mov    esi,0xff0
  65b347:	call   QWORD PTR [rip+0x9cf0b]        # 6f8258 <_DYNAMIC+0x240>
  65b34d:	ud2    
  65b34f:	mov    edi,0x1
  65b354:	mov    esi,0xff0
  65b359:	call   QWORD PTR [rip+0x9cef9]        # 6f8258 <_DYNAMIC+0x240>
  65b35f:	jmp    65b380 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3e10>
  65b361:	jmp    65b385 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3e15>
  65b363:	mov    r14,rax
  65b366:	mov    rdi,rbx
  65b369:	mov    rbx,QWORD PTR [rip+0x9ceb0]        # 6f8220 <free@GLIBC_2.2.5>
  65b370:	call   rbx
  65b372:	mov    rdi,r12
  65b375:	call   rbx
  65b377:	jmp    65b3a7 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3e37>
  65b379:	mov    r14,rax
  65b37c:	jmp    65b39e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3e2e>
  65b37e:	jmp    65b380 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3e10>
  65b380:	mov    r14,rax
  65b383:	jmp    65b3a7 <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3e37>
  65b385:	mov    r14,rax
  65b388:	mov    rbx,QWORD PTR [rsp+0x38]
  65b38d:	jmp    65b39e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3e2e>
  65b38f:	mov    r14,rax
  65b392:	jmp    65b39e <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3e2e>
  65b394:	mov    r14,rax
  65b397:	jmp    65b3af <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3e3f>
  65b399:	jmp    65b39b <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3e2b>
  65b39b:	mov    r14,rax
  65b39e:	mov    rdi,rbx
  65b3a1:	call   QWORD PTR [rip+0x9ce79]        # 6f8220 <free@GLIBC_2.2.5>
  65b3a7:	cmp    QWORD PTR [rsp+0x50],0x0
  65b3ad:	je     65b3bd <_RINvNvNtNtCsgA7WsSuASij_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerIB1e_INtNtNtB8_5slice6chunks14ChunksProducerhEB1H_EIB1I_NtNtCskPVWNEMsUTt_11flock_field7gf2_1284F128EEINtNtB6_3map11MapConsumerINtNtB6_6reduce14ReduceConsumerNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregates0_0NCB4c_s_0ENCB4c_0EEB4g_+0x3e4d>
  65b3af:	mov    rdi,QWORD PTR [rsp+0xe8]
  65b3b7:	call   QWORD PTR [rip+0x9ce63]        # 6f8220 <free@GLIBC_2.2.5>
  65b3bd:	mov    rdi,r14
  65b3c0:	call   6f1930 <_Unwind_Resume@plt>
  65b3c5:	int3   
  65b3c6:	int3   
  65b3c7:	int3   
  65b3c8:	int3   
  65b3c9:	int3   
  65b3ca:	int3   
  65b3cb:	int3   
  65b3cc:	int3   
  65b3cd:	int3   
  65b3ce:	int3   
  65b3cf:	int3   

