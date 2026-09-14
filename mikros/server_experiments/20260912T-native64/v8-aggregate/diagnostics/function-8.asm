000000000067d4c0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block>:
  67d4c0:	55                   	push   rbp
  67d4c1:	48 89 e5             	mov    rbp,rsp
  67d4c4:	41 57                	push   r15
  67d4c6:	41 56                	push   r14
  67d4c8:	41 55                	push   r13
  67d4ca:	41 54                	push   r12
  67d4cc:	53                   	push   rbx
  67d4cd:	48 83 e4 c0          	and    rsp,0xffffffffffffffc0
  67d4d1:	48 81 ec c0 0f 00 00 	sub    rsp,0xfc0
  67d4d8:	48 c7 04 24 00 00 00 	mov    QWORD PTR [rsp],0x0
  67d4df:	00 
  67d4e0:	48 81 ec 00 10 00 00 	sub    rsp,0x1000
  67d4e7:	48 c7 04 24 00 00 00 	mov    QWORD PTR [rsp],0x0
  67d4ee:	00 
  67d4ef:	48 81 ec 00 10 00 00 	sub    rsp,0x1000
  67d4f6:	48 c7 04 24 00 00 00 	mov    QWORD PTR [rsp],0x0
  67d4fd:	00 
  67d4fe:	48 81 ec c0 0d 00 00 	sub    rsp,0xdc0
  67d505:	4c 89 8c 24 c8 00 00 	mov    QWORD PTR [rsp+0xc8],r9
  67d50c:	00 
  67d50d:	4d 89 c7             	mov    r15,r8
  67d510:	48 89 8c 24 b8 00 00 	mov    QWORD PTR [rsp+0xb8],rcx
  67d517:	00 
  67d518:	48 89 54 24 30       	mov    QWORD PTR [rsp+0x30],rdx
  67d51d:	48 89 b4 24 b0 00 00 	mov    QWORD PTR [rsp+0xb0],rsi
  67d524:	00 
  67d525:	48 89 bc 24 f0 00 00 	mov    QWORD PTR [rsp+0xf0],rdi
  67d52c:	00 
  67d52d:	8b 05 1d d8 08 00    	mov    eax,DWORD PTR [rip+0x8d81d]        # 70ad50 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1120>
  67d533:	85 c0                	test   eax,eax
  67d535:	0f 85 55 15 00 00    	jne    67ea90 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x15d0>
  67d53b:	8b 05 e7 c6 08 00    	mov    eax,DWORD PTR [rip+0x8c6e7]        # 709c28 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290+0x930>
  67d541:	85 c0                	test   eax,eax
  67d543:	0f 85 5a 15 00 00    	jne    67eaa3 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x15e3>
  67d549:	48 8d bc 24 40 0d 00 	lea    rdi,[rsp+0xd40]
  67d550:	00 
  67d551:	31 db                	xor    ebx,ebx
  67d553:	4c 8b 35 5e 9e 08 00 	mov    r14,QWORD PTR [rip+0x89e5e]        # 7073b8 <memset@GLIBC_2.2.5>
  67d55a:	ba 00 10 00 00       	mov    edx,0x1000
  67d55f:	31 f6                	xor    esi,esi
  67d561:	41 ff d6             	call   r14
  67d564:	48 8d bc 24 40 01 00 	lea    rdi,[rsp+0x140]
  67d56b:	00 
  67d56c:	ba 00 04 00 00       	mov    edx,0x400
  67d571:	31 f6                	xor    esi,esi
  67d573:	41 ff d6             	call   r14
  67d576:	48 8d bc 24 40 09 00 	lea    rdi,[rsp+0x940]
  67d57d:	00 
  67d57e:	ba 00 04 00 00       	mov    edx,0x400
  67d583:	31 f6                	xor    esi,esi
  67d585:	41 ff d6             	call   r14
  67d588:	4d 85 ff             	test   r15,r15
  67d58b:	4c 89 bc 24 c0 00 00 	mov    QWORD PTR [rsp+0xc0],r15
  67d592:	00 
  67d593:	74 21                	je     67d5b6 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0xf6>
  67d595:	48 8b 44 24 30       	mov    rax,QWORD PTR [rsp+0x30]
  67d59a:	48 05 ff 03 00 00    	add    rax,0x3ff
  67d5a0:	48 c1 e8 0a          	shr    rax,0xa
  67d5a4:	49 8d 9f ff 03 00 00 	lea    rbx,[r15+0x3ff]
  67d5ab:	48 c1 eb 0a          	shr    rbx,0xa
  67d5af:	48 39 c3             	cmp    rbx,rax
  67d5b2:	48 0f 43 d8          	cmovae rbx,rax
  67d5b6:	48 83 7d 10 00       	cmp    QWORD PTR [rbp+0x10],0x0
  67d5bb:	0f 84 60 13 00 00    	je     67e921 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x1461>
  67d5c1:	48 8b 45 10          	mov    rax,QWORD PTR [rbp+0x10]
  67d5c5:	48 83 c0 3f          	add    rax,0x3f
  67d5c9:	48 c1 e8 06          	shr    rax,0x6
  67d5cd:	48 39 d8             	cmp    rax,rbx
  67d5d0:	48 0f 43 c3          	cmovae rax,rbx
  67d5d4:	48 89 84 24 f8 00 00 	mov    QWORD PTR [rsp+0xf8],rax
  67d5db:	00 
  67d5dc:	48 85 c0             	test   rax,rax
  67d5df:	0f 84 3c 13 00 00    	je     67e921 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x1461>
  67d5e5:	48 bb c0 ff ff ff ff 	movabs rbx,0x7fffffffffffffc0
  67d5ec:	ff ff 7f 
  67d5ef:	48 8b 84 24 c8 00 00 	mov    rax,QWORD PTR [rsp+0xc8]
  67d5f6:	00 
  67d5f7:	48 8d 48 08          	lea    rcx,[rax+0x8]
  67d5fb:	48 89 4c 24 20       	mov    QWORD PTR [rsp+0x20],rcx
  67d600:	31 d2                	xor    edx,edx
  67d602:	0f b6 4d 18          	movzx  ecx,BYTE PTR [rbp+0x18]
  67d606:	89 8c 24 ac 00 00 00 	mov    DWORD PTR [rsp+0xac],ecx
  67d60d:	4c 8d 25 2c 52 99 ff 	lea    r12,[rip+0xffffffffff99522c]        # 12840 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x16f2>
  67d614:	62 e1 fd 48 6f 3d a2 	vmovdqa64 zmm23,ZMMWORD PTR [rip+0xffffffffff9950a2]        # 126c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1572>
  67d61b:	50 99 ff 
  67d61e:	4c 8d 2d 3b 52 99 ff 	lea    r13,[rip+0xffffffffff99523b]        # 12860 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1712>
  67d625:	62 61 fd 48 6f 05 51 	vmovdqa64 zmm24,ZMMWORD PTR [rip+0xffffffffff995051]        # 12680 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1532>
  67d62c:	50 99 ff 
  67d62f:	4c 8d 35 1a 52 99 ff 	lea    r14,[rip+0xffffffffff99521a]        # 12850 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1702>
  67d636:	4c 8d 3d f3 51 99 ff 	lea    r15,[rip+0xffffffffff9951f3]        # 12830 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x16e2>
  67d63d:	48 8b 8c 24 b8 00 00 	mov    rcx,QWORD PTR [rsp+0xb8]
  67d644:	00 
  67d645:	48 89 8c 24 e0 00 00 	mov    QWORD PTR [rsp+0xe0],rcx
  67d64c:	00 
  67d64d:	48 8b 8c 24 b0 00 00 	mov    rcx,QWORD PTR [rsp+0xb0]
  67d654:	00 
  67d655:	48 89 8c 24 d8 00 00 	mov    QWORD PTR [rsp+0xd8],rcx
  67d65c:	00 
  67d65d:	48 89 84 24 d0 00 00 	mov    QWORD PTR [rsp+0xd0],rax
  67d664:	00 
  67d665:	48 8b b4 24 c0 00 00 	mov    rsi,QWORD PTR [rsp+0xc0]
  67d66c:	00 
  67d66d:	48 8b 7c 24 30       	mov    rdi,QWORD PTR [rsp+0x30]
  67d672:	48 8b 4d 10          	mov    rcx,QWORD PTR [rbp+0x10]
  67d676:	e9 61 01 00 00       	jmp    67d7dc <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x31c>
  67d67b:	0f 1f 44 00 00       	nop    DWORD PTR [rax+rax*1+0x0]
  67d680:	48 ff 84 24 e8 00 00 	inc    QWORD PTR [rsp+0xe8]
  67d687:	00 
  67d688:	48 8d bc 24 40 0d 00 	lea    rdi,[rsp+0xd40]
  67d68f:	00 
  67d690:	48 8d b4 24 40 1d 00 	lea    rsi,[rsp+0x1d40]
  67d697:	00 
  67d698:	48 8b 54 24 18       	mov    rdx,QWORD PTR [rsp+0x18]
  67d69d:	48 8d 8c 24 40 01 00 	lea    rcx,[rsp+0x140]
  67d6a4:	00 
  67d6a5:	49 89 d0             	mov    r8,rdx
  67d6a8:	4c 8d 8c 24 40 09 00 	lea    r9,[rsp+0x940]
  67d6af:	00 
  67d6b0:	6a 00                	push   0x0
  67d6b2:	52                   	push   rdx
  67d6b3:	c5 f8 77             	vzeroupper 
  67d6b6:	e8 c5 fb ff ff       	call   67d280 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled12consume_four>
  67d6bb:	48 83 c4 10          	add    rsp,0x10
  67d6bf:	48 8d bc 24 40 0d 00 	lea    rdi,[rsp+0xd40]
  67d6c6:	00 
  67d6c7:	48 8d b4 24 40 1d 00 	lea    rsi,[rsp+0x1d40]
  67d6ce:	00 
  67d6cf:	48 8b 54 24 18       	mov    rdx,QWORD PTR [rsp+0x18]
  67d6d4:	48 8d 8c 24 40 01 00 	lea    rcx,[rsp+0x140]
  67d6db:	00 
  67d6dc:	49 89 d0             	mov    r8,rdx
  67d6df:	4c 8d 8c 24 40 09 00 	lea    r9,[rsp+0x940]
  67d6e6:	00 
  67d6e7:	6a 04                	push   0x4
  67d6e9:	52                   	push   rdx
  67d6ea:	e8 91 fb ff ff       	call   67d280 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled12consume_four>
  67d6ef:	48 83 c4 10          	add    rsp,0x10
  67d6f3:	48 8d bc 24 40 0d 00 	lea    rdi,[rsp+0xd40]
  67d6fa:	00 
  67d6fb:	48 8d b4 24 40 1d 00 	lea    rsi,[rsp+0x1d40]
  67d702:	00 
  67d703:	48 8b 54 24 18       	mov    rdx,QWORD PTR [rsp+0x18]
  67d708:	48 8d 8c 24 40 01 00 	lea    rcx,[rsp+0x140]
  67d70f:	00 
  67d710:	49 89 d0             	mov    r8,rdx
  67d713:	4c 8d 8c 24 40 09 00 	lea    r9,[rsp+0x940]
  67d71a:	00 
  67d71b:	6a 08                	push   0x8
  67d71d:	52                   	push   rdx
  67d71e:	e8 5d fb ff ff       	call   67d280 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled12consume_four>
  67d723:	48 83 c4 10          	add    rsp,0x10
  67d727:	48 8d bc 24 40 0d 00 	lea    rdi,[rsp+0xd40]
  67d72e:	00 
  67d72f:	48 8d b4 24 40 1d 00 	lea    rsi,[rsp+0x1d40]
  67d736:	00 
  67d737:	48 8b 54 24 18       	mov    rdx,QWORD PTR [rsp+0x18]
  67d73c:	48 8d 8c 24 40 01 00 	lea    rcx,[rsp+0x140]
  67d743:	00 
  67d744:	49 89 d0             	mov    r8,rdx
  67d747:	4c 8d 8c 24 40 09 00 	lea    r9,[rsp+0x940]
  67d74e:	00 
  67d74f:	6a 0c                	push   0xc
  67d751:	52                   	push   rdx
  67d752:	e8 29 fb ff ff       	call   67d280 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled12consume_four>
  67d757:	62 61 fd 48 6f 05 1f 	vmovdqa64 zmm24,ZMMWORD PTR [rip+0xffffffffff994f1f]        # 12680 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1532>
  67d75e:	4f 99 ff 
  67d761:	62 e1 fd 48 6f 3d 55 	vmovdqa64 zmm23,ZMMWORD PTR [rip+0xffffffffff994f55]        # 126c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1572>
  67d768:	4f 99 ff 
  67d76b:	48 8b 94 24 f8 00 00 	mov    rdx,QWORD PTR [rsp+0xf8]
  67d772:	00 
  67d773:	48 83 c4 10          	add    rsp,0x10
  67d777:	48 8b 8c 24 10 01 00 	mov    rcx,QWORD PTR [rsp+0x110]
  67d77e:	00 
  67d77f:	48 83 c1 c0          	add    rcx,0xffffffffffffffc0
  67d783:	48 8b bc 24 18 01 00 	mov    rdi,QWORD PTR [rsp+0x118]
  67d78a:	00 
  67d78b:	48 81 c7 00 fc ff ff 	add    rdi,0xfffffffffffffc00
  67d792:	48 8b b4 24 20 01 00 	mov    rsi,QWORD PTR [rsp+0x120]
  67d799:	00 
  67d79a:	48 81 c6 00 fc ff ff 	add    rsi,0xfffffffffffffc00
  67d7a1:	48 81 44 24 20 00 04 	add    QWORD PTR [rsp+0x20],0x400
  67d7a8:	00 00 
  67d7aa:	48 81 84 24 d0 00 00 	add    QWORD PTR [rsp+0xd0],0x400
  67d7b1:	00 00 04 00 00 
  67d7b6:	48 81 84 24 d8 00 00 	add    QWORD PTR [rsp+0xd8],0x400
  67d7bd:	00 00 04 00 00 
  67d7c2:	48 81 84 24 e0 00 00 	add    QWORD PTR [rsp+0xe0],0x400
  67d7c9:	00 00 04 00 00 
  67d7ce:	48 3b 94 24 f8 00 00 	cmp    rdx,QWORD PTR [rsp+0xf8]
  67d7d5:	00 
  67d7d6:	0f 84 45 11 00 00    	je     67e921 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x1461>
  67d7dc:	48 83 f9 40          	cmp    rcx,0x40
  67d7e0:	41 b8 40 00 00 00    	mov    r8d,0x40
  67d7e6:	4c 0f 42 c1          	cmovb  r8,rcx
  67d7ea:	48 83 f9 01          	cmp    rcx,0x1
  67d7ee:	48 89 8c 24 10 01 00 	mov    QWORD PTR [rsp+0x110],rcx
  67d7f5:	00 
  67d7f6:	48 83 d1 00          	adc    rcx,0x0
  67d7fa:	48 83 f9 40          	cmp    rcx,0x40
  67d7fe:	b8 40 00 00 00       	mov    eax,0x40
  67d803:	48 0f 43 c8          	cmovae rcx,rax
  67d807:	48 81 ff 00 04 00 00 	cmp    rdi,0x400
  67d80e:	b8 00 04 00 00       	mov    eax,0x400
  67d813:	48 89 bc 24 18 01 00 	mov    QWORD PTR [rsp+0x118],rdi
  67d81a:	00 
  67d81b:	48 0f 42 c7          	cmovb  rax,rdi
  67d81f:	c1 e8 04             	shr    eax,0x4
  67d822:	48 81 fe 00 04 00 00 	cmp    rsi,0x400
  67d829:	bf 00 04 00 00       	mov    edi,0x400
  67d82e:	48 89 b4 24 20 01 00 	mov    QWORD PTR [rsp+0x120],rsi
  67d835:	00 
  67d836:	48 0f 42 fe          	cmovb  rdi,rsi
  67d83a:	c1 ef 04             	shr    edi,0x4
  67d83d:	48 39 f8             	cmp    rax,rdi
  67d840:	48 0f 42 f8          	cmovb  rdi,rax
  67d844:	48 89 bc 24 a0 00 00 	mov    QWORD PTR [rsp+0xa0],rdi
  67d84b:	00 
  67d84c:	48 89 d7             	mov    rdi,rdx
  67d84f:	48 c1 e7 0a          	shl    rdi,0xa
  67d853:	48 8b 74 24 30       	mov    rsi,QWORD PTR [rsp+0x30]
  67d858:	48 29 fe             	sub    rsi,rdi
  67d85b:	48 81 fe 00 04 00 00 	cmp    rsi,0x400
  67d862:	b8 00 04 00 00       	mov    eax,0x400
  67d867:	48 0f 43 f0          	cmovae rsi,rax
  67d86b:	48 89 b4 24 08 01 00 	mov    QWORD PTR [rsp+0x108],rsi
  67d872:	00 
  67d873:	48 8b b4 24 c0 00 00 	mov    rsi,QWORD PTR [rsp+0xc0]
  67d87a:	00 
  67d87b:	48 29 fe             	sub    rsi,rdi
  67d87e:	48 81 fe 00 04 00 00 	cmp    rsi,0x400
  67d885:	48 0f 43 f0          	cmovae rsi,rax
  67d889:	48 89 b4 24 00 01 00 	mov    QWORD PTR [rsp+0x100],rsi
  67d890:	00 
  67d891:	48 89 94 24 e8 00 00 	mov    QWORD PTR [rsp+0xe8],rdx
  67d898:	00 
  67d899:	48 89 d0             	mov    rax,rdx
  67d89c:	48 c1 e0 06          	shl    rax,0x6
  67d8a0:	48 8b 55 10          	mov    rdx,QWORD PTR [rbp+0x10]
  67d8a4:	48 29 c2             	sub    rdx,rax
  67d8a7:	48 83 fa 40          	cmp    rdx,0x40
  67d8ab:	be 40 00 00 00       	mov    esi,0x40
  67d8b0:	48 0f 42 f2          	cmovb  rsi,rdx
  67d8b4:	48 89 74 24 18       	mov    QWORD PTR [rsp+0x18],rsi
  67d8b9:	48 8b b4 24 c8 00 00 	mov    rsi,QWORD PTR [rsp+0xc8]
  67d8c0:	00 
  67d8c1:	48 89 7c 24 28       	mov    QWORD PTR [rsp+0x28],rdi
  67d8c6:	48 01 f7             	add    rdi,rsi
  67d8c9:	80 7d 18 01          	cmp    BYTE PTR [rbp+0x18],0x1
  67d8cd:	0f 84 bd 02 00 00    	je     67db90 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x6d0>
  67d8d3:	83 bc 24 ac 00 00 00 	cmp    DWORD PTR [rsp+0xac],0x0
  67d8da:	00 
  67d8db:	0f 85 df 02 00 00    	jne    67dbc0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x700>
  67d8e1:	48 39 45 10          	cmp    QWORD PTR [rbp+0x10],rax
  67d8e5:	0f 84 73 0c 00 00    	je     67e55e <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x109e>
  67d8eb:	48 8b 3d 1e ba 08 00 	mov    rdi,QWORD PTR [rip+0x8ba1e]        # 709310 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290+0x18>
  67d8f2:	48 83 ff 10          	cmp    rdi,0x10
  67d8f6:	0f 82 b1 11 00 00    	jb     67eaad <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x15ed>
  67d8fc:	c1 e1 04             	shl    ecx,0x4
  67d8ff:	48 8b 05 02 ba 08 00 	mov    rax,QWORD PTR [rip+0x8ba02]        # 709308 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290+0x10>
  67d906:	48 8d 90 00 10 00 00 	lea    rdx,[rax+0x1000]
  67d90d:	48 89 54 24 08       	mov    QWORD PTR [rsp+0x8],rdx
  67d912:	48 8d 90 00 20 00 00 	lea    rdx,[rax+0x2000]
  67d919:	48 89 14 24          	mov    QWORD PTR [rsp],rdx
  67d91d:	48 8d 90 00 30 00 00 	lea    rdx,[rax+0x3000]
  67d924:	48 89 94 24 98 00 00 	mov    QWORD PTR [rsp+0x98],rdx
  67d92b:	00 
  67d92c:	48 8d 90 00 40 00 00 	lea    rdx,[rax+0x4000]
  67d933:	48 89 94 24 90 00 00 	mov    QWORD PTR [rsp+0x90],rdx
  67d93a:	00 
  67d93b:	48 8d 90 00 50 00 00 	lea    rdx,[rax+0x5000]
  67d942:	48 89 94 24 88 00 00 	mov    QWORD PTR [rsp+0x88],rdx
  67d949:	00 
  67d94a:	48 8d 90 00 60 00 00 	lea    rdx,[rax+0x6000]
  67d951:	48 89 94 24 80 00 00 	mov    QWORD PTR [rsp+0x80],rdx
  67d958:	00 
  67d959:	48 8d 90 00 70 00 00 	lea    rdx,[rax+0x7000]
  67d960:	48 89 54 24 78       	mov    QWORD PTR [rsp+0x78],rdx
  67d965:	48 8d 90 00 80 00 00 	lea    rdx,[rax+0x8000]
  67d96c:	48 89 54 24 70       	mov    QWORD PTR [rsp+0x70],rdx
  67d971:	48 8d 90 00 90 00 00 	lea    rdx,[rax+0x9000]
  67d978:	48 89 54 24 68       	mov    QWORD PTR [rsp+0x68],rdx
  67d97d:	48 8d 90 00 a0 00 00 	lea    rdx,[rax+0xa000]
  67d984:	48 89 54 24 60       	mov    QWORD PTR [rsp+0x60],rdx
  67d989:	48 8d 90 00 b0 00 00 	lea    rdx,[rax+0xb000]
  67d990:	48 89 54 24 58       	mov    QWORD PTR [rsp+0x58],rdx
  67d995:	48 8d 90 00 c0 00 00 	lea    rdx,[rax+0xc000]
  67d99c:	48 89 54 24 50       	mov    QWORD PTR [rsp+0x50],rdx
  67d9a1:	48 8d 90 00 d0 00 00 	lea    rdx,[rax+0xd000]
  67d9a8:	48 89 54 24 48       	mov    QWORD PTR [rsp+0x48],rdx
  67d9ad:	48 8d 90 00 e0 00 00 	lea    rdx,[rax+0xe000]
  67d9b4:	48 89 54 24 40       	mov    QWORD PTR [rsp+0x40],rdx
  67d9b9:	48 89 44 24 10       	mov    QWORD PTR [rsp+0x10],rax
  67d9be:	48 05 00 f0 00 00    	add    rax,0xf000
  67d9c4:	48 89 44 24 38       	mov    QWORD PTR [rsp+0x38],rax
  67d9c9:	31 f6                	xor    esi,esi
  67d9cb:	0f 1f 44 00 00       	nop    DWORD PTR [rax+rax*1+0x0]
  67d9d0:	48 8b 44 24 20       	mov    rax,QWORD PTR [rsp+0x20]
  67d9d5:	48 8b 7c 30 f8       	mov    rdi,QWORD PTR [rax+rsi*1-0x8]
  67d9da:	49 89 fa             	mov    r10,rdi
  67d9dd:	49 89 f9             	mov    r9,rdi
  67d9e0:	49 89 fb             	mov    r11,rdi
  67d9e3:	48 89 fa             	mov    rdx,rdi
  67d9e6:	41 89 f8             	mov    r8d,edi
  67d9e9:	41 c1 e8 04          	shr    r8d,0x4
  67d9ed:	41 81 e0 f0 0f 00 00 	and    r8d,0xff0
  67d9f4:	48 8b 44 24 08       	mov    rax,QWORD PTR [rsp+0x8]
  67d9f9:	c4 a1 79 6f 04 00    	vmovdqa xmm0,XMMWORD PTR [rax+r8*1]
  67d9ff:	44 0f b6 c7          	movzx  r8d,dil
  67da03:	41 c1 e0 04          	shl    r8d,0x4
  67da07:	48 89 c8             	mov    rax,rcx
  67da0a:	48 8b 4c 24 10       	mov    rcx,QWORD PTR [rsp+0x10]
  67da0f:	c4 a1 79 ef 04 01    	vpxor  xmm0,xmm0,XMMWORD PTR [rcx+r8*1]
  67da15:	48 89 c1             	mov    rcx,rax
  67da18:	41 89 f8             	mov    r8d,edi
  67da1b:	41 c1 e8 0c          	shr    r8d,0xc
  67da1f:	41 81 e0 f0 0f 00 00 	and    r8d,0xff0
  67da26:	c1 ef 14             	shr    edi,0x14
  67da29:	83 e7 f0             	and    edi,0xfffffff0
  67da2c:	48 8b 84 24 98 00 00 	mov    rax,QWORD PTR [rsp+0x98]
  67da33:	00 
  67da34:	c5 f9 6f 0c 38       	vmovdqa xmm1,XMMWORD PTR [rax+rdi*1]
  67da39:	48 8b 04 24          	mov    rax,QWORD PTR [rsp]
  67da3d:	62 b3 fd 08 25 0c 00 	vpternlogq xmm1,xmm0,XMMWORD PTR [rax+r8*1],0x96
  67da44:	96 
  67da45:	48 8b 44 24 20       	mov    rax,QWORD PTR [rsp+0x20]
  67da4a:	4c 8b 04 30          	mov    r8,QWORD PTR [rax+rsi*1]
  67da4e:	4c 89 c7             	mov    rdi,r8
  67da51:	49 c1 eb 28          	shr    r11,0x28
  67da55:	45 0f b6 db          	movzx  r11d,r11b
  67da59:	41 c1 e3 04          	shl    r11d,0x4
  67da5d:	48 8b 84 24 88 00 00 	mov    rax,QWORD PTR [rsp+0x88]
  67da64:	00 
  67da65:	c4 a1 79 6f 04 18    	vmovdqa xmm0,XMMWORD PTR [rax+r11*1]
  67da6b:	4d 89 c3             	mov    r11,r8
  67da6e:	48 c1 ea 20          	shr    rdx,0x20
  67da72:	0f b6 d2             	movzx  edx,dl
  67da75:	c1 e2 04             	shl    edx,0x4
  67da78:	48 8b 84 24 90 00 00 	mov    rax,QWORD PTR [rsp+0x90]
  67da7f:	00 
  67da80:	62 f3 f5 08 25 04 10 	vpternlogq xmm0,xmm1,XMMWORD PTR [rax+rdx*1],0x96
  67da87:	96 
  67da88:	4c 89 c2             	mov    rdx,r8
  67da8b:	49 c1 ea 38          	shr    r10,0x38
  67da8f:	41 c1 e2 04          	shl    r10d,0x4
  67da93:	48 8b 44 24 78       	mov    rax,QWORD PTR [rsp+0x78]
  67da98:	c4 a1 79 6f 0c 10    	vmovdqa xmm1,XMMWORD PTR [rax+r10*1]
  67da9e:	4d 89 c2             	mov    r10,r8
  67daa1:	49 c1 e9 30          	shr    r9,0x30
  67daa5:	45 0f b6 c9          	movzx  r9d,r9b
  67daa9:	41 c1 e1 04          	shl    r9d,0x4
  67daad:	48 8b 84 24 80 00 00 	mov    rax,QWORD PTR [rsp+0x80]
  67dab4:	00 
  67dab5:	62 b3 fd 08 25 0c 08 	vpternlogq xmm1,xmm0,XMMWORD PTR [rax+r9*1],0x96
  67dabc:	96 
  67dabd:	45 89 c1             	mov    r9d,r8d
  67dac0:	41 c1 e9 04          	shr    r9d,0x4
  67dac4:	41 81 e1 f0 0f 00 00 	and    r9d,0xff0
  67dacb:	48 8b 44 24 68       	mov    rax,QWORD PTR [rsp+0x68]
  67dad0:	c4 a1 79 6f 04 08    	vmovdqa xmm0,XMMWORD PTR [rax+r9*1]
  67dad6:	45 0f b6 c8          	movzx  r9d,r8b
  67dada:	41 c1 e1 04          	shl    r9d,0x4
  67dade:	48 8b 44 24 70       	mov    rax,QWORD PTR [rsp+0x70]
  67dae3:	62 b3 f5 08 25 04 08 	vpternlogq xmm0,xmm1,XMMWORD PTR [rax+r9*1],0x96
  67daea:	96 
  67daeb:	45 89 c1             	mov    r9d,r8d
  67daee:	41 c1 e8 14          	shr    r8d,0x14
  67daf2:	41 83 e0 f0          	and    r8d,0xfffffff0
  67daf6:	48 8b 44 24 58       	mov    rax,QWORD PTR [rsp+0x58]
  67dafb:	c4 a1 79 6f 0c 00    	vmovdqa xmm1,XMMWORD PTR [rax+r8*1]
  67db01:	41 c1 e9 0c          	shr    r9d,0xc
  67db05:	41 81 e1 f0 0f 00 00 	and    r9d,0xff0
  67db0c:	48 8b 44 24 60       	mov    rax,QWORD PTR [rsp+0x60]
  67db11:	62 b3 fd 08 25 0c 08 	vpternlogq xmm1,xmm0,XMMWORD PTR [rax+r9*1],0x96
  67db18:	96 
  67db19:	48 c1 ea 28          	shr    rdx,0x28
  67db1d:	0f b6 d2             	movzx  edx,dl
  67db20:	c1 e2 04             	shl    edx,0x4
  67db23:	48 8b 44 24 48       	mov    rax,QWORD PTR [rsp+0x48]
  67db28:	c5 f9 6f 04 10       	vmovdqa xmm0,XMMWORD PTR [rax+rdx*1]
  67db2d:	49 c1 ea 20          	shr    r10,0x20
  67db31:	41 0f b6 d2          	movzx  edx,r10b
  67db35:	c1 e2 04             	shl    edx,0x4
  67db38:	48 8b 44 24 50       	mov    rax,QWORD PTR [rsp+0x50]
  67db3d:	62 f3 f5 08 25 04 10 	vpternlogq xmm0,xmm1,XMMWORD PTR [rax+rdx*1],0x96
  67db44:	96 
  67db45:	48 c1 ef 38          	shr    rdi,0x38
  67db49:	c1 e7 04             	shl    edi,0x4
  67db4c:	48 8b 44 24 38       	mov    rax,QWORD PTR [rsp+0x38]
  67db51:	c5 f9 6f 0c 38       	vmovdqa xmm1,XMMWORD PTR [rax+rdi*1]
  67db56:	49 c1 eb 30          	shr    r11,0x30
  67db5a:	41 0f b6 d3          	movzx  edx,r11b
  67db5e:	c1 e2 04             	shl    edx,0x4
  67db61:	48 8b 44 24 40       	mov    rax,QWORD PTR [rsp+0x40]
  67db66:	62 f3 fd 08 25 0c 10 	vpternlogq xmm1,xmm0,XMMWORD PTR [rax+rdx*1],0x96
  67db6d:	96 
  67db6e:	c5 fa 7f 8c 34 40 01 	vmovdqu XMMWORD PTR [rsp+rsi*1+0x140],xmm1
  67db75:	00 00 
  67db77:	48 83 c6 10          	add    rsi,0x10
  67db7b:	48 39 f1             	cmp    rcx,rsi
  67db7e:	0f 85 4c fe ff ff    	jne    67d9d0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x510>
  67db84:	e9 d5 09 00 00       	jmp    67e55e <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x109e>
  67db89:	0f 1f 80 00 00 00 00 	nop    DWORD PTR [rax+0x0]
  67db90:	48 8b 4c 24 18       	mov    rcx,QWORD PTR [rsp+0x18]
  67db95:	48 89 ce             	mov    rsi,rcx
  67db98:	48 8d 94 24 40 01 00 	lea    rdx,[rsp+0x140]
  67db9f:	00 
  67dba0:	e8 9b 57 ff ff       	call   673340 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate5fieldNtB2_10SmallField15decompose_block>
  67dba5:	62 61 fd 48 6f 05 d1 	vmovdqa64 zmm24,ZMMWORD PTR [rip+0xffffffffff994ad1]        # 12680 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1532>
  67dbac:	4a 99 ff 
  67dbaf:	62 e1 fd 48 6f 3d 07 	vmovdqa64 zmm23,ZMMWORD PTR [rip+0xffffffffff994b07]        # 126c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1572>
  67dbb6:	4b 99 ff 
  67dbb9:	e9 a0 09 00 00       	jmp    67e55e <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x109e>
  67dbbe:	66 90                	xchg   ax,ax
  67dbc0:	48 83 fa 40          	cmp    rdx,0x40
  67dbc4:	0f 82 c2 06 00 00    	jb     67e28c <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0xdcc>
  67dbca:	62 f1 fe 48 6f 07    	vmovdqu64 zmm0,ZMMWORD PTR [rdi]
  67dbd0:	62 f1 fe 48 6f 4f 01 	vmovdqu64 zmm1,ZMMWORD PTR [rdi+0x40]
  67dbd7:	62 f1 fe 48 6f 57 02 	vmovdqu64 zmm2,ZMMWORD PTR [rdi+0x80]
  67dbde:	62 f1 fe 48 6f 5f 03 	vmovdqu64 zmm3,ZMMWORD PTR [rdi+0xc0]
  67dbe5:	62 f1 fe 48 6f 67 04 	vmovdqu64 zmm4,ZMMWORD PTR [rdi+0x100]
  67dbec:	62 f1 fe 48 6f 6f 05 	vmovdqu64 zmm5,ZMMWORD PTR [rdi+0x140]
  67dbf3:	62 f1 fe 48 6f 77 06 	vmovdqu64 zmm6,ZMMWORD PTR [rdi+0x180]
  67dbfa:	62 f1 fe 48 6f 7f 07 	vmovdqu64 zmm7,ZMMWORD PTR [rdi+0x1c0]
  67dc01:	62 71 fe 48 6f 47 08 	vmovdqu64 zmm8,ZMMWORD PTR [rdi+0x200]
  67dc08:	62 71 fe 48 6f 4f 09 	vmovdqu64 zmm9,ZMMWORD PTR [rdi+0x240]
  67dc0f:	62 71 fe 48 6f 57 0a 	vmovdqu64 zmm10,ZMMWORD PTR [rdi+0x280]
  67dc16:	62 71 fe 48 6f 5f 0b 	vmovdqu64 zmm11,ZMMWORD PTR [rdi+0x2c0]
  67dc1d:	62 71 fe 48 6f 67 0c 	vmovdqu64 zmm12,ZMMWORD PTR [rdi+0x300]
  67dc24:	62 71 fe 48 6f 6f 0d 	vmovdqu64 zmm13,ZMMWORD PTR [rdi+0x340]
  67dc2b:	62 71 fe 48 6f 77 0e 	vmovdqu64 zmm14,ZMMWORD PTR [rdi+0x380]
  67dc32:	62 71 fe 48 6f 7f 0f 	vmovdqu64 zmm15,ZMMWORD PTR [rdi+0x3c0]
  67dc39:	62 e1 7d 48 60 c1    	vpunpcklbw zmm16,zmm0,zmm1
  67dc3f:	62 f1 7d 48 68 c1    	vpunpckhbw zmm0,zmm0,zmm1
  67dc45:	62 f1 6d 48 60 cb    	vpunpcklbw zmm1,zmm2,zmm3
  67dc4b:	62 f1 6d 48 68 d3    	vpunpckhbw zmm2,zmm2,zmm3
  67dc51:	62 f1 5d 48 60 dd    	vpunpcklbw zmm3,zmm4,zmm5
  67dc57:	62 f1 5d 48 68 e5    	vpunpckhbw zmm4,zmm4,zmm5
  67dc5d:	62 f1 4d 48 60 ef    	vpunpcklbw zmm5,zmm6,zmm7
  67dc63:	62 f1 4d 48 68 f7    	vpunpckhbw zmm6,zmm6,zmm7
  67dc69:	62 d1 3d 48 60 f9    	vpunpcklbw zmm7,zmm8,zmm9
  67dc6f:	62 51 3d 48 68 c1    	vpunpckhbw zmm8,zmm8,zmm9
  67dc75:	62 51 2d 48 60 cb    	vpunpcklbw zmm9,zmm10,zmm11
  67dc7b:	62 51 2d 48 68 d3    	vpunpckhbw zmm10,zmm10,zmm11
  67dc81:	62 51 1d 48 60 dd    	vpunpcklbw zmm11,zmm12,zmm13
  67dc87:	62 51 1d 48 68 e5    	vpunpckhbw zmm12,zmm12,zmm13
  67dc8d:	62 51 0d 48 60 ef    	vpunpcklbw zmm13,zmm14,zmm15
  67dc93:	62 51 0d 48 68 f7    	vpunpckhbw zmm14,zmm14,zmm15
  67dc99:	62 71 7d 40 61 f9    	vpunpcklwd zmm15,zmm16,zmm1
  67dc9f:	62 f1 7d 40 69 c9    	vpunpckhwd zmm1,zmm16,zmm1
  67dca5:	62 e1 7d 48 61 c2    	vpunpcklwd zmm16,zmm0,zmm2
  67dcab:	62 f1 7d 48 69 c2    	vpunpckhwd zmm0,zmm0,zmm2
  67dcb1:	62 f1 65 48 61 d5    	vpunpcklwd zmm2,zmm3,zmm5
  67dcb7:	62 f1 65 48 69 dd    	vpunpckhwd zmm3,zmm3,zmm5
  67dcbd:	62 f1 5d 48 61 ee    	vpunpcklwd zmm5,zmm4,zmm6
  67dcc3:	62 f1 5d 48 69 e6    	vpunpckhwd zmm4,zmm4,zmm6
  67dcc9:	62 d1 45 48 61 f1    	vpunpcklwd zmm6,zmm7,zmm9
  67dccf:	62 d1 45 48 69 f9    	vpunpckhwd zmm7,zmm7,zmm9
  67dcd5:	62 51 3d 48 61 ca    	vpunpcklwd zmm9,zmm8,zmm10
  67dcdb:	62 51 3d 48 69 c2    	vpunpckhwd zmm8,zmm8,zmm10
  67dce1:	62 51 25 48 61 d5    	vpunpcklwd zmm10,zmm11,zmm13
  67dce7:	62 51 25 48 69 dd    	vpunpckhwd zmm11,zmm11,zmm13
  67dced:	62 51 1d 48 61 ee    	vpunpcklwd zmm13,zmm12,zmm14
  67dcf3:	62 51 1d 48 69 e6    	vpunpckhwd zmm12,zmm12,zmm14
  67dcf9:	62 71 05 48 62 f2    	vpunpckldq zmm14,zmm15,zmm2
  67dcff:	62 71 05 48 6a fa    	vpunpckhdq zmm15,zmm15,zmm2
  67dd05:	62 e1 75 48 62 cb    	vpunpckldq zmm17,zmm1,zmm3
  67dd0b:	62 e1 75 48 6a d3    	vpunpckhdq zmm18,zmm1,zmm3
  67dd11:	62 e1 7d 40 62 dd    	vpunpckldq zmm19,zmm16,zmm5
  67dd17:	62 e1 7d 40 6a c5    	vpunpckhdq zmm16,zmm16,zmm5
  67dd1d:	62 e1 7d 48 62 e4    	vpunpckldq zmm20,zmm0,zmm4
  67dd23:	62 e1 7d 48 6a ec    	vpunpckhdq zmm21,zmm0,zmm4
  67dd29:	62 d1 4d 48 62 ca    	vpunpckldq zmm1,zmm6,zmm10
  67dd2f:	62 d1 4d 48 6a da    	vpunpckhdq zmm3,zmm6,zmm10
  67dd35:	62 d1 45 48 62 eb    	vpunpckldq zmm5,zmm7,zmm11
  67dd3b:	62 d1 45 48 6a fb    	vpunpckhdq zmm7,zmm7,zmm11
  67dd41:	62 51 35 48 62 d5    	vpunpckldq zmm10,zmm9,zmm13
  67dd47:	62 51 35 48 6a dd    	vpunpckhdq zmm11,zmm9,zmm13
  67dd4d:	62 51 3d 48 62 ec    	vpunpckldq zmm13,zmm8,zmm12
  67dd53:	62 c1 3d 48 6a f4    	vpunpckhdq zmm22,zmm8,zmm12
  67dd59:	62 f1 8d 48 6c c1    	vpunpcklqdq zmm0,zmm14,zmm1
  67dd5f:	62 f1 8d 48 6d c9    	vpunpckhqdq zmm1,zmm14,zmm1
  67dd65:	62 f1 85 48 6c d3    	vpunpcklqdq zmm2,zmm15,zmm3
  67dd6b:	62 f1 85 48 6d db    	vpunpckhqdq zmm3,zmm15,zmm3
  67dd71:	62 f1 f5 40 6c e5    	vpunpcklqdq zmm4,zmm17,zmm5
  67dd77:	62 f1 f5 40 6d ed    	vpunpckhqdq zmm5,zmm17,zmm5
  67dd7d:	62 f1 ed 40 6c f7    	vpunpcklqdq zmm6,zmm18,zmm7
  67dd83:	62 f1 ed 40 6d ff    	vpunpckhqdq zmm7,zmm18,zmm7
  67dd89:	62 51 e5 40 6c c2    	vpunpcklqdq zmm8,zmm19,zmm10
  67dd8f:	62 51 e5 40 6d ca    	vpunpckhqdq zmm9,zmm19,zmm10
  67dd95:	62 51 fd 40 6c d3    	vpunpcklqdq zmm10,zmm16,zmm11
  67dd9b:	62 51 fd 40 6d db    	vpunpckhqdq zmm11,zmm16,zmm11
  67dda1:	62 51 dd 40 6c e5    	vpunpcklqdq zmm12,zmm20,zmm13
  67dda7:	62 51 dd 40 6d ed    	vpunpckhqdq zmm13,zmm20,zmm13
  67ddad:	62 31 d5 40 6c f6    	vpunpcklqdq zmm14,zmm21,zmm22
  67ddb3:	62 31 d5 40 6d fe    	vpunpckhqdq zmm15,zmm21,zmm22
  67ddb9:	31 c9                	xor    ecx,ecx
  67ddbb:	48 8d 15 36 b5 08 00 	lea    rdx,[rip+0x8b536]        # 7092f8 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290>
  67ddc2:	eb 25                	jmp    67dde9 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x929>
  67ddc4:	66 66 66 2e 0f 1f 84 	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  67ddcb:	00 00 00 00 00 
  67ddd0:	62 e1 fd 48 7f 44 0c 	vmovdqa64 ZMMWORD PTR [rsp+rcx*1+0x540],zmm16
  67ddd7:	15 
  67ddd8:	48 83 c1 40          	add    rcx,0x40
  67dddc:	48 81 f9 00 04 00 00 	cmp    rcx,0x400
  67dde3:	0f 84 23 02 00 00    	je     67e00c <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0xb4c>
  67dde9:	48 8b 84 4a 30 01 00 	mov    rax,QWORD PTR [rdx+rcx*2+0x130]
  67ddf0:	00 
  67ddf1:	48 85 c0             	test   rax,rax
  67ddf4:	0f 84 f6 01 00 00    	je     67dff0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0xb30>
  67ddfa:	62 e2 fd 48 7c c0    	vpbroadcastq zmm16,rax
  67de00:	62 a3 fd 48 ce c0 00 	vgf2p8affineqb zmm16,zmm0,zmm16,0x0
  67de07:	48 8b 84 4a 38 01 00 	mov    rax,QWORD PTR [rdx+rcx*2+0x138]
  67de0e:	00 
  67de0f:	48 85 c0             	test   rax,rax
  67de12:	74 13                	je     67de27 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x967>
  67de14:	62 e2 fd 48 7c c8    	vpbroadcastq zmm17,rax
  67de1a:	62 a3 f5 48 ce c9 00 	vgf2p8affineqb zmm17,zmm1,zmm17,0x0
  67de21:	62 a1 7d 40 ef c1    	vpxord zmm16,zmm16,zmm17
  67de27:	48 8b 84 4a 40 01 00 	mov    rax,QWORD PTR [rdx+rcx*2+0x140]
  67de2e:	00 
  67de2f:	48 85 c0             	test   rax,rax
  67de32:	74 13                	je     67de47 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x987>
  67de34:	62 e2 fd 48 7c c8    	vpbroadcastq zmm17,rax
  67de3a:	62 a3 ed 48 ce c9 00 	vgf2p8affineqb zmm17,zmm2,zmm17,0x0
  67de41:	62 a1 7d 40 ef c1    	vpxord zmm16,zmm16,zmm17
  67de47:	48 8b 84 4a 48 01 00 	mov    rax,QWORD PTR [rdx+rcx*2+0x148]
  67de4e:	00 
  67de4f:	48 85 c0             	test   rax,rax
  67de52:	74 13                	je     67de67 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x9a7>
  67de54:	62 e2 fd 48 7c c8    	vpbroadcastq zmm17,rax
  67de5a:	62 a3 e5 48 ce c9 00 	vgf2p8affineqb zmm17,zmm3,zmm17,0x0
  67de61:	62 a1 7d 40 ef c1    	vpxord zmm16,zmm16,zmm17
  67de67:	48 8b 84 4a 50 01 00 	mov    rax,QWORD PTR [rdx+rcx*2+0x150]
  67de6e:	00 
  67de6f:	48 85 c0             	test   rax,rax
  67de72:	74 13                	je     67de87 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x9c7>
  67de74:	62 e2 fd 48 7c c8    	vpbroadcastq zmm17,rax
  67de7a:	62 a3 dd 48 ce c9 00 	vgf2p8affineqb zmm17,zmm4,zmm17,0x0
  67de81:	62 a1 7d 40 ef c1    	vpxord zmm16,zmm16,zmm17
  67de87:	48 8b 84 4a 58 01 00 	mov    rax,QWORD PTR [rdx+rcx*2+0x158]
  67de8e:	00 
  67de8f:	48 85 c0             	test   rax,rax
  67de92:	74 13                	je     67dea7 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x9e7>
  67de94:	62 e2 fd 48 7c c8    	vpbroadcastq zmm17,rax
  67de9a:	62 a3 d5 48 ce c9 00 	vgf2p8affineqb zmm17,zmm5,zmm17,0x0
  67dea1:	62 a1 7d 40 ef c1    	vpxord zmm16,zmm16,zmm17
  67dea7:	48 8b 84 4a 60 01 00 	mov    rax,QWORD PTR [rdx+rcx*2+0x160]
  67deae:	00 
  67deaf:	48 85 c0             	test   rax,rax
  67deb2:	74 13                	je     67dec7 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0xa07>
  67deb4:	62 e2 fd 48 7c c8    	vpbroadcastq zmm17,rax
  67deba:	62 a3 cd 48 ce c9 00 	vgf2p8affineqb zmm17,zmm6,zmm17,0x0
  67dec1:	62 a1 7d 40 ef c1    	vpxord zmm16,zmm16,zmm17
  67dec7:	48 8b 84 4a 68 01 00 	mov    rax,QWORD PTR [rdx+rcx*2+0x168]
  67dece:	00 
  67decf:	48 85 c0             	test   rax,rax
  67ded2:	74 13                	je     67dee7 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0xa27>
  67ded4:	62 e2 fd 48 7c c8    	vpbroadcastq zmm17,rax
  67deda:	62 a3 c5 48 ce c9 00 	vgf2p8affineqb zmm17,zmm7,zmm17,0x0
  67dee1:	62 a1 7d 40 ef c1    	vpxord zmm16,zmm16,zmm17
  67dee7:	48 8b 84 4a 70 01 00 	mov    rax,QWORD PTR [rdx+rcx*2+0x170]
  67deee:	00 
  67deef:	48 85 c0             	test   rax,rax
  67def2:	74 13                	je     67df07 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0xa47>
  67def4:	62 e2 fd 48 7c c8    	vpbroadcastq zmm17,rax
  67defa:	62 a3 bd 48 ce c9 00 	vgf2p8affineqb zmm17,zmm8,zmm17,0x0
  67df01:	62 a1 7d 40 ef c1    	vpxord zmm16,zmm16,zmm17
  67df07:	48 8b 84 4a 78 01 00 	mov    rax,QWORD PTR [rdx+rcx*2+0x178]
  67df0e:	00 
  67df0f:	48 85 c0             	test   rax,rax
  67df12:	74 13                	je     67df27 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0xa67>
  67df14:	62 e2 fd 48 7c c8    	vpbroadcastq zmm17,rax
  67df1a:	62 a3 b5 48 ce c9 00 	vgf2p8affineqb zmm17,zmm9,zmm17,0x0
  67df21:	62 a1 7d 40 ef c1    	vpxord zmm16,zmm16,zmm17
  67df27:	48 8b 84 4a 80 01 00 	mov    rax,QWORD PTR [rdx+rcx*2+0x180]
  67df2e:	00 
  67df2f:	48 85 c0             	test   rax,rax
  67df32:	74 13                	je     67df47 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0xa87>
  67df34:	62 e2 fd 48 7c c8    	vpbroadcastq zmm17,rax
  67df3a:	62 a3 ad 48 ce c9 00 	vgf2p8affineqb zmm17,zmm10,zmm17,0x0
  67df41:	62 a1 7d 40 ef c1    	vpxord zmm16,zmm16,zmm17
  67df47:	48 8b 84 4a 88 01 00 	mov    rax,QWORD PTR [rdx+rcx*2+0x188]
  67df4e:	00 
  67df4f:	48 85 c0             	test   rax,rax
  67df52:	74 13                	je     67df67 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0xaa7>
  67df54:	62 e2 fd 48 7c c8    	vpbroadcastq zmm17,rax
  67df5a:	62 a3 a5 48 ce c9 00 	vgf2p8affineqb zmm17,zmm11,zmm17,0x0
  67df61:	62 a1 7d 40 ef c1    	vpxord zmm16,zmm16,zmm17
  67df67:	48 8b 84 4a 90 01 00 	mov    rax,QWORD PTR [rdx+rcx*2+0x190]
  67df6e:	00 
  67df6f:	48 85 c0             	test   rax,rax
  67df72:	74 13                	je     67df87 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0xac7>
  67df74:	62 e2 fd 48 7c c8    	vpbroadcastq zmm17,rax
  67df7a:	62 a3 9d 48 ce c9 00 	vgf2p8affineqb zmm17,zmm12,zmm17,0x0
  67df81:	62 a1 7d 40 ef c1    	vpxord zmm16,zmm16,zmm17
  67df87:	48 8b 84 4a 98 01 00 	mov    rax,QWORD PTR [rdx+rcx*2+0x198]
  67df8e:	00 
  67df8f:	48 85 c0             	test   rax,rax
  67df92:	74 13                	je     67dfa7 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0xae7>
  67df94:	62 e2 fd 48 7c c8    	vpbroadcastq zmm17,rax
  67df9a:	62 a3 95 48 ce c9 00 	vgf2p8affineqb zmm17,zmm13,zmm17,0x0
  67dfa1:	62 a1 7d 40 ef c1    	vpxord zmm16,zmm16,zmm17
  67dfa7:	48 8b 84 4a a0 01 00 	mov    rax,QWORD PTR [rdx+rcx*2+0x1a0]
  67dfae:	00 
  67dfaf:	48 85 c0             	test   rax,rax
  67dfb2:	74 13                	je     67dfc7 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0xb07>
  67dfb4:	62 e2 fd 48 7c c8    	vpbroadcastq zmm17,rax
  67dfba:	62 a3 8d 48 ce c9 00 	vgf2p8affineqb zmm17,zmm14,zmm17,0x0
  67dfc1:	62 a1 7d 40 ef c1    	vpxord zmm16,zmm16,zmm17
  67dfc7:	48 8b 84 4a a8 01 00 	mov    rax,QWORD PTR [rdx+rcx*2+0x1a8]
  67dfce:	00 
  67dfcf:	48 85 c0             	test   rax,rax
  67dfd2:	0f 84 f8 fd ff ff    	je     67ddd0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x910>
  67dfd8:	62 e2 fd 48 7c c8    	vpbroadcastq zmm17,rax
  67dfde:	62 a3 85 48 ce c9 00 	vgf2p8affineqb zmm17,zmm15,zmm17,0x0
  67dfe5:	62 a1 7d 40 ef c1    	vpxord zmm16,zmm16,zmm17
  67dfeb:	e9 e0 fd ff ff       	jmp    67ddd0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x910>
  67dff0:	62 a1 7d 00 ef c0    	vpxord xmm16,xmm16,xmm16
  67dff6:	48 8b 84 4a 38 01 00 	mov    rax,QWORD PTR [rdx+rcx*2+0x138]
  67dffd:	00 
  67dffe:	48 85 c0             	test   rax,rax
  67e001:	0f 85 0d fe ff ff    	jne    67de14 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x954>
  67e007:	e9 1b fe ff ff       	jmp    67de27 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x967>
  67e00c:	62 f1 fd 48 6f 44 24 	vmovdqa64 zmm0,ZMMWORD PTR [rsp+0x540]
  67e013:	15 
  67e014:	62 f1 fd 48 6f 4c 24 	vmovdqa64 zmm1,ZMMWORD PTR [rsp+0x580]
  67e01b:	16 
  67e01c:	62 f1 fd 48 6f 54 24 	vmovdqa64 zmm2,ZMMWORD PTR [rsp+0x5c0]
  67e023:	17 
  67e024:	62 f1 fd 48 6f 5c 24 	vmovdqa64 zmm3,ZMMWORD PTR [rsp+0x600]
  67e02b:	18 
  67e02c:	62 f1 fd 48 6f 64 24 	vmovdqa64 zmm4,ZMMWORD PTR [rsp+0x640]
  67e033:	19 
  67e034:	62 f1 fd 48 6f 6c 24 	vmovdqa64 zmm5,ZMMWORD PTR [rsp+0x680]
  67e03b:	1a 
  67e03c:	62 f1 fd 48 6f 74 24 	vmovdqa64 zmm6,ZMMWORD PTR [rsp+0x6c0]
  67e043:	1b 
  67e044:	62 f1 fd 48 6f 7c 24 	vmovdqa64 zmm7,ZMMWORD PTR [rsp+0x700]
  67e04b:	1c 
  67e04c:	62 71 fd 48 6f 44 24 	vmovdqa64 zmm8,ZMMWORD PTR [rsp+0x740]
  67e053:	1d 
  67e054:	62 71 fd 48 6f 4c 24 	vmovdqa64 zmm9,ZMMWORD PTR [rsp+0x780]
  67e05b:	1e 
  67e05c:	62 71 fd 48 6f 54 24 	vmovdqa64 zmm10,ZMMWORD PTR [rsp+0x7c0]
  67e063:	1f 
  67e064:	62 71 fd 48 6f 5c 24 	vmovdqa64 zmm11,ZMMWORD PTR [rsp+0x800]
  67e06b:	20 
  67e06c:	62 71 fd 48 6f 64 24 	vmovdqa64 zmm12,ZMMWORD PTR [rsp+0x840]
  67e073:	21 
  67e074:	62 71 fd 48 6f 6c 24 	vmovdqa64 zmm13,ZMMWORD PTR [rsp+0x880]
  67e07b:	22 
  67e07c:	62 71 fd 48 6f 74 24 	vmovdqa64 zmm14,ZMMWORD PTR [rsp+0x8c0]
  67e083:	23 
  67e084:	62 71 fd 48 6f 7c 24 	vmovdqa64 zmm15,ZMMWORD PTR [rsp+0x900]
  67e08b:	24 
  67e08c:	62 e1 7d 48 60 c1    	vpunpcklbw zmm16,zmm0,zmm1
  67e092:	62 f1 7d 48 68 c1    	vpunpckhbw zmm0,zmm0,zmm1
  67e098:	62 f1 6d 48 60 cb    	vpunpcklbw zmm1,zmm2,zmm3
  67e09e:	62 f1 6d 48 68 d3    	vpunpckhbw zmm2,zmm2,zmm3
  67e0a4:	62 f1 5d 48 60 dd    	vpunpcklbw zmm3,zmm4,zmm5
  67e0aa:	62 f1 5d 48 68 e5    	vpunpckhbw zmm4,zmm4,zmm5
  67e0b0:	62 f1 4d 48 60 ef    	vpunpcklbw zmm5,zmm6,zmm7
  67e0b6:	62 f1 4d 48 68 f7    	vpunpckhbw zmm6,zmm6,zmm7
  67e0bc:	62 d1 3d 48 60 f9    	vpunpcklbw zmm7,zmm8,zmm9
  67e0c2:	62 51 3d 48 68 c1    	vpunpckhbw zmm8,zmm8,zmm9
  67e0c8:	62 51 2d 48 60 cb    	vpunpcklbw zmm9,zmm10,zmm11
  67e0ce:	62 51 2d 48 68 d3    	vpunpckhbw zmm10,zmm10,zmm11
  67e0d4:	62 51 1d 48 60 dd    	vpunpcklbw zmm11,zmm12,zmm13
  67e0da:	62 51 1d 48 68 e5    	vpunpckhbw zmm12,zmm12,zmm13
  67e0e0:	62 51 0d 48 60 ef    	vpunpcklbw zmm13,zmm14,zmm15
  67e0e6:	62 51 0d 48 68 f7    	vpunpckhbw zmm14,zmm14,zmm15
  67e0ec:	62 71 7d 40 61 f9    	vpunpcklwd zmm15,zmm16,zmm1
  67e0f2:	62 f1 7d 40 69 c9    	vpunpckhwd zmm1,zmm16,zmm1
  67e0f8:	62 e1 7d 48 61 c2    	vpunpcklwd zmm16,zmm0,zmm2
  67e0fe:	62 f1 7d 48 69 c2    	vpunpckhwd zmm0,zmm0,zmm2
  67e104:	62 f1 65 48 61 d5    	vpunpcklwd zmm2,zmm3,zmm5
  67e10a:	62 f1 65 48 69 dd    	vpunpckhwd zmm3,zmm3,zmm5
  67e110:	62 f1 5d 48 61 ee    	vpunpcklwd zmm5,zmm4,zmm6
  67e116:	62 f1 5d 48 69 e6    	vpunpckhwd zmm4,zmm4,zmm6
  67e11c:	62 d1 45 48 61 f1    	vpunpcklwd zmm6,zmm7,zmm9
  67e122:	62 d1 45 48 69 f9    	vpunpckhwd zmm7,zmm7,zmm9
  67e128:	62 51 3d 48 61 ca    	vpunpcklwd zmm9,zmm8,zmm10
  67e12e:	62 51 3d 48 69 c2    	vpunpckhwd zmm8,zmm8,zmm10
  67e134:	62 51 25 48 61 d5    	vpunpcklwd zmm10,zmm11,zmm13
  67e13a:	62 51 25 48 69 dd    	vpunpckhwd zmm11,zmm11,zmm13
  67e140:	62 51 1d 48 61 ee    	vpunpcklwd zmm13,zmm12,zmm14
  67e146:	62 51 1d 48 69 e6    	vpunpckhwd zmm12,zmm12,zmm14
  67e14c:	62 71 05 48 62 f2    	vpunpckldq zmm14,zmm15,zmm2
  67e152:	62 f1 05 48 6a d2    	vpunpckhdq zmm2,zmm15,zmm2
  67e158:	62 71 75 48 62 fb    	vpunpckldq zmm15,zmm1,zmm3
  67e15e:	62 f1 75 48 6a cb    	vpunpckhdq zmm1,zmm1,zmm3
  67e164:	62 f1 7d 40 62 dd    	vpunpckldq zmm3,zmm16,zmm5
  67e16a:	62 f1 7d 40 6a ed    	vpunpckhdq zmm5,zmm16,zmm5
  67e170:	62 e1 7d 48 62 c4    	vpunpckldq zmm16,zmm0,zmm4
  67e176:	62 f1 7d 48 6a c4    	vpunpckhdq zmm0,zmm0,zmm4
  67e17c:	62 d1 4d 48 62 e2    	vpunpckldq zmm4,zmm6,zmm10
  67e182:	62 d1 4d 48 6a f2    	vpunpckhdq zmm6,zmm6,zmm10
  67e188:	62 51 45 48 62 d3    	vpunpckldq zmm10,zmm7,zmm11
  67e18e:	62 d1 45 48 6a fb    	vpunpckhdq zmm7,zmm7,zmm11
  67e194:	62 51 35 48 62 dd    	vpunpckldq zmm11,zmm9,zmm13
  67e19a:	62 51 35 48 6a cd    	vpunpckhdq zmm9,zmm9,zmm13
  67e1a0:	62 51 3d 48 62 ec    	vpunpckldq zmm13,zmm8,zmm12
  67e1a6:	62 51 3d 48 6a c4    	vpunpckhdq zmm8,zmm8,zmm12
  67e1ac:	62 71 8d 48 6c e4    	vpunpcklqdq zmm12,zmm14,zmm4
  67e1b2:	62 f1 8d 48 6d e4    	vpunpckhqdq zmm4,zmm14,zmm4
  67e1b8:	62 71 ed 48 6c f6    	vpunpcklqdq zmm14,zmm2,zmm6
  67e1be:	62 f1 ed 48 6d d6    	vpunpckhqdq zmm2,zmm2,zmm6
  67e1c4:	62 d1 85 48 6c f2    	vpunpcklqdq zmm6,zmm15,zmm10
  67e1ca:	62 51 85 48 6d d2    	vpunpckhqdq zmm10,zmm15,zmm10
  67e1d0:	62 71 f5 48 6c ff    	vpunpcklqdq zmm15,zmm1,zmm7
  67e1d6:	62 f1 f5 48 6d cf    	vpunpckhqdq zmm1,zmm1,zmm7
  67e1dc:	62 d1 e5 48 6c fb    	vpunpcklqdq zmm7,zmm3,zmm11
  67e1e2:	62 d1 e5 48 6d db    	vpunpckhqdq zmm3,zmm3,zmm11
  67e1e8:	62 51 d5 48 6c d9    	vpunpcklqdq zmm11,zmm5,zmm9
  67e1ee:	62 d1 d5 48 6d e9    	vpunpckhqdq zmm5,zmm5,zmm9
  67e1f4:	62 51 fd 40 6c cd    	vpunpcklqdq zmm9,zmm16,zmm13
  67e1fa:	62 51 fd 40 6d ed    	vpunpckhqdq zmm13,zmm16,zmm13
  67e200:	62 c1 fd 48 6c c0    	vpunpcklqdq zmm16,zmm0,zmm8
  67e206:	62 d1 fd 48 6d c0    	vpunpckhqdq zmm0,zmm0,zmm8
  67e20c:	62 71 fe 48 7f 64 24 	vmovdqu64 ZMMWORD PTR [rsp+0x140],zmm12
  67e213:	05 
  67e214:	62 f1 fe 48 7f 64 24 	vmovdqu64 ZMMWORD PTR [rsp+0x180],zmm4
  67e21b:	06 
  67e21c:	62 71 fe 48 7f 74 24 	vmovdqu64 ZMMWORD PTR [rsp+0x1c0],zmm14
  67e223:	07 
  67e224:	62 f1 fe 48 7f 54 24 	vmovdqu64 ZMMWORD PTR [rsp+0x200],zmm2
  67e22b:	08 
  67e22c:	62 f1 fe 48 7f 74 24 	vmovdqu64 ZMMWORD PTR [rsp+0x240],zmm6
  67e233:	09 
  67e234:	62 71 fe 48 7f 54 24 	vmovdqu64 ZMMWORD PTR [rsp+0x280],zmm10
  67e23b:	0a 
  67e23c:	62 71 fe 48 7f 7c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x2c0],zmm15
  67e243:	0b 
  67e244:	62 f1 fe 48 7f 4c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x300],zmm1
  67e24b:	0c 
  67e24c:	62 f1 fe 48 7f 7c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x340],zmm7
  67e253:	0d 
  67e254:	62 f1 fe 48 7f 5c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x380],zmm3
  67e25b:	0e 
  67e25c:	62 71 fe 48 7f 5c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x3c0],zmm11
  67e263:	0f 
  67e264:	62 f1 fe 48 7f 6c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x400],zmm5
  67e26b:	10 
  67e26c:	62 71 fe 48 7f 4c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x440],zmm9
  67e273:	11 
  67e274:	62 71 fe 48 7f 6c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x480],zmm13
  67e27b:	12 
  67e27c:	62 e1 fe 48 7f 44 24 	vmovdqu64 ZMMWORD PTR [rsp+0x4c0],zmm16
  67e283:	13 
  67e284:	62 f1 fe 48 7f 44 24 	vmovdqu64 ZMMWORD PTR [rsp+0x500],zmm0
  67e28b:	14 
  67e28c:	f6 44 24 18 3f       	test   BYTE PTR [rsp+0x18],0x3f
  67e291:	0f 84 c7 02 00 00    	je     67e55e <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x109e>
  67e297:	48 8b 3d 72 b0 08 00 	mov    rdi,QWORD PTR [rip+0x8b072]        # 709310 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290+0x18>
  67e29e:	48 83 ff 10          	cmp    rdi,0x10
  67e2a2:	0f 82 05 08 00 00    	jb     67eaad <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x15ed>
  67e2a8:	44 89 c1             	mov    ecx,r8d
  67e2ab:	83 e1 40             	and    ecx,0x40
  67e2ae:	c1 e1 04             	shl    ecx,0x4
  67e2b1:	48 8d 84 0c 40 01 00 	lea    rax,[rsp+rcx*1+0x140]
  67e2b8:	00 
  67e2b9:	48 89 44 24 10       	mov    QWORD PTR [rsp+0x10],rax
  67e2be:	48 03 8c 24 d0 00 00 	add    rcx,QWORD PTR [rsp+0xd0]
  67e2c5:	00 
  67e2c6:	48 89 0c 24          	mov    QWORD PTR [rsp],rcx
  67e2ca:	41 83 e0 3f          	and    r8d,0x3f
  67e2ce:	41 c1 e0 04          	shl    r8d,0x4
  67e2d2:	48 8b 05 2f b0 08 00 	mov    rax,QWORD PTR [rip+0x8b02f]        # 709308 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290+0x10>
  67e2d9:	48 8d 88 00 10 00 00 	lea    rcx,[rax+0x1000]
  67e2e0:	48 89 8c 24 90 00 00 	mov    QWORD PTR [rsp+0x90],rcx
  67e2e7:	00 
  67e2e8:	48 8d 88 00 20 00 00 	lea    rcx,[rax+0x2000]
  67e2ef:	48 89 8c 24 88 00 00 	mov    QWORD PTR [rsp+0x88],rcx
  67e2f6:	00 
  67e2f7:	48 8d 88 00 30 00 00 	lea    rcx,[rax+0x3000]
  67e2fe:	48 89 8c 24 80 00 00 	mov    QWORD PTR [rsp+0x80],rcx
  67e305:	00 
  67e306:	48 8d 88 00 40 00 00 	lea    rcx,[rax+0x4000]
  67e30d:	48 89 4c 24 78       	mov    QWORD PTR [rsp+0x78],rcx
  67e312:	48 8d 88 00 50 00 00 	lea    rcx,[rax+0x5000]
  67e319:	48 89 4c 24 70       	mov    QWORD PTR [rsp+0x70],rcx
  67e31e:	48 8d 88 00 60 00 00 	lea    rcx,[rax+0x6000]
  67e325:	48 89 4c 24 68       	mov    QWORD PTR [rsp+0x68],rcx
  67e32a:	48 8d 88 00 70 00 00 	lea    rcx,[rax+0x7000]
  67e331:	48 89 4c 24 60       	mov    QWORD PTR [rsp+0x60],rcx
  67e336:	48 8d 88 00 80 00 00 	lea    rcx,[rax+0x8000]
  67e33d:	48 89 4c 24 58       	mov    QWORD PTR [rsp+0x58],rcx
  67e342:	48 8d 88 00 90 00 00 	lea    rcx,[rax+0x9000]
  67e349:	48 89 4c 24 50       	mov    QWORD PTR [rsp+0x50],rcx
  67e34e:	48 8d 88 00 a0 00 00 	lea    rcx,[rax+0xa000]
  67e355:	48 89 4c 24 48       	mov    QWORD PTR [rsp+0x48],rcx
  67e35a:	48 8d 88 00 b0 00 00 	lea    rcx,[rax+0xb000]
  67e361:	48 89 4c 24 40       	mov    QWORD PTR [rsp+0x40],rcx
  67e366:	48 8d 88 00 c0 00 00 	lea    rcx,[rax+0xc000]
  67e36d:	48 89 4c 24 38       	mov    QWORD PTR [rsp+0x38],rcx
  67e372:	48 8d 88 00 d0 00 00 	lea    rcx,[rax+0xd000]
  67e379:	48 89 8c 24 38 01 00 	mov    QWORD PTR [rsp+0x138],rcx
  67e380:	00 
  67e381:	48 8d 88 00 e0 00 00 	lea    rcx,[rax+0xe000]
  67e388:	48 89 8c 24 30 01 00 	mov    QWORD PTR [rsp+0x130],rcx
  67e38f:	00 
  67e390:	48 89 44 24 08       	mov    QWORD PTR [rsp+0x8],rax
  67e395:	48 05 00 f0 00 00    	add    rax,0xf000
  67e39b:	48 89 84 24 28 01 00 	mov    QWORD PTR [rsp+0x128],rax
  67e3a2:	00 
  67e3a3:	45 31 d2             	xor    r10d,r10d
  67e3a6:	4c 89 84 24 98 00 00 	mov    QWORD PTR [rsp+0x98],r8
  67e3ad:	00 
  67e3ae:	66 90                	xchg   ax,ax
  67e3b0:	48 8b 34 24          	mov    rsi,QWORD PTR [rsp]
  67e3b4:	4a 8b 04 16          	mov    rax,QWORD PTR [rsi+r10*1]
  67e3b8:	49 89 c0             	mov    r8,rax
  67e3bb:	48 89 c7             	mov    rdi,rax
  67e3be:	49 89 c1             	mov    r9,rax
  67e3c1:	48 89 c2             	mov    rdx,rax
  67e3c4:	41 89 c3             	mov    r11d,eax
  67e3c7:	41 c1 eb 04          	shr    r11d,0x4
  67e3cb:	41 81 e3 f0 0f 00 00 	and    r11d,0xff0
  67e3d2:	48 8b 8c 24 90 00 00 	mov    rcx,QWORD PTR [rsp+0x90]
  67e3d9:	00 
  67e3da:	c4 a1 79 6f 04 19    	vmovdqa xmm0,XMMWORD PTR [rcx+r11*1]
  67e3e0:	44 0f b6 d8          	movzx  r11d,al
  67e3e4:	41 c1 e3 04          	shl    r11d,0x4
  67e3e8:	48 8b 4c 24 08       	mov    rcx,QWORD PTR [rsp+0x8]
  67e3ed:	c4 a1 79 ef 04 19    	vpxor  xmm0,xmm0,XMMWORD PTR [rcx+r11*1]
  67e3f3:	41 89 c3             	mov    r11d,eax
  67e3f6:	41 c1 eb 0c          	shr    r11d,0xc
  67e3fa:	41 81 e3 f0 0f 00 00 	and    r11d,0xff0
  67e401:	c1 e8 14             	shr    eax,0x14
  67e404:	83 e0 f0             	and    eax,0xfffffff0
  67e407:	48 8b 8c 24 80 00 00 	mov    rcx,QWORD PTR [rsp+0x80]
  67e40e:	00 
  67e40f:	c5 f9 6f 0c 01       	vmovdqa xmm1,XMMWORD PTR [rcx+rax*1]
  67e414:	48 8b 84 24 88 00 00 	mov    rax,QWORD PTR [rsp+0x88]
  67e41b:	00 
  67e41c:	62 b3 fd 08 25 0c 18 	vpternlogq xmm1,xmm0,XMMWORD PTR [rax+r11*1],0x96
  67e423:	96 
  67e424:	4a 8b 44 16 08       	mov    rax,QWORD PTR [rsi+r10*1+0x8]
  67e429:	49 89 c3             	mov    r11,rax
  67e42c:	49 c1 e9 28          	shr    r9,0x28
  67e430:	45 0f b6 c9          	movzx  r9d,r9b
  67e434:	41 c1 e1 04          	shl    r9d,0x4
  67e438:	48 8b 4c 24 70       	mov    rcx,QWORD PTR [rsp+0x70]
  67e43d:	c4 a1 79 6f 04 09    	vmovdqa xmm0,XMMWORD PTR [rcx+r9*1]
  67e443:	49 89 c1             	mov    r9,rax
  67e446:	48 c1 ea 20          	shr    rdx,0x20
  67e44a:	0f b6 d2             	movzx  edx,dl
  67e44d:	c1 e2 04             	shl    edx,0x4
  67e450:	48 8b 4c 24 78       	mov    rcx,QWORD PTR [rsp+0x78]
  67e455:	62 f3 f5 08 25 04 11 	vpternlogq xmm0,xmm1,XMMWORD PTR [rcx+rdx*1],0x96
  67e45c:	96 
  67e45d:	48 89 c2             	mov    rdx,rax
  67e460:	49 c1 e8 38          	shr    r8,0x38
  67e464:	41 c1 e0 04          	shl    r8d,0x4
  67e468:	48 8b 4c 24 60       	mov    rcx,QWORD PTR [rsp+0x60]
  67e46d:	c4 a1 79 6f 0c 01    	vmovdqa xmm1,XMMWORD PTR [rcx+r8*1]
  67e473:	49 89 c0             	mov    r8,rax
  67e476:	48 c1 ef 30          	shr    rdi,0x30
  67e47a:	40 0f b6 ff          	movzx  edi,dil
  67e47e:	c1 e7 04             	shl    edi,0x4
  67e481:	48 8b 4c 24 68       	mov    rcx,QWORD PTR [rsp+0x68]
  67e486:	62 f3 fd 08 25 0c 39 	vpternlogq xmm1,xmm0,XMMWORD PTR [rcx+rdi*1],0x96
  67e48d:	96 
  67e48e:	89 c7                	mov    edi,eax
  67e490:	c1 ef 04             	shr    edi,0x4
  67e493:	81 e7 f0 0f 00 00    	and    edi,0xff0
  67e499:	48 8b 4c 24 50       	mov    rcx,QWORD PTR [rsp+0x50]
  67e49e:	c5 f9 6f 04 39       	vmovdqa xmm0,XMMWORD PTR [rcx+rdi*1]
  67e4a3:	0f b6 f8             	movzx  edi,al
  67e4a6:	c1 e7 04             	shl    edi,0x4
  67e4a9:	48 8b 4c 24 58       	mov    rcx,QWORD PTR [rsp+0x58]
  67e4ae:	62 f3 f5 08 25 04 39 	vpternlogq xmm0,xmm1,XMMWORD PTR [rcx+rdi*1],0x96
  67e4b5:	96 
  67e4b6:	89 c7                	mov    edi,eax
  67e4b8:	c1 e8 14             	shr    eax,0x14
  67e4bb:	83 e0 f0             	and    eax,0xfffffff0
  67e4be:	48 8b 4c 24 40       	mov    rcx,QWORD PTR [rsp+0x40]
  67e4c3:	c5 f9 6f 0c 01       	vmovdqa xmm1,XMMWORD PTR [rcx+rax*1]
  67e4c8:	c1 ef 0c             	shr    edi,0xc
  67e4cb:	81 e7 f0 0f 00 00    	and    edi,0xff0
  67e4d1:	48 8b 44 24 48       	mov    rax,QWORD PTR [rsp+0x48]
  67e4d6:	62 f3 fd 08 25 0c 38 	vpternlogq xmm1,xmm0,XMMWORD PTR [rax+rdi*1],0x96
  67e4dd:	96 
  67e4de:	48 c1 ea 28          	shr    rdx,0x28
  67e4e2:	0f b6 c2             	movzx  eax,dl
  67e4e5:	c1 e0 04             	shl    eax,0x4
  67e4e8:	48 8b 8c 24 38 01 00 	mov    rcx,QWORD PTR [rsp+0x138]
  67e4ef:	00 
  67e4f0:	c5 f9 6f 04 01       	vmovdqa xmm0,XMMWORD PTR [rcx+rax*1]
  67e4f5:	49 c1 e8 20          	shr    r8,0x20
  67e4f9:	41 0f b6 c0          	movzx  eax,r8b
  67e4fd:	4c 8b 84 24 98 00 00 	mov    r8,QWORD PTR [rsp+0x98]
  67e504:	00 
  67e505:	c1 e0 04             	shl    eax,0x4
  67e508:	48 8b 4c 24 38       	mov    rcx,QWORD PTR [rsp+0x38]
  67e50d:	62 f3 f5 08 25 04 01 	vpternlogq xmm0,xmm1,XMMWORD PTR [rcx+rax*1],0x96
  67e514:	96 
  67e515:	49 c1 eb 38          	shr    r11,0x38
  67e519:	41 c1 e3 04          	shl    r11d,0x4
  67e51d:	48 8b 84 24 28 01 00 	mov    rax,QWORD PTR [rsp+0x128]
  67e524:	00 
  67e525:	c4 a1 79 6f 0c 18    	vmovdqa xmm1,XMMWORD PTR [rax+r11*1]
  67e52b:	49 c1 e9 30          	shr    r9,0x30
  67e52f:	41 0f b6 c1          	movzx  eax,r9b
  67e533:	c1 e0 04             	shl    eax,0x4
  67e536:	48 8b 8c 24 30 01 00 	mov    rcx,QWORD PTR [rsp+0x130]
  67e53d:	00 
  67e53e:	62 f3 fd 08 25 0c 01 	vpternlogq xmm1,xmm0,XMMWORD PTR [rcx+rax*1],0x96
  67e545:	96 
  67e546:	48 8b 44 24 10       	mov    rax,QWORD PTR [rsp+0x10]
  67e54b:	c4 a1 7a 7f 0c 10    	vmovdqu XMMWORD PTR [rax+r10*1],xmm1
  67e551:	49 83 c2 10          	add    r10,0x10
  67e555:	4d 39 d0             	cmp    r8,r10
  67e558:	0f 85 52 fe ff ff    	jne    67e3b0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0xef0>
  67e55e:	48 8b 84 24 08 01 00 	mov    rax,QWORD PTR [rsp+0x108]
  67e565:	00 
  67e566:	48 c1 e8 04          	shr    rax,0x4
  67e56a:	48 8b 8c 24 00 01 00 	mov    rcx,QWORD PTR [rsp+0x100]
  67e571:	00 
  67e572:	48 c1 e9 04          	shr    rcx,0x4
  67e576:	48 39 c1             	cmp    rcx,rax
  67e579:	48 0f 43 c8          	cmovae rcx,rax
  67e57d:	48 85 c9             	test   rcx,rcx
  67e580:	0f 84 fa f0 ff ff    	je     67d680 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x1c0>
  67e586:	48 8b 84 24 b0 00 00 	mov    rax,QWORD PTR [rsp+0xb0]
  67e58d:	00 
  67e58e:	48 8b 4c 24 28       	mov    rcx,QWORD PTR [rsp+0x28]
  67e593:	48 01 c8             	add    rax,rcx
  67e596:	48 89 04 24          	mov    QWORD PTR [rsp],rax
  67e59a:	48 03 8c 24 b8 00 00 	add    rcx,QWORD PTR [rsp+0xb8]
  67e5a1:	00 
  67e5a2:	48 89 4c 24 28       	mov    QWORD PTR [rsp+0x28],rcx
  67e5a7:	48 8b 0d 8a c6 08 00 	mov    rcx,QWORD PTR [rip+0x8c68a]        # 70ac38 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1008>
  67e5ae:	48 03 0d c3 c6 08 00 	add    rcx,QWORD PTR [rip+0x8c6c3]        # 70ac78 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1048>
  67e5b5:	48 8b 94 24 e0 00 00 	mov    rdx,QWORD PTR [rsp+0xe0]
  67e5bc:	00 
  67e5bd:	48 8b b4 24 d8 00 00 	mov    rsi,QWORD PTR [rsp+0xd8]
  67e5c4:	00 
  67e5c5:	31 ff                	xor    edi,edi
  67e5c7:	eb 45                	jmp    67e60e <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x114e>
  67e5c9:	0f 1f 80 00 00 00 00 	nop    DWORD PTR [rax+0x0]
  67e5d0:	48 c1 e7 07          	shl    rdi,0x7
  67e5d4:	62 f2 75 48 cf cb    	vgf2p8mulb zmm1,zmm1,zmm3
  67e5da:	62 f1 fd 48 7f 4c 3c 	vmovdqa64 ZMMWORD PTR [rsp+rdi*1+0x1d40],zmm1
  67e5e1:	75 
  67e5e2:	62 f2 7d 48 cf c2    	vgf2p8mulb zmm0,zmm0,zmm2
  67e5e8:	62 f1 fd 48 7f 44 3c 	vmovdqa64 ZMMWORD PTR [rsp+rdi*1+0x1d80],zmm0
  67e5ef:	76 
  67e5f0:	48 83 c6 10          	add    rsi,0x10
  67e5f4:	48 83 c2 10          	add    rdx,0x10
  67e5f8:	48 8b 44 24 10       	mov    rax,QWORD PTR [rsp+0x10]
  67e5fd:	48 89 c7             	mov    rdi,rax
  67e600:	48 3b 84 24 a0 00 00 	cmp    rax,QWORD PTR [rsp+0xa0]
  67e607:	00 
  67e608:	0f 84 72 f0 ff ff    	je     67d680 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x1c0>
  67e60e:	48 8d 47 01          	lea    rax,[rdi+0x1]
  67e612:	48 89 44 24 10       	mov    QWORD PTR [rsp+0x10],rax
  67e617:	48 89 f8             	mov    rax,rdi
  67e61a:	48 c1 e0 04          	shl    rax,0x4
  67e61e:	4c 8b 4c 24 28       	mov    r9,QWORD PTR [rsp+0x28]
  67e623:	4d 8b 04 01          	mov    r8,QWORD PTR [r9+rax*1]
  67e627:	4c 89 44 24 08       	mov    QWORD PTR [rsp+0x8],r8
  67e62c:	4c 8b 04 24          	mov    r8,QWORD PTR [rsp]
  67e630:	c4 c1 78 10 04 00    	vmovups xmm0,XMMWORD PTR [r8+rax*1]
  67e636:	45 8b 04 00          	mov    r8d,DWORD PTR [r8+rax*1]
  67e63a:	c4 c1 78 54 04 01    	vandps xmm0,xmm0,XMMWORD PTR [r9+rax*1]
  67e640:	c5 f8 11 84 04 40 09 	vmovups XMMWORD PTR [rsp+rax*1+0x940],xmm0
  67e647:	00 00 
  67e649:	41 0f b6 c0          	movzx  eax,r8b
  67e64d:	c1 e0 07             	shl    eax,0x7
  67e650:	62 f1 fe 48 6f 0c 01 	vmovdqu64 zmm1,ZMMWORD PTR [rcx+rax*1]
  67e657:	62 f1 fe 48 6f 44 01 	vmovdqu64 zmm0,ZMMWORD PTR [rcx+rax*1+0x40]
  67e65e:	01 
  67e65f:	41 b9 01 00 00 00    	mov    r9d,0x1
  67e665:	41 b8 08 00 00 00    	mov    r8d,0x8
  67e66b:	eb 3c                	jmp    67e6a9 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x11e9>
  67e66d:	62 f2 c5 40 36 d2    	vpermq zmm2,zmm23,zmm2
  67e673:	49 83 f3 40          	xor    r11,0x40
  67e677:	62 b1 fe 48 6f 1c 18 	vmovdqu64 zmm3,ZMMWORD PTR [rax+r11*1]
  67e67e:	62 f3 e5 48 43 db 4e 	vshufi64x2 zmm3,zmm3,zmm3,0x4e
  67e685:	62 f1 6d 48 ef c9    	vpxord zmm1,zmm2,zmm1
  67e68b:	62 f1 7d 48 70 d3 4e 	vpshufd zmm2,zmm3,0x4e
  67e692:	62 f1 6d 48 ef c0    	vpxord zmm0,zmm2,zmm0
  67e698:	49 ff c1             	inc    r9
  67e69b:	49 83 c0 08          	add    r8,0x8
  67e69f:	49 83 f9 10          	cmp    r9,0x10
  67e6a3:	0f 84 07 01 00 00    	je     67e7b0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x12f0>
  67e6a9:	42 0f b6 04 0e       	movzx  eax,BYTE PTR [rsi+r9*1]
  67e6ae:	c1 e0 07             	shl    eax,0x7
  67e6b1:	48 01 c8             	add    rax,rcx
  67e6b4:	45 89 ca             	mov    r10d,r9d
  67e6b7:	41 d1 ea             	shr    r10d,1
  67e6ba:	41 83 e2 03          	and    r10d,0x3
  67e6be:	4d 89 c3             	mov    r11,r8
  67e6c1:	49 21 db             	and    r11,rbx
  67e6c4:	62 d1 fe 48 6f 14 03 	vmovdqu64 zmm2,ZMMWORD PTR [r11+rax*1]
  67e6cb:	41 f6 c1 01          	test   r9b,0x1
  67e6cf:	75 1f                	jne    67e6f0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x1230>
  67e6d1:	4f 63 14 94          	movsxd r10,DWORD PTR [r12+r10*4]
  67e6d5:	4d 01 e2             	add    r10,r12
  67e6d8:	41 ff e2             	jmp    r10
  67e6db:	49 83 f3 40          	xor    r11,0x40
  67e6df:	62 b1 fe 48 6f 1c 18 	vmovdqu64 zmm3,ZMMWORD PTR [rax+r11*1]
  67e6e6:	e9 8c 00 00 00       	jmp    67e777 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x12b7>
  67e6eb:	0f 1f 44 00 00       	nop    DWORD PTR [rax+rax*1+0x0]
  67e6f0:	4f 63 14 97          	movsxd r10,DWORD PTR [r15+r10*4]
  67e6f4:	4d 01 fa             	add    r10,r15
  67e6f7:	41 ff e2             	jmp    r10
  67e6fa:	62 f1 7d 48 70 d2 4e 	vpshufd zmm2,zmm2,0x4e
  67e701:	49 83 f3 40          	xor    r11,0x40
  67e705:	62 b1 fe 48 6f 1c 18 	vmovdqu64 zmm3,ZMMWORD PTR [rax+r11*1]
  67e70c:	e9 74 ff ff ff       	jmp    67e685 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x11c5>
  67e711:	62 f3 ed 48 43 d2 4e 	vshufi64x2 zmm2,zmm2,zmm2,0x4e
  67e718:	49 83 f3 40          	xor    r11,0x40
  67e71c:	62 b1 fe 48 6f 1c 18 	vmovdqu64 zmm3,ZMMWORD PTR [rax+r11*1]
  67e723:	62 f3 e5 48 43 db 4e 	vshufi64x2 zmm3,zmm3,zmm3,0x4e
  67e72a:	eb 4b                	jmp    67e777 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x12b7>
  67e72c:	62 f3 ed 48 43 d2 1b 	vshufi64x2 zmm2,zmm2,zmm2,0x1b
  67e733:	49 83 f3 40          	xor    r11,0x40
  67e737:	62 b1 fe 48 6f 1c 18 	vmovdqu64 zmm3,ZMMWORD PTR [rax+r11*1]
  67e73e:	62 f3 e5 48 43 db 1b 	vshufi64x2 zmm3,zmm3,zmm3,0x1b
  67e745:	eb 30                	jmp    67e777 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x12b7>
  67e747:	62 f2 bd 40 36 d2    	vpermq zmm2,zmm24,zmm2
  67e74d:	49 83 f3 40          	xor    r11,0x40
  67e751:	62 b1 fe 48 6f 1c 18 	vmovdqu64 zmm3,ZMMWORD PTR [rax+r11*1]
  67e758:	62 f3 e5 48 43 db 1b 	vshufi64x2 zmm3,zmm3,zmm3,0x1b
  67e75f:	e9 21 ff ff ff       	jmp    67e685 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x11c5>
  67e764:	62 f3 fd 48 00 d2 4e 	vpermq zmm2,zmm2,0x4e
  67e76b:	49 83 f3 40          	xor    r11,0x40
  67e76f:	62 b3 fd 48 00 1c 18 	vpermq zmm3,ZMMWORD PTR [rax+r11*1],0x4e
  67e776:	4e 
  67e777:	62 f1 75 48 ef ca    	vpxord zmm1,zmm1,zmm2
  67e77d:	62 f1 7d 48 ef c3    	vpxord zmm0,zmm0,zmm3
  67e783:	49 ff c1             	inc    r9
  67e786:	49 83 c0 08          	add    r8,0x8
  67e78a:	49 83 f9 10          	cmp    r9,0x10
  67e78e:	0f 85 15 ff ff ff    	jne    67e6a9 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x11e9>
  67e794:	eb 1a                	jmp    67e7b0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x12f0>
  67e796:	62 f3 fd 48 00 d2 1b 	vpermq zmm2,zmm2,0x1b
  67e79d:	49 83 f3 40          	xor    r11,0x40
  67e7a1:	62 b3 fd 48 00 1c 18 	vpermq zmm3,ZMMWORD PTR [rax+r11*1],0x4e
  67e7a8:	4e 
  67e7a9:	e9 d7 fe ff ff       	jmp    67e685 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x11c5>
  67e7ae:	66 90                	xchg   ax,ax
  67e7b0:	0f b6 44 24 08       	movzx  eax,BYTE PTR [rsp+0x8]
  67e7b5:	c1 e0 07             	shl    eax,0x7
  67e7b8:	62 f1 fe 48 6f 1c 01 	vmovdqu64 zmm3,ZMMWORD PTR [rcx+rax*1]
  67e7bf:	62 f1 fe 48 6f 54 01 	vmovdqu64 zmm2,ZMMWORD PTR [rcx+rax*1+0x40]
  67e7c6:	01 
  67e7c7:	41 b9 01 00 00 00    	mov    r9d,0x1
  67e7cd:	41 ba 08 00 00 00    	mov    r10d,0x8
  67e7d3:	eb 3c                	jmp    67e811 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x1351>
  67e7d5:	62 f2 c5 40 36 e4    	vpermq zmm4,zmm23,zmm4
  67e7db:	49 83 f0 40          	xor    r8,0x40
  67e7df:	62 b1 fe 48 6f 2c 00 	vmovdqu64 zmm5,ZMMWORD PTR [rax+r8*1]
  67e7e6:	62 f3 d5 48 43 ed 4e 	vshufi64x2 zmm5,zmm5,zmm5,0x4e
  67e7ed:	62 f1 5d 48 ef db    	vpxord zmm3,zmm4,zmm3
  67e7f3:	62 f1 7d 48 70 e5 4e 	vpshufd zmm4,zmm5,0x4e
  67e7fa:	62 f1 5d 48 ef d2    	vpxord zmm2,zmm4,zmm2
  67e800:	49 ff c1             	inc    r9
  67e803:	49 83 c2 08          	add    r10,0x8
  67e807:	49 83 f9 10          	cmp    r9,0x10
  67e80b:	0f 84 bf fd ff ff    	je     67e5d0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x1110>
  67e811:	42 0f b6 04 0a       	movzx  eax,BYTE PTR [rdx+r9*1]
  67e816:	c1 e0 07             	shl    eax,0x7
  67e819:	48 01 c8             	add    rax,rcx
  67e81c:	45 89 cb             	mov    r11d,r9d
  67e81f:	41 d1 eb             	shr    r11d,1
  67e822:	41 83 e3 03          	and    r11d,0x3
  67e826:	4d 89 d0             	mov    r8,r10
  67e829:	49 21 d8             	and    r8,rbx
  67e82c:	62 d1 fe 48 6f 24 00 	vmovdqu64 zmm4,ZMMWORD PTR [r8+rax*1]
  67e833:	41 f6 c1 01          	test   r9b,0x1
  67e837:	75 27                	jne    67e860 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x13a0>
  67e839:	4f 63 5c 9d 00       	movsxd r11,DWORD PTR [r13+r11*4+0x0]
  67e83e:	4d 01 eb             	add    r11,r13
  67e841:	41 ff e3             	jmp    r11
  67e844:	49 83 f0 40          	xor    r8,0x40
  67e848:	62 b1 fe 48 6f 2c 00 	vmovdqu64 zmm5,ZMMWORD PTR [rax+r8*1]
  67e84f:	e9 93 00 00 00       	jmp    67e8e7 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x1427>
  67e854:	66 66 66 2e 0f 1f 84 	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  67e85b:	00 00 00 00 00 
  67e860:	4f 63 1c 9e          	movsxd r11,DWORD PTR [r14+r11*4]
  67e864:	4d 01 f3             	add    r11,r14
  67e867:	41 ff e3             	jmp    r11
  67e86a:	62 f1 7d 48 70 e4 4e 	vpshufd zmm4,zmm4,0x4e
  67e871:	49 83 f0 40          	xor    r8,0x40
  67e875:	62 b1 fe 48 6f 2c 00 	vmovdqu64 zmm5,ZMMWORD PTR [rax+r8*1]
  67e87c:	e9 6c ff ff ff       	jmp    67e7ed <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x132d>
  67e881:	62 f3 dd 48 43 e4 4e 	vshufi64x2 zmm4,zmm4,zmm4,0x4e
  67e888:	49 83 f0 40          	xor    r8,0x40
  67e88c:	62 b1 fe 48 6f 2c 00 	vmovdqu64 zmm5,ZMMWORD PTR [rax+r8*1]
  67e893:	62 f3 d5 48 43 ed 4e 	vshufi64x2 zmm5,zmm5,zmm5,0x4e
  67e89a:	eb 4b                	jmp    67e8e7 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x1427>
  67e89c:	62 f3 dd 48 43 e4 1b 	vshufi64x2 zmm4,zmm4,zmm4,0x1b
  67e8a3:	49 83 f0 40          	xor    r8,0x40
  67e8a7:	62 b1 fe 48 6f 2c 00 	vmovdqu64 zmm5,ZMMWORD PTR [rax+r8*1]
  67e8ae:	62 f3 d5 48 43 ed 1b 	vshufi64x2 zmm5,zmm5,zmm5,0x1b
  67e8b5:	eb 30                	jmp    67e8e7 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x1427>
  67e8b7:	62 f2 bd 40 36 e4    	vpermq zmm4,zmm24,zmm4
  67e8bd:	49 83 f0 40          	xor    r8,0x40
  67e8c1:	62 b1 fe 48 6f 2c 00 	vmovdqu64 zmm5,ZMMWORD PTR [rax+r8*1]
  67e8c8:	62 f3 d5 48 43 ed 1b 	vshufi64x2 zmm5,zmm5,zmm5,0x1b
  67e8cf:	e9 19 ff ff ff       	jmp    67e7ed <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x132d>
  67e8d4:	62 f3 fd 48 00 e4 4e 	vpermq zmm4,zmm4,0x4e
  67e8db:	49 83 f0 40          	xor    r8,0x40
  67e8df:	62 b3 fd 48 00 2c 00 	vpermq zmm5,ZMMWORD PTR [rax+r8*1],0x4e
  67e8e6:	4e 
  67e8e7:	62 f1 65 48 ef dc    	vpxord zmm3,zmm3,zmm4
  67e8ed:	62 f1 6d 48 ef d5    	vpxord zmm2,zmm2,zmm5
  67e8f3:	49 ff c1             	inc    r9
  67e8f6:	49 83 c2 08          	add    r10,0x8
  67e8fa:	49 83 f9 10          	cmp    r9,0x10
  67e8fe:	0f 85 0d ff ff ff    	jne    67e811 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x1351>
  67e904:	e9 c7 fc ff ff       	jmp    67e5d0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x1110>
  67e909:	62 f3 fd 48 00 e4 1b 	vpermq zmm4,zmm4,0x1b
  67e910:	49 83 f0 40          	xor    r8,0x40
  67e914:	62 b3 fd 48 00 2c 00 	vpermq zmm5,ZMMWORD PTR [rax+r8*1],0x4e
  67e91b:	4e 
  67e91c:	e9 cc fe ff ff       	jmp    67e7ed <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x132d>
  67e921:	bf f0 0f 00 00       	mov    edi,0xff0
  67e926:	ff 15 5c 8a 08 00    	call   QWORD PTR [rip+0x88a5c]        # 707388 <malloc@GLIBC_2.2.5>
  67e92c:	48 85 c0             	test   rax,rax
  67e92f:	0f 84 8b 01 00 00    	je     67eac0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x1600>
  67e935:	b9 0f 00 00 00       	mov    ecx,0xf
  67e93a:	48 8d 94 24 40 1c 00 	lea    rdx,[rsp+0x1c40]
  67e941:	00 
  67e942:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  67e949:	1f 84 00 00 00 00 00 
  67e950:	0f b6 b2 00 f1 ff ff 	movzx  esi,BYTE PTR [rdx-0xf00]
  67e957:	40 88 74 24 08       	mov    BYTE PTR [rsp+0x8],sil
  67e95c:	0f b6 ba 00 f2 ff ff 	movzx  edi,BYTE PTR [rdx-0xe00]
  67e963:	44 0f b6 82 00 f3 ff 	movzx  r8d,BYTE PTR [rdx-0xd00]
  67e96a:	ff 
  67e96b:	44 0f b6 8a 00 f4 ff 	movzx  r9d,BYTE PTR [rdx-0xc00]
  67e972:	ff 
  67e973:	44 0f b6 92 00 f5 ff 	movzx  r10d,BYTE PTR [rdx-0xb00]
  67e97a:	ff 
  67e97b:	44 0f b6 9a 00 f6 ff 	movzx  r11d,BYTE PTR [rdx-0xa00]
  67e982:	ff 
  67e983:	0f b6 9a 00 f7 ff ff 	movzx  ebx,BYTE PTR [rdx-0x900]
  67e98a:	44 0f b6 b2 00 f8 ff 	movzx  r14d,BYTE PTR [rdx-0x800]
  67e991:	ff 
  67e992:	44 0f b6 ba 00 f9 ff 	movzx  r15d,BYTE PTR [rdx-0x700]
  67e999:	ff 
  67e99a:	44 0f b6 a2 00 fa ff 	movzx  r12d,BYTE PTR [rdx-0x600]
  67e9a1:	ff 
  67e9a2:	44 0f b6 aa 00 fb ff 	movzx  r13d,BYTE PTR [rdx-0x500]
  67e9a9:	ff 
  67e9aa:	0f b6 b2 00 fc ff ff 	movzx  esi,BYTE PTR [rdx-0x400]
  67e9b1:	40 88 74 24 10       	mov    BYTE PTR [rsp+0x10],sil
  67e9b6:	0f b6 b2 00 fd ff ff 	movzx  esi,BYTE PTR [rdx-0x300]
  67e9bd:	40 88 74 24 28       	mov    BYTE PTR [rsp+0x28],sil
  67e9c2:	0f b6 b2 00 fe ff ff 	movzx  esi,BYTE PTR [rdx-0x200]
  67e9c9:	40 88 b4 24 a0 00 00 	mov    BYTE PTR [rsp+0xa0],sil
  67e9d0:	00 
  67e9d1:	0f b6 b2 00 ff ff ff 	movzx  esi,BYTE PTR [rdx-0x100]
  67e9d8:	40 88 34 24          	mov    BYTE PTR [rsp],sil
  67e9dc:	0f b6 32             	movzx  esi,BYTE PTR [rdx]
  67e9df:	40 88 74 24 20       	mov    BYTE PTR [rsp+0x20],sil
  67e9e4:	0f b6 74 24 08       	movzx  esi,BYTE PTR [rsp+0x8]
  67e9e9:	40 88 74 08 f1       	mov    BYTE PTR [rax+rcx*1-0xf],sil
  67e9ee:	40 88 7c 08 f2       	mov    BYTE PTR [rax+rcx*1-0xe],dil
  67e9f3:	44 88 44 08 f3       	mov    BYTE PTR [rax+rcx*1-0xd],r8b
  67e9f8:	44 88 4c 08 f4       	mov    BYTE PTR [rax+rcx*1-0xc],r9b
  67e9fd:	44 88 54 08 f5       	mov    BYTE PTR [rax+rcx*1-0xb],r10b
  67ea02:	44 88 5c 08 f6       	mov    BYTE PTR [rax+rcx*1-0xa],r11b
  67ea07:	88 5c 08 f7          	mov    BYTE PTR [rax+rcx*1-0x9],bl
  67ea0b:	44 88 74 08 f8       	mov    BYTE PTR [rax+rcx*1-0x8],r14b
  67ea10:	44 88 7c 08 f9       	mov    BYTE PTR [rax+rcx*1-0x7],r15b
  67ea15:	44 88 64 08 fa       	mov    BYTE PTR [rax+rcx*1-0x6],r12b
  67ea1a:	44 88 6c 08 fb       	mov    BYTE PTR [rax+rcx*1-0x5],r13b
  67ea1f:	0f b6 74 24 10       	movzx  esi,BYTE PTR [rsp+0x10]
  67ea24:	40 88 74 08 fc       	mov    BYTE PTR [rax+rcx*1-0x4],sil
  67ea29:	0f b6 74 24 28       	movzx  esi,BYTE PTR [rsp+0x28]
  67ea2e:	40 88 74 08 fd       	mov    BYTE PTR [rax+rcx*1-0x3],sil
  67ea33:	0f b6 b4 24 a0 00 00 	movzx  esi,BYTE PTR [rsp+0xa0]
  67ea3a:	00 
  67ea3b:	40 88 74 08 fe       	mov    BYTE PTR [rax+rcx*1-0x2],sil
  67ea40:	0f b6 34 24          	movzx  esi,BYTE PTR [rsp]
  67ea44:	40 88 74 08 ff       	mov    BYTE PTR [rax+rcx*1-0x1],sil
  67ea49:	0f b6 74 24 20       	movzx  esi,BYTE PTR [rsp+0x20]
  67ea4e:	40 88 34 08          	mov    BYTE PTR [rax+rcx*1],sil
  67ea52:	48 83 c1 10          	add    rcx,0x10
  67ea56:	48 ff c2             	inc    rdx
  67ea59:	48 81 f9 ff 0f 00 00 	cmp    rcx,0xfff
  67ea60:	0f 85 ea fe ff ff    	jne    67e950 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x1490>
  67ea66:	48 8b 8c 24 f0 00 00 	mov    rcx,QWORD PTR [rsp+0xf0]
  67ea6d:	00 
  67ea6e:	48 c7 01 ff 00 00 00 	mov    QWORD PTR [rcx],0xff
  67ea75:	48 89 41 08          	mov    QWORD PTR [rcx+0x8],rax
  67ea79:	48 c7 41 10 ff 00 00 	mov    QWORD PTR [rcx+0x10],0xff
  67ea80:	00 
  67ea81:	48 8d 65 d8          	lea    rsp,[rbp-0x28]
  67ea85:	5b                   	pop    rbx
  67ea86:	41 5c                	pop    r12
  67ea88:	41 5d                	pop    r13
  67ea8a:	41 5e                	pop    r14
  67ea8c:	41 5f                	pop    r15
  67ea8e:	5d                   	pop    rbp
  67ea8f:	c3                   	ret    
  67ea90:	e8 31 f6 fd ff       	call   65e0c6 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate6domain6DomainE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.8267282299410697290>
  67ea95:	8b 05 8d b1 08 00    	mov    eax,DWORD PTR [rip+0x8b18d]        # 709c28 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290+0x930>
  67ea9b:	85 c0                	test   eax,eax
  67ea9d:	0f 84 a6 ea ff ff    	je     67d549 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x89>
  67eaa3:	e8 cc f5 fd ff       	call   65e074 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.8267282299410697290>
  67eaa8:	e9 9c ea ff ff       	jmp    67d549 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block+0x89>
  67eaad:	48 8d 15 8c 5a 08 00 	lea    rdx,[rip+0x85a8c]        # 704540 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x6c0>
  67eab4:	48 89 fe             	mov    rsi,rdi
  67eab7:	c5 f8 77             	vzeroupper 
  67eaba:	ff 15 20 89 08 00    	call   QWORD PTR [rip+0x88920]        # 7073e0 <_DYNAMIC+0x250>
  67eac0:	bf 01 00 00 00       	mov    edi,0x1
  67eac5:	be f0 0f 00 00       	mov    esi,0xff0
  67eaca:	ff 15 00 89 08 00    	call   QWORD PTR [rip+0x88900]        # 7073d0 <_DYNAMIC+0x240>

