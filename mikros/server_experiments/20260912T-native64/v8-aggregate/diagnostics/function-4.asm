000000000066a510 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_>:
  66a510:	55                   	push   rbp
  66a511:	48 89 e5             	mov    rbp,rsp
  66a514:	41 57                	push   r15
  66a516:	41 56                	push   r14
  66a518:	41 55                	push   r13
  66a51a:	41 54                	push   r12
  66a51c:	53                   	push   rbx
  66a51d:	48 83 e4 c0          	and    rsp,0xffffffffffffffc0
  66a521:	48 81 ec c0 0f 00 00 	sub    rsp,0xfc0
  66a528:	48 c7 04 24 00 00 00 	mov    QWORD PTR [rsp],0x0
  66a52f:	00 
  66a530:	48 81 ec 00 10 00 00 	sub    rsp,0x1000
  66a537:	48 c7 04 24 00 00 00 	mov    QWORD PTR [rsp],0x0
  66a53e:	00 
  66a53f:	48 81 ec 00 10 00 00 	sub    rsp,0x1000
  66a546:	48 c7 04 24 00 00 00 	mov    QWORD PTR [rsp],0x0
  66a54d:	00 
  66a54e:	48 81 ec 40 0b 00 00 	sub    rsp,0xb40
  66a555:	4c 8b 32             	mov    r14,QWORD PTR [rdx]
  66a558:	48 8b 42 08          	mov    rax,QWORD PTR [rdx+0x8]
  66a55c:	48 89 44 24 28       	mov    QWORD PTR [rsp+0x28],rax
  66a561:	48 8b 4a 10          	mov    rcx,QWORD PTR [rdx+0x10]
  66a565:	4c 8b 7a 18          	mov    r15,QWORD PTR [rdx+0x18]
  66a569:	48 8b 42 20          	mov    rax,QWORD PTR [rdx+0x20]
  66a56d:	48 89 44 24 70       	mov    QWORD PTR [rsp+0x70],rax
  66a572:	48 8b 42 28          	mov    rax,QWORD PTR [rdx+0x28]
  66a576:	48 89 84 24 c0 00 00 	mov    QWORD PTR [rsp+0xc0],rax
  66a57d:	00 
  66a57e:	40 0f b6 c6          	movzx  eax,sil
  66a582:	48 8d 15 b7 81 9a ff 	lea    rdx,[rip+0xffffffffff9a81b7]        # 12740 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x15f2>
  66a589:	48 63 04 82          	movsxd rax,DWORD PTR [rdx+rax*4]
  66a58d:	48 01 d0             	add    rax,rdx
  66a590:	ff e0                	jmp    rax
  66a592:	8b 05 90 f6 09 00    	mov    eax,DWORD PTR [rip+0x9f690]        # 709c28 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290+0x930>
  66a598:	85 c0                	test   eax,eax
  66a59a:	0f 85 e5 58 00 00    	jne    66fe85 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5975>
  66a5a0:	8b 05 aa 07 0a 00    	mov    eax,DWORD PTR [rip+0xa07aa]        # 70ad50 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1120>
  66a5a6:	85 c0                	test   eax,eax
  66a5a8:	0f 85 fc 58 00 00    	jne    66feaa <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x599a>
  66a5ae:	4c 89 74 24 60       	mov    QWORD PTR [rsp+0x60],r14
  66a5b3:	48 89 4c 24 30       	mov    QWORD PTR [rsp+0x30],rcx
  66a5b8:	48 89 7c 24 50       	mov    QWORD PTR [rsp+0x50],rdi
  66a5bd:	bf f0 0f 00 00       	mov    edi,0xff0
  66a5c2:	be 01 00 00 00       	mov    esi,0x1
  66a5c7:	ff 15 e3 cd 09 00    	call   QWORD PTR [rip+0x9cde3]        # 7073b0 <calloc@GLIBC_2.2.5>
  66a5cd:	48 89 44 24 68       	mov    QWORD PTR [rsp+0x68],rax
  66a5d2:	48 85 c0             	test   rax,rax
  66a5d5:	0f 84 10 5c 00 00    	je     6701eb <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5cdb>
  66a5db:	48 8b 44 24 28       	mov    rax,QWORD PTR [rsp+0x28]
  66a5e0:	48 c1 e8 04          	shr    rax,0x4
  66a5e4:	49 c1 ef 04          	shr    r15,0x4
  66a5e8:	49 39 c7             	cmp    r15,rax
  66a5eb:	4c 0f 43 f8          	cmovae r15,rax
  66a5ef:	48 8b 84 24 c0 00 00 	mov    rax,QWORD PTR [rsp+0xc0]
  66a5f6:	00 
  66a5f7:	4c 39 f8             	cmp    rax,r15
  66a5fa:	4c 0f 42 f8          	cmovb  r15,rax
  66a5fe:	4c 89 7c 24 38       	mov    QWORD PTR [rsp+0x38],r15
  66a603:	4d 85 ff             	test   r15,r15
  66a606:	0f 84 a8 03 00 00    	je     66a9b4 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4a4>
  66a60c:	4c 8d a4 24 e0 1a 00 	lea    r12,[rsp+0x1ae0]
  66a613:	00 
  66a614:	4c 8d ac 24 00 1c 00 	lea    r13,[rsp+0x1c00]
  66a61b:	00 
  66a61c:	45 31 f6             	xor    r14d,r14d
  66a61f:	4c 8b 3d 9a d1 09 00 	mov    r15,QWORD PTR [rip+0x9d19a]        # 7077c0 <_DYNAMIC+0x630>
  66a626:	b8 87 00 00 00       	mov    eax,0x87
  66a62b:	c4 61 f9 6e c0       	vmovq  xmm8,rax
  66a630:	c5 79 7f 84 24 c0 00 	vmovdqa XMMWORD PTR [rsp+0xc0],xmm8
  66a637:	00 00 
  66a639:	0f 1f 80 00 00 00 00 	nop    DWORD PTR [rax+0x0]
  66a640:	4c 89 f3             	mov    rbx,r14
  66a643:	48 c1 e3 04          	shl    rbx,0x4
  66a647:	48 8b 44 24 60       	mov    rax,QWORD PTR [rsp+0x60]
  66a64c:	48 8d 34 18          	lea    rsi,[rax+rbx*1]
  66a650:	48 8d bc 24 c0 12 00 	lea    rdi,[rsp+0x12c0]
  66a657:	00 
  66a658:	c5 f8 77             	vzeroupper 
  66a65b:	e8 70 a1 00 00       	call   6747d0 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate6domainNtB2_6Domain8eval_row>
  66a660:	48 8b 44 24 30       	mov    rax,QWORD PTR [rsp+0x30]
  66a665:	48 8d 34 18          	lea    rsi,[rax+rbx*1]
  66a669:	48 8d bc 24 c0 02 00 	lea    rdi,[rsp+0x2c0]
  66a670:	00 
  66a671:	e8 5a a1 00 00       	call   6747d0 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate6domainNtB2_6Domain8eval_row>
  66a676:	49 ff c6             	inc    r14
  66a679:	c5 fc 10 84 24 9f 13 	vmovups ymm0,YMMWORD PTR [rsp+0x139f]
  66a680:	00 00 
  66a682:	c4 c1 7c 11 84 24 df 	vmovups YMMWORD PTR [r12+0xdf],ymm0
  66a689:	00 00 00 
  66a68c:	c5 fc 10 84 24 80 13 	vmovups ymm0,YMMWORD PTR [rsp+0x1380]
  66a693:	00 00 
  66a695:	c4 c1 7c 11 84 24 c0 	vmovups YMMWORD PTR [r12+0xc0],ymm0
  66a69c:	00 00 00 
  66a69f:	c5 fc 10 84 24 60 13 	vmovups ymm0,YMMWORD PTR [rsp+0x1360]
  66a6a6:	00 00 
  66a6a8:	c4 c1 7c 11 84 24 a0 	vmovups YMMWORD PTR [r12+0xa0],ymm0
  66a6af:	00 00 00 
  66a6b2:	c5 fc 10 84 24 40 13 	vmovups ymm0,YMMWORD PTR [rsp+0x1340]
  66a6b9:	00 00 
  66a6bb:	c4 c1 7c 11 84 24 80 	vmovups YMMWORD PTR [r12+0x80],ymm0
  66a6c2:	00 00 00 
  66a6c5:	c5 fc 10 84 24 c0 12 	vmovups ymm0,YMMWORD PTR [rsp+0x12c0]
  66a6cc:	00 00 
  66a6ce:	c5 fc 10 8c 24 e0 12 	vmovups ymm1,YMMWORD PTR [rsp+0x12e0]
  66a6d5:	00 00 
  66a6d7:	c5 fc 10 94 24 00 13 	vmovups ymm2,YMMWORD PTR [rsp+0x1300]
  66a6de:	00 00 
  66a6e0:	c5 fc 10 9c 24 20 13 	vmovups ymm3,YMMWORD PTR [rsp+0x1320]
  66a6e7:	00 00 
  66a6e9:	c4 c1 7c 11 5c 24 60 	vmovups YMMWORD PTR [r12+0x60],ymm3
  66a6f0:	c4 c1 7c 11 54 24 40 	vmovups YMMWORD PTR [r12+0x40],ymm2
  66a6f7:	c4 c1 7c 11 4c 24 20 	vmovups YMMWORD PTR [r12+0x20],ymm1
  66a6fe:	c4 c1 7c 11 04 24    	vmovups YMMWORD PTR [r12],ymm0
  66a704:	48 8d 84 24 c1 16 00 	lea    rax,[rsp+0x16c1]
  66a70b:	00 
  66a70c:	c5 d8 57 e4          	vxorps xmm4,xmm4,xmm4
  66a710:	c5 f8 11 20          	vmovups XMMWORD PTR [rax],xmm4
  66a714:	c5 fc 10 84 24 9f 03 	vmovups ymm0,YMMWORD PTR [rsp+0x39f]
  66a71b:	00 00 
  66a71d:	c4 c1 7c 11 85 df 00 	vmovups YMMWORD PTR [r13+0xdf],ymm0
  66a724:	00 00 
  66a726:	c5 fc 10 84 24 80 03 	vmovups ymm0,YMMWORD PTR [rsp+0x380]
  66a72d:	00 00 
  66a72f:	c4 c1 7c 11 85 c0 00 	vmovups YMMWORD PTR [r13+0xc0],ymm0
  66a736:	00 00 
  66a738:	c5 fc 10 84 24 60 03 	vmovups ymm0,YMMWORD PTR [rsp+0x360]
  66a73f:	00 00 
  66a741:	c4 c1 7c 11 85 a0 00 	vmovups YMMWORD PTR [r13+0xa0],ymm0
  66a748:	00 00 
  66a74a:	c5 fc 10 84 24 40 03 	vmovups ymm0,YMMWORD PTR [rsp+0x340]
  66a751:	00 00 
  66a753:	c4 c1 7c 11 85 80 00 	vmovups YMMWORD PTR [r13+0x80],ymm0
  66a75a:	00 00 
  66a75c:	c5 fc 10 84 24 c0 02 	vmovups ymm0,YMMWORD PTR [rsp+0x2c0]
  66a763:	00 00 
  66a765:	c5 fc 10 8c 24 e0 02 	vmovups ymm1,YMMWORD PTR [rsp+0x2e0]
  66a76c:	00 00 
  66a76e:	c5 fc 10 94 24 00 03 	vmovups ymm2,YMMWORD PTR [rsp+0x300]
  66a775:	00 00 
  66a777:	c5 fc 10 9c 24 20 03 	vmovups ymm3,YMMWORD PTR [rsp+0x320]
  66a77e:	00 00 
  66a780:	c4 c1 7c 11 5d 60    	vmovups YMMWORD PTR [r13+0x60],ymm3
  66a786:	c4 c1 7c 11 55 40    	vmovups YMMWORD PTR [r13+0x40],ymm2
  66a78c:	c4 c1 7c 11 4d 20    	vmovups YMMWORD PTR [r13+0x20],ymm1
  66a792:	c4 c1 7c 11 45 00    	vmovups YMMWORD PTR [r13+0x0],ymm0
  66a798:	48 03 5c 24 70       	add    rbx,QWORD PTR [rsp+0x70]
  66a79d:	0f b6 84 24 d0 16 00 	movzx  eax,BYTE PTR [rsp+0x16d0]
  66a7a4:	00 
  66a7a5:	c5 f8 10 84 24 c0 16 	vmovups xmm0,XMMWORD PTR [rsp+0x16c0]
  66a7ac:	00 00 
  66a7ae:	c4 c1 78 11 84 24 ff 	vmovups XMMWORD PTR [r12+0xff],xmm0
  66a7b5:	00 00 00 
  66a7b8:	41 88 84 24 0f 01 00 	mov    BYTE PTR [r12+0x10f],al
  66a7bf:	00 
  66a7c0:	48 8d 84 24 01 02 00 	lea    rax,[rsp+0x201]
  66a7c7:	00 
  66a7c8:	c5 f8 11 20          	vmovups XMMWORD PTR [rax],xmm4
  66a7cc:	48 c7 84 24 f0 1b 00 	mov    QWORD PTR [rsp+0x1bf0],0x0
  66a7d3:	00 00 00 00 00 
  66a7d8:	48 c7 84 24 f8 1b 00 	mov    QWORD PTR [rsp+0x1bf8],0xff
  66a7df:	00 ff 00 00 00 
  66a7e4:	0f b6 84 24 10 02 00 	movzx  eax,BYTE PTR [rsp+0x210]
  66a7eb:	00 
  66a7ec:	41 88 84 24 2f 02 00 	mov    BYTE PTR [r12+0x22f],al
  66a7f3:	00 
  66a7f4:	c5 f8 10 84 24 00 02 	vmovups xmm0,XMMWORD PTR [rsp+0x200]
  66a7fb:	00 00 
  66a7fd:	c4 c1 78 11 84 24 1f 	vmovups XMMWORD PTR [r12+0x21f],xmm0
  66a804:	02 00 00 
  66a807:	48 8b 03             	mov    rax,QWORD PTR [rbx]
  66a80a:	48 8b 4b 08          	mov    rcx,QWORD PTR [rbx+0x8]
  66a80e:	c4 e1 f9 6e c0       	vmovq  xmm0,rax
  66a813:	c4 e1 f9 6e c9       	vmovq  xmm1,rcx
  66a818:	48 31 c1             	xor    rcx,rax
  66a81b:	c4 e1 f9 6e d1       	vmovq  xmm2,rcx
  66a820:	4c 89 e8             	mov    rax,r13
  66a823:	31 c9                	xor    ecx,ecx
  66a825:	4c 8b 54 24 68       	mov    r10,QWORD PTR [rsp+0x68]
  66a82a:	c5 79 6f 84 24 c0 00 	vmovdqa xmm8,XMMWORD PTR [rsp+0xc0]
  66a831:	00 00 
  66a833:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  66a83a:	84 00 00 00 00 00 
  66a840:	0f b6 90 e0 fe ff ff 	movzx  edx,BYTE PTR [rax-0x120]
  66a847:	c1 e2 04             	shl    edx,0x4
  66a84a:	49 8b 34 17          	mov    rsi,QWORD PTR [r15+rdx*1]
  66a84e:	49 8b 54 17 08       	mov    rdx,QWORD PTR [r15+rdx*1+0x8]
  66a853:	c4 e1 f9 6e de       	vmovq  xmm3,rsi
  66a858:	c4 e3 79 44 db 00    	vpclmullqlqdq xmm3,xmm0,xmm3
  66a85e:	c4 e1 f9 6e e2       	vmovq  xmm4,rdx
  66a863:	c4 e3 71 44 e4 00    	vpclmullqlqdq xmm4,xmm1,xmm4
  66a869:	48 31 f2             	xor    rdx,rsi
  66a86c:	c4 e1 f9 6e ea       	vmovq  xmm5,rdx
  66a871:	c4 e3 69 44 ed 00    	vpclmullqlqdq xmm5,xmm2,xmm5
  66a877:	62 f3 e5 08 25 ec 96 	vpternlogq xmm5,xmm3,xmm4,0x96
  66a87e:	c5 f9 70 f4 ee       	vpshufd xmm6,xmm4,0xee
  66a883:	c4 c3 49 44 f0 00    	vpclmullqlqdq xmm6,xmm6,xmm8
  66a889:	c5 f9 70 fd ee       	vpshufd xmm7,xmm5,0xee
  66a88e:	c5 c1 ef e4          	vpxor  xmm4,xmm7,xmm4
  66a892:	0f b6 10             	movzx  edx,BYTE PTR [rax]
  66a895:	c4 c3 59 44 e0 00    	vpclmullqlqdq xmm4,xmm4,xmm8
  66a89b:	c4 e3 f9 16 f6 01    	vpextrq rsi,xmm6,0x1
  66a8a1:	48 8d 3c 36          	lea    rdi,[rsi+rsi*1]
  66a8a5:	4c 8d 04 b5 00 00 00 	lea    r8,[rsi*4+0x0]
  66a8ac:	00 
  66a8ad:	49 89 f1             	mov    r9,rsi
  66a8b0:	49 31 f8             	xor    r8,rdi
  66a8b3:	49 c1 e1 07          	shl    r9,0x7
  66a8b7:	4d 31 c1             	xor    r9,r8
  66a8ba:	c5 e1 ef dc          	vpxor  xmm3,xmm3,xmm4
  66a8be:	c4 e1 f9 7e df       	vmovq  rdi,xmm3
  66a8c3:	c5 f9 70 db ee       	vpshufd xmm3,xmm3,0xee
  66a8c8:	4c 31 cf             	xor    rdi,r9
  66a8cb:	62 f3 d5 08 25 de 96 	vpternlogq xmm3,xmm5,xmm6,0x96
  66a8d2:	c4 c1 f9 7e d8       	vmovq  r8,xmm3
  66a8d7:	c1 e2 04             	shl    edx,0x4
  66a8da:	4d 8b 0c 17          	mov    r9,QWORD PTR [r15+rdx*1]
  66a8de:	49 8b 54 17 08       	mov    rdx,QWORD PTR [r15+rdx*1+0x8]
  66a8e3:	48 31 f7             	xor    rdi,rsi
  66a8e6:	c4 e1 f9 6e e7       	vmovq  xmm4,rdi
  66a8eb:	c4 c1 f9 6e e9       	vmovq  xmm5,r9
  66a8f0:	c4 e1 f9 6e f2       	vmovq  xmm6,rdx
  66a8f5:	c4 e3 61 44 de 00    	vpclmullqlqdq xmm3,xmm3,xmm6
  66a8fb:	49 31 f8             	xor    r8,rdi
  66a8fe:	c4 e3 59 44 e5 00    	vpclmullqlqdq xmm4,xmm4,xmm5
  66a904:	4c 31 ca             	xor    rdx,r9
  66a907:	c4 c1 f9 6e e8       	vmovq  xmm5,r8
  66a90c:	c4 e1 f9 6e f2       	vmovq  xmm6,rdx
  66a911:	c4 e3 51 44 ee 00    	vpclmullqlqdq xmm5,xmm5,xmm6
  66a917:	c5 f9 70 f3 ee       	vpshufd xmm6,xmm3,0xee
  66a91c:	62 f3 dd 08 25 eb 96 	vpternlogq xmm5,xmm4,xmm3,0x96
  66a923:	c4 c3 49 44 f0 00    	vpclmullqlqdq xmm6,xmm6,xmm8
  66a929:	c5 f9 70 fd ee       	vpshufd xmm7,xmm5,0xee
  66a92e:	c5 c1 ef db          	vpxor  xmm3,xmm7,xmm3
  66a932:	c4 c3 61 44 d8 00    	vpclmullqlqdq xmm3,xmm3,xmm8
  66a938:	c4 e3 f9 16 f2 01    	vpextrq rdx,xmm6,0x1
  66a93e:	48 8d 34 12          	lea    rsi,[rdx+rdx*1]
  66a942:	48 c1 e2 02          	shl    rdx,0x2
  66a946:	48 31 f2             	xor    rdx,rsi
  66a949:	c5 d9 ef fb          	vpxor  xmm7,xmm4,xmm3
  66a94d:	c4 e1 f9 7e fe       	vmovq  rsi,xmm7
  66a952:	48 31 d6             	xor    rsi,rdx
  66a955:	c5 f9 70 e4 ee       	vpshufd xmm4,xmm4,0xee
  66a95a:	c5 c1 73 f6 07       	vpsllq xmm7,xmm6,0x7
  66a95f:	c5 c9 ef e4          	vpxor  xmm4,xmm6,xmm4
  66a963:	c4 e2 79 59 ed       	vpbroadcastq xmm5,xmm5
  66a968:	c4 e3 d1 22 ee 00    	vpinsrq xmm5,xmm5,rsi,0x0
  66a96e:	c4 e3 59 0f e7 08    	vpalignr xmm4,xmm4,xmm7,0x8
  66a974:	c4 c1 79 6f 3c 0a    	vmovdqa xmm7,XMMWORD PTR [r10+rcx*1]
  66a97a:	c4 e3 41 02 db 0c    	vpblendd xmm3,xmm7,xmm3,0xc
  66a980:	c5 c9 6d f7          	vpunpckhqdq xmm6,xmm6,xmm7
  66a984:	c5 d9 ef e5          	vpxor  xmm4,xmm4,xmm5
  66a988:	62 f3 dd 08 25 f3 96 	vpternlogq xmm6,xmm4,xmm3,0x96
  66a98f:	c4 c1 79 7f 34 0a    	vmovdqa XMMWORD PTR [r10+rcx*1],xmm6
  66a995:	48 83 c1 10          	add    rcx,0x10
  66a999:	48 ff c0             	inc    rax
  66a99c:	48 81 f9 f0 0f 00 00 	cmp    rcx,0xff0
  66a9a3:	0f 85 97 fe ff ff    	jne    66a840 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x330>
  66a9a9:	4c 3b 74 24 38       	cmp    r14,QWORD PTR [rsp+0x38]
  66a9ae:	0f 85 8c fc ff ff    	jne    66a640 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x130>
  66a9b4:	bf f0 0f 00 00       	mov    edi,0xff0
  66a9b9:	c5 f8 77             	vzeroupper 
  66a9bc:	ff 15 c6 c9 09 00    	call   QWORD PTR [rip+0x9c9c6]        # 707388 <malloc@GLIBC_2.2.5>
  66a9c2:	48 85 c0             	test   rax,rax
  66a9c5:	0f 84 40 58 00 00    	je     67020b <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5cfb>
  66a9cb:	49 89 c6             	mov    r14,rax
  66a9ce:	48 8b 3d 3b e9 09 00 	mov    rdi,QWORD PTR [rip+0x9e93b]        # 709310 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290+0x18>
  66a9d5:	48 83 ff 10          	cmp    rdi,0x10
  66a9d9:	0f 82 3e 58 00 00    	jb     67021d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5d0d>
  66a9df:	48 8b 05 22 e9 09 00 	mov    rax,QWORD PTR [rip+0x9e922]        # 709308 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290+0x10>
  66a9e6:	48 8d 90 00 d0 00 00 	lea    rdx,[rax+0xd000]
  66a9ed:	48 8d b0 00 e0 00 00 	lea    rsi,[rax+0xe000]
  66a9f4:	4c 8d a8 00 f0 00 00 	lea    r13,[rax+0xf000]
  66a9fb:	45 31 c0             	xor    r8d,r8d
  66a9fe:	48 8b 7c 24 68       	mov    rdi,QWORD PTR [rsp+0x68]
  66aa03:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  66aa0a:	84 00 00 00 00 00 
  66aa10:	4e 8b 0c 07          	mov    r9,QWORD PTR [rdi+r8*1]
  66aa14:	4d 89 cb             	mov    r11,r9
  66aa17:	4d 89 ca             	mov    r10,r9
  66aa1a:	4d 89 cf             	mov    r15,r9
  66aa1d:	4d 89 cc             	mov    r12,r9
  66aa20:	44 89 cb             	mov    ebx,r9d
  66aa23:	c1 eb 04             	shr    ebx,0x4
  66aa26:	81 e3 f0 0f 00 00    	and    ebx,0xff0
  66aa2c:	48 8d 88 00 10 00 00 	lea    rcx,[rax+0x1000]
  66aa33:	c5 f9 6f 04 19       	vmovdqa xmm0,XMMWORD PTR [rcx+rbx*1]
  66aa38:	41 0f b6 d9          	movzx  ebx,r9b
  66aa3c:	c1 e3 04             	shl    ebx,0x4
  66aa3f:	c5 f9 ef 04 18       	vpxor  xmm0,xmm0,XMMWORD PTR [rax+rbx*1]
  66aa44:	44 89 cb             	mov    ebx,r9d
  66aa47:	c1 eb 0c             	shr    ebx,0xc
  66aa4a:	81 e3 f0 0f 00 00    	and    ebx,0xff0
  66aa50:	41 c1 e9 14          	shr    r9d,0x14
  66aa54:	41 83 e1 f0          	and    r9d,0xfffffff0
  66aa58:	48 8d 88 00 30 00 00 	lea    rcx,[rax+0x3000]
  66aa5f:	c4 a1 79 6f 0c 09    	vmovdqa xmm1,XMMWORD PTR [rcx+r9*1]
  66aa65:	48 8d 88 00 20 00 00 	lea    rcx,[rax+0x2000]
  66aa6c:	62 f3 fd 08 25 0c 19 	vpternlogq xmm1,xmm0,XMMWORD PTR [rcx+rbx*1],0x96
  66aa73:	96 
  66aa74:	4e 8b 4c 07 08       	mov    r9,QWORD PTR [rdi+r8*1+0x8]
  66aa79:	4c 89 cb             	mov    rbx,r9
  66aa7c:	49 c1 ef 28          	shr    r15,0x28
  66aa80:	45 0f b6 ff          	movzx  r15d,r15b
  66aa84:	41 c1 e7 04          	shl    r15d,0x4
  66aa88:	48 8d 88 00 50 00 00 	lea    rcx,[rax+0x5000]
  66aa8f:	c4 a1 79 6f 04 39    	vmovdqa xmm0,XMMWORD PTR [rcx+r15*1]
  66aa95:	4d 89 cf             	mov    r15,r9
  66aa98:	49 c1 ec 20          	shr    r12,0x20
  66aa9c:	45 0f b6 e4          	movzx  r12d,r12b
  66aaa0:	41 c1 e4 04          	shl    r12d,0x4
  66aaa4:	48 8d 88 00 40 00 00 	lea    rcx,[rax+0x4000]
  66aaab:	62 b3 f5 08 25 04 21 	vpternlogq xmm0,xmm1,XMMWORD PTR [rcx+r12*1],0x96
  66aab2:	96 
  66aab3:	4d 89 cc             	mov    r12,r9
  66aab6:	49 c1 eb 38          	shr    r11,0x38
  66aaba:	41 c1 e3 04          	shl    r11d,0x4
  66aabe:	48 8d 88 00 70 00 00 	lea    rcx,[rax+0x7000]
  66aac5:	c4 a1 79 6f 0c 19    	vmovdqa xmm1,XMMWORD PTR [rcx+r11*1]
  66aacb:	4d 89 cb             	mov    r11,r9
  66aace:	49 c1 ea 30          	shr    r10,0x30
  66aad2:	45 0f b6 d2          	movzx  r10d,r10b
  66aad6:	41 c1 e2 04          	shl    r10d,0x4
  66aada:	48 8d 88 00 60 00 00 	lea    rcx,[rax+0x6000]
  66aae1:	62 b3 fd 08 25 0c 11 	vpternlogq xmm1,xmm0,XMMWORD PTR [rcx+r10*1],0x96
  66aae8:	96 
  66aae9:	45 89 ca             	mov    r10d,r9d
  66aaec:	41 c1 ea 04          	shr    r10d,0x4
  66aaf0:	41 81 e2 f0 0f 00 00 	and    r10d,0xff0
  66aaf7:	48 8d 88 00 90 00 00 	lea    rcx,[rax+0x9000]
  66aafe:	c4 a1 79 6f 04 11    	vmovdqa xmm0,XMMWORD PTR [rcx+r10*1]
  66ab04:	45 0f b6 d1          	movzx  r10d,r9b
  66ab08:	41 c1 e2 04          	shl    r10d,0x4
  66ab0c:	48 8d 88 00 80 00 00 	lea    rcx,[rax+0x8000]
  66ab13:	62 b3 f5 08 25 04 11 	vpternlogq xmm0,xmm1,XMMWORD PTR [rcx+r10*1],0x96
  66ab1a:	96 
  66ab1b:	45 89 ca             	mov    r10d,r9d
  66ab1e:	41 c1 e9 14          	shr    r9d,0x14
  66ab22:	41 83 e1 f0          	and    r9d,0xfffffff0
  66ab26:	48 8d 88 00 b0 00 00 	lea    rcx,[rax+0xb000]
  66ab2d:	c4 a1 79 6f 0c 09    	vmovdqa xmm1,XMMWORD PTR [rcx+r9*1]
  66ab33:	41 c1 ea 0c          	shr    r10d,0xc
  66ab37:	41 81 e2 f0 0f 00 00 	and    r10d,0xff0
  66ab3e:	48 8d 88 00 a0 00 00 	lea    rcx,[rax+0xa000]
  66ab45:	62 b3 fd 08 25 0c 11 	vpternlogq xmm1,xmm0,XMMWORD PTR [rcx+r10*1],0x96
  66ab4c:	96 
  66ab4d:	49 c1 ec 28          	shr    r12,0x28
  66ab51:	45 0f b6 cc          	movzx  r9d,r12b
  66ab55:	41 c1 e1 04          	shl    r9d,0x4
  66ab59:	c4 a1 79 6f 04 0a    	vmovdqa xmm0,XMMWORD PTR [rdx+r9*1]
  66ab5f:	49 c1 eb 20          	shr    r11,0x20
  66ab63:	45 0f b6 cb          	movzx  r9d,r11b
  66ab67:	41 c1 e1 04          	shl    r9d,0x4
  66ab6b:	48 8d 88 00 c0 00 00 	lea    rcx,[rax+0xc000]
  66ab72:	62 b3 f5 08 25 04 09 	vpternlogq xmm0,xmm1,XMMWORD PTR [rcx+r9*1],0x96
  66ab79:	96 
  66ab7a:	48 c1 eb 38          	shr    rbx,0x38
  66ab7e:	c1 e3 04             	shl    ebx,0x4
  66ab81:	c4 c1 79 6f 4c 1d 00 	vmovdqa xmm1,XMMWORD PTR [r13+rbx*1+0x0]
  66ab88:	49 c1 ef 30          	shr    r15,0x30
  66ab8c:	45 0f b6 cf          	movzx  r9d,r15b
  66ab90:	41 c1 e1 04          	shl    r9d,0x4
  66ab94:	62 b3 fd 08 25 0c 0e 	vpternlogq xmm1,xmm0,XMMWORD PTR [rsi+r9*1],0x96
  66ab9b:	96 
  66ab9c:	c4 81 7a 7f 0c 06    	vmovdqu XMMWORD PTR [r14+r8*1],xmm1
  66aba2:	49 83 c0 10          	add    r8,0x10
  66aba6:	49 81 f8 f0 0f 00 00 	cmp    r8,0xff0
  66abad:	0f 85 5d fe ff ff    	jne    66aa10 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x500>
  66abb3:	e9 fe 09 00 00       	jmp    66b5b6 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x10a6>
  66abb8:	48 89 4c 24 30       	mov    QWORD PTR [rsp+0x30],rcx
  66abbd:	8b 05 65 f0 09 00    	mov    eax,DWORD PTR [rip+0x9f065]        # 709c28 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290+0x930>
  66abc3:	85 c0                	test   eax,eax
  66abc5:	0f 85 fb 52 00 00    	jne    66fec6 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x59b6>
  66abcb:	48 89 fb             	mov    rbx,rdi
  66abce:	8b 05 7c 01 0a 00    	mov    eax,DWORD PTR [rip+0xa017c]        # 70ad50 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1120>
  66abd4:	85 c0                	test   eax,eax
  66abd6:	0f 85 06 53 00 00    	jne    66fee2 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x59d2>
  66abdc:	4c 89 74 24 60       	mov    QWORD PTR [rsp+0x60],r14
  66abe1:	bf f0 0f 00 00       	mov    edi,0xff0
  66abe6:	be 01 00 00 00       	mov    esi,0x1
  66abeb:	ff 15 bf c7 09 00    	call   QWORD PTR [rip+0x9c7bf]        # 7073b0 <calloc@GLIBC_2.2.5>
  66abf1:	48 85 c0             	test   rax,rax
  66abf4:	0f 84 01 56 00 00    	je     6701fb <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5ceb>
  66abfa:	48 89 44 24 68       	mov    QWORD PTR [rsp+0x68],rax
  66abff:	48 89 5c 24 50       	mov    QWORD PTR [rsp+0x50],rbx
  66ac04:	48 8d bc 24 c0 1a 00 	lea    rdi,[rsp+0x1ac0]
  66ac0b:	00 
  66ac0c:	ba 00 10 00 00       	mov    edx,0x1000
  66ac11:	31 f6                	xor    esi,esi
  66ac13:	ff 15 9f c7 09 00    	call   QWORD PTR [rip+0x9c79f]        # 7073b8 <memset@GLIBC_2.2.5>
  66ac19:	48 8b 44 24 28       	mov    rax,QWORD PTR [rsp+0x28]
  66ac1e:	48 c1 e8 04          	shr    rax,0x4
  66ac22:	49 c1 ef 04          	shr    r15,0x4
  66ac26:	49 39 c7             	cmp    r15,rax
  66ac29:	4c 0f 43 f8          	cmovae r15,rax
  66ac2d:	48 8b 84 24 c0 00 00 	mov    rax,QWORD PTR [rsp+0xc0]
  66ac34:	00 
  66ac35:	4c 39 f8             	cmp    rax,r15
  66ac38:	4c 0f 42 f8          	cmovb  r15,rax
  66ac3c:	4d 85 ff             	test   r15,r15
  66ac3f:	0f 84 3e 09 00 00    	je     66b583 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1073>
  66ac45:	45 31 e4             	xor    r12d,r12d
  66ac48:	48 8d 05 21 93 09 00 	lea    rax,[rip+0x99321]        # 703f70 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0xf0>
  66ac4f:	48 89 84 24 b8 01 00 	mov    QWORD PTR [rsp+0x1b8],rax
  66ac56:	00 
  66ac57:	4c 89 7c 24 38       	mov    QWORD PTR [rsp+0x38],r15
  66ac5c:	0f 1f 40 00          	nop    DWORD PTR [rax+0x0]
  66ac60:	4d 89 e7             	mov    r15,r12
  66ac63:	49 c1 e7 04          	shl    r15,0x4
  66ac67:	48 8b 44 24 70       	mov    rax,QWORD PTR [rsp+0x70]
  66ac6c:	4e 8b 34 38          	mov    r14,QWORD PTR [rax+r15*1]
  66ac70:	4a 8b 5c 38 08       	mov    rbx,QWORD PTR [rax+r15*1+0x8]
  66ac75:	48 8b 44 24 60       	mov    rax,QWORD PTR [rsp+0x60]
  66ac7a:	4a 8d 34 38          	lea    rsi,[rax+r15*1]
  66ac7e:	48 8d bc 24 c0 16 00 	lea    rdi,[rsp+0x16c0]
  66ac85:	00 
  66ac86:	c5 f8 77             	vzeroupper 
  66ac89:	e8 42 9b 00 00       	call   6747d0 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate6domainNtB2_6Domain8eval_row>
  66ac8e:	4c 03 7c 24 30       	add    r15,QWORD PTR [rsp+0x30]
  66ac93:	48 8d bc 24 c0 12 00 	lea    rdi,[rsp+0x12c0]
  66ac9a:	00 
  66ac9b:	4c 89 fe             	mov    rsi,r15
  66ac9e:	e8 2d 9b 00 00       	call   6747d0 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate6domainNtB2_6Domain8eval_row>
  66aca3:	48 8b 3d 66 e6 09 00 	mov    rdi,QWORD PTR [rip+0x9e666]        # 709310 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290+0x18>
  66acaa:	48 83 ff 10          	cmp    rdi,0x10
  66acae:	0f 82 10 55 00 00    	jb     6701c4 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5cb4>
  66acb4:	49 ff c4             	inc    r12
  66acb7:	4c 89 a4 24 c8 01 00 	mov    QWORD PTR [rsp+0x1c8],r12
  66acbe:	00 
  66acbf:	48 89 d8             	mov    rax,rbx
  66acc2:	48 c1 e8 38          	shr    rax,0x38
  66acc6:	48 89 d9             	mov    rcx,rbx
  66acc9:	48 c1 e9 30          	shr    rcx,0x30
  66accd:	48 89 de             	mov    rsi,rbx
  66acd0:	48 c1 ee 28          	shr    rsi,0x28
  66acd4:	48 89 df             	mov    rdi,rbx
  66acd7:	4d 89 f0             	mov    r8,r14
  66acda:	49 c1 e8 38          	shr    r8,0x38
  66acde:	4d 89 f1             	mov    r9,r14
  66ace1:	49 c1 e9 30          	shr    r9,0x30
  66ace5:	4d 89 f2             	mov    r10,r14
  66ace8:	49 c1 ea 28          	shr    r10,0x28
  66acec:	4d 89 f3             	mov    r11,r14
  66acef:	48 8b 15 12 e6 09 00 	mov    rdx,QWORD PTR [rip+0x9e612]        # 709308 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290+0x10>
  66acf6:	45 0f b6 fe          	movzx  r15d,r14b
  66acfa:	41 c1 e7 04          	shl    r15d,0x4
  66acfe:	45 89 f4             	mov    r12d,r14d
  66ad01:	41 c1 ec 04          	shr    r12d,0x4
  66ad05:	41 81 e4 f0 0f 00 00 	and    r12d,0xff0
  66ad0c:	c4 a1 79 6f 84 22 00 	vmovdqa xmm0,XMMWORD PTR [rdx+r12*1+0x1000]
  66ad13:	10 00 00 
  66ad16:	c4 a1 79 ef 04 3a    	vpxor  xmm0,xmm0,XMMWORD PTR [rdx+r15*1]
  66ad1c:	45 89 f7             	mov    r15d,r14d
  66ad1f:	41 c1 ef 0c          	shr    r15d,0xc
  66ad23:	41 81 e7 f0 0f 00 00 	and    r15d,0xff0
  66ad2a:	41 c1 ee 14          	shr    r14d,0x14
  66ad2e:	41 83 e6 f0          	and    r14d,0xfffffff0
  66ad32:	c4 a1 79 6f 8c 32 00 	vmovdqa xmm1,XMMWORD PTR [rdx+r14*1+0x3000]
  66ad39:	30 00 00 
  66ad3c:	62 b3 fd 08 25 8c 3a 	vpternlogq xmm1,xmm0,XMMWORD PTR [rdx+r15*1+0x2000],0x96
  66ad43:	00 20 00 00 96 
  66ad48:	49 c1 eb 20          	shr    r11,0x20
  66ad4c:	45 0f b6 db          	movzx  r11d,r11b
  66ad50:	45 0f b6 d2          	movzx  r10d,r10b
  66ad54:	41 c1 e2 04          	shl    r10d,0x4
  66ad58:	c4 a1 79 6f 84 12 00 	vmovdqa xmm0,XMMWORD PTR [rdx+r10*1+0x5000]
  66ad5f:	50 00 00 
  66ad62:	41 c1 e3 04          	shl    r11d,0x4
  66ad66:	62 b3 f5 08 25 84 1a 	vpternlogq xmm0,xmm1,XMMWORD PTR [rdx+r11*1+0x4000],0x96
  66ad6d:	00 40 00 00 96 
  66ad72:	45 0f b6 c9          	movzx  r9d,r9b
  66ad76:	41 c1 e1 04          	shl    r9d,0x4
  66ad7a:	41 c1 e0 04          	shl    r8d,0x4
  66ad7e:	c4 a1 79 6f 8c 02 00 	vmovdqa xmm1,XMMWORD PTR [rdx+r8*1+0x7000]
  66ad85:	70 00 00 
  66ad88:	62 b3 fd 08 25 8c 0a 	vpternlogq xmm1,xmm0,XMMWORD PTR [rdx+r9*1+0x6000],0x96
  66ad8f:	00 60 00 00 96 
  66ad94:	44 0f b6 c3          	movzx  r8d,bl
  66ad98:	41 c1 e0 04          	shl    r8d,0x4
  66ad9c:	41 89 d9             	mov    r9d,ebx
  66ad9f:	41 c1 e9 04          	shr    r9d,0x4
  66ada3:	41 81 e1 f0 0f 00 00 	and    r9d,0xff0
  66adaa:	c4 a1 79 6f 84 0a 00 	vmovdqa xmm0,XMMWORD PTR [rdx+r9*1+0x9000]
  66adb1:	90 00 00 
  66adb4:	62 b3 f5 08 25 84 02 	vpternlogq xmm0,xmm1,XMMWORD PTR [rdx+r8*1+0x8000],0x96
  66adbb:	00 80 00 00 96 
  66adc0:	48 c1 ef 20          	shr    rdi,0x20
  66adc4:	41 89 d8             	mov    r8d,ebx
  66adc7:	41 c1 e8 0c          	shr    r8d,0xc
  66adcb:	c1 eb 14             	shr    ebx,0x14
  66adce:	83 e3 f0             	and    ebx,0xfffffff0
  66add1:	c5 f9 6f 8c 1a 00 b0 	vmovdqa xmm1,XMMWORD PTR [rdx+rbx*1+0xb000]
  66add8:	00 00 
  66adda:	41 81 e0 f0 0f 00 00 	and    r8d,0xff0
  66ade1:	62 b3 fd 08 25 8c 02 	vpternlogq xmm1,xmm0,XMMWORD PTR [rdx+r8*1+0xa000],0x96
  66ade8:	00 a0 00 00 96 
  66aded:	40 0f b6 ff          	movzx  edi,dil
  66adf1:	c1 e7 04             	shl    edi,0x4
  66adf4:	40 0f b6 f6          	movzx  esi,sil
  66adf8:	c1 e6 04             	shl    esi,0x4
  66adfb:	c5 f9 6f 84 32 00 d0 	vmovdqa xmm0,XMMWORD PTR [rdx+rsi*1+0xd000]
  66ae02:	00 00 
  66ae04:	62 f3 f5 08 25 84 3a 	vpternlogq xmm0,xmm1,XMMWORD PTR [rdx+rdi*1+0xc000],0x96
  66ae0b:	00 c0 00 00 96 
  66ae10:	0f b6 c9             	movzx  ecx,cl
  66ae13:	c1 e1 04             	shl    ecx,0x4
  66ae16:	c1 e0 04             	shl    eax,0x4
  66ae19:	c5 f9 6f 8c 02 00 f0 	vmovdqa xmm1,XMMWORD PTR [rdx+rax*1+0xf000]
  66ae20:	00 00 
  66ae22:	62 f3 fd 08 25 8c 0a 	vpternlogq xmm1,xmm0,XMMWORD PTR [rdx+rcx*1+0xe000],0x96
  66ae29:	00 e0 00 00 96 
  66ae2e:	c4 e3 f9 16 c9 01    	vpextrq rcx,xmm1,0x1
  66ae34:	c4 e1 f9 7e ca       	vmovq  rdx,xmm1
  66ae39:	89 d0                	mov    eax,edx
  66ae3b:	c1 e0 08             	shl    eax,0x8
  66ae3e:	0f b7 c0             	movzx  eax,ax
  66ae41:	48 89 44 24 28       	mov    QWORD PTR [rsp+0x28],rax
  66ae46:	89 d0                	mov    eax,edx
  66ae48:	25 00 ff 00 00       	and    eax,0xff00
  66ae4d:	48 89 44 24 58       	mov    QWORD PTR [rsp+0x58],rax
  66ae52:	89 d0                	mov    eax,edx
  66ae54:	c1 e8 08             	shr    eax,0x8
  66ae57:	25 00 ff 00 00       	and    eax,0xff00
  66ae5c:	48 89 84 24 80 00 00 	mov    QWORD PTR [rsp+0x80],rax
  66ae63:	00 
  66ae64:	89 d0                	mov    eax,edx
  66ae66:	c1 e8 10             	shr    eax,0x10
  66ae69:	25 00 ff 00 00       	and    eax,0xff00
  66ae6e:	48 89 84 24 98 00 00 	mov    QWORD PTR [rsp+0x98],rax
  66ae75:	00 
  66ae76:	48 89 d0             	mov    rax,rdx
  66ae79:	48 c1 e8 18          	shr    rax,0x18
  66ae7d:	25 00 ff 00 00       	and    eax,0xff00
  66ae82:	48 89 84 24 88 00 00 	mov    QWORD PTR [rsp+0x88],rax
  66ae89:	00 
  66ae8a:	48 89 d0             	mov    rax,rdx
  66ae8d:	48 c1 e8 20          	shr    rax,0x20
  66ae91:	25 00 ff 00 00       	and    eax,0xff00
  66ae96:	48 89 44 24 78       	mov    QWORD PTR [rsp+0x78],rax
  66ae9b:	48 89 d0             	mov    rax,rdx
  66ae9e:	48 c1 e8 28          	shr    rax,0x28
  66aea2:	25 00 ff 00 00       	and    eax,0xff00
  66aea7:	48 89 84 24 90 00 00 	mov    QWORD PTR [rsp+0x90],rax
  66aeae:	00 
  66aeaf:	48 c1 ea 30          	shr    rdx,0x30
  66aeb3:	81 e2 00 ff 00 00    	and    edx,0xff00
  66aeb9:	48 89 94 24 20 01 00 	mov    QWORD PTR [rsp+0x120],rdx
  66aec0:	00 
  66aec1:	89 c8                	mov    eax,ecx
  66aec3:	c1 e0 08             	shl    eax,0x8
  66aec6:	0f b7 c0             	movzx  eax,ax
  66aec9:	48 89 84 24 30 01 00 	mov    QWORD PTR [rsp+0x130],rax
  66aed0:	00 
  66aed1:	89 c8                	mov    eax,ecx
  66aed3:	25 00 ff 00 00       	and    eax,0xff00
  66aed8:	48 89 84 24 28 01 00 	mov    QWORD PTR [rsp+0x128],rax
  66aedf:	00 
  66aee0:	89 c8                	mov    eax,ecx
  66aee2:	c1 e8 08             	shr    eax,0x8
  66aee5:	25 00 ff 00 00       	and    eax,0xff00
  66aeea:	48 89 84 24 38 01 00 	mov    QWORD PTR [rsp+0x138],rax
  66aef1:	00 
  66aef2:	89 c8                	mov    eax,ecx
  66aef4:	c1 e8 10             	shr    eax,0x10
  66aef7:	25 00 ff 00 00       	and    eax,0xff00
  66aefc:	48 89 84 24 f8 01 00 	mov    QWORD PTR [rsp+0x1f8],rax
  66af03:	00 
  66af04:	48 89 c8             	mov    rax,rcx
  66af07:	48 c1 e8 18          	shr    rax,0x18
  66af0b:	25 00 ff 00 00       	and    eax,0xff00
  66af10:	48 89 84 24 f0 01 00 	mov    QWORD PTR [rsp+0x1f0],rax
  66af17:	00 
  66af18:	48 89 c8             	mov    rax,rcx
  66af1b:	48 c1 e8 20          	shr    rax,0x20
  66af1f:	25 00 ff 00 00       	and    eax,0xff00
  66af24:	48 89 84 24 e8 01 00 	mov    QWORD PTR [rsp+0x1e8],rax
  66af2b:	00 
  66af2c:	48 89 c8             	mov    rax,rcx
  66af2f:	48 c1 e8 28          	shr    rax,0x28
  66af33:	25 00 ff 00 00       	and    eax,0xff00
  66af38:	48 89 84 24 e0 01 00 	mov    QWORD PTR [rsp+0x1e0],rax
  66af3f:	00 
  66af40:	48 c1 e9 30          	shr    rcx,0x30
  66af44:	81 e1 00 ff 00 00    	and    ecx,0xff00
  66af4a:	48 89 8c 24 d8 01 00 	mov    QWORD PTR [rsp+0x1d8],rcx
  66af51:	00 
  66af52:	48 8b 35 a7 e3 09 00 	mov    rsi,QWORD PTR [rip+0x9e3a7]        # 709300 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290+0x8>
  66af59:	48 8b 1d 98 e3 09 00 	mov    rbx,QWORD PTR [rip+0x9e398]        # 7092f8 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290>
  66af60:	4c 8d a4 24 c0 02 00 	lea    r12,[rsp+0x2c0]
  66af67:	00 
  66af68:	31 c9                	xor    ecx,ecx
  66af6a:	48 89 b4 24 c0 01 00 	mov    QWORD PTR [rsp+0x1c0],rsi
  66af71:	00 
  66af72:	48 89 9c 24 d0 01 00 	mov    QWORD PTR [rsp+0x1d0],rbx
  66af79:	00 
  66af7a:	66 0f 1f 44 00 00    	nop    WORD PTR [rax+rax*1+0x0]
  66af80:	b0 01                	mov    al,0x1
  66af82:	d2 e0                	shl    al,cl
  66af84:	44 0f b6 c8          	movzx  r9d,al
  66af88:	48 8b 7c 24 28       	mov    rdi,QWORD PTR [rsp+0x28]
  66af8d:	4c 09 cf             	or     rdi,r9
  66af90:	48 39 f7             	cmp    rdi,rsi
  66af93:	0f 83 d4 4e 00 00    	jae    66fe6d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x595d>
  66af99:	4c 8b 74 24 58       	mov    r14,QWORD PTR [rsp+0x58]
  66af9e:	4d 09 ce             	or     r14,r9
  66afa1:	49 39 f6             	cmp    r14,rsi
  66afa4:	0f 83 bb 4e 00 00    	jae    66fe65 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5955>
  66afaa:	48 8b 84 24 80 00 00 	mov    rax,QWORD PTR [rsp+0x80]
  66afb1:	00 
  66afb2:	4c 09 c8             	or     rax,r9
  66afb5:	48 39 f0             	cmp    rax,rsi
  66afb8:	0f 83 7c 4e 00 00    	jae    66fe3a <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x592a>
  66afbe:	4c 8b ac 24 98 00 00 	mov    r13,QWORD PTR [rsp+0x98]
  66afc5:	00 
  66afc6:	4d 09 cd             	or     r13,r9
  66afc9:	49 39 f5             	cmp    r13,rsi
  66afcc:	0f 83 98 4e 00 00    	jae    66fe6a <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x595a>
  66afd2:	4c 8b bc 24 88 00 00 	mov    r15,QWORD PTR [rsp+0x88]
  66afd9:	00 
  66afda:	4d 09 cf             	or     r15,r9
  66afdd:	49 39 f7             	cmp    r15,rsi
  66afe0:	0f 83 4f 4e 00 00    	jae    66fe35 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5925>
  66afe6:	4c 8b 44 24 78       	mov    r8,QWORD PTR [rsp+0x78]
  66afeb:	4d 09 c8             	or     r8,r9
  66afee:	49 39 f0             	cmp    r8,rsi
  66aff1:	0f 83 39 4e 00 00    	jae    66fe30 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5920>
  66aff7:	4c 8b 94 24 90 00 00 	mov    r10,QWORD PTR [rsp+0x90]
  66affe:	00 
  66afff:	4d 09 ca             	or     r10,r9
  66b002:	49 39 f2             	cmp    r10,rsi
  66b005:	0f 83 34 4e 00 00    	jae    66fe3f <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x592f>
  66b00b:	4c 8b 9c 24 20 01 00 	mov    r11,QWORD PTR [rsp+0x120]
  66b012:	00 
  66b013:	4d 09 cb             	or     r11,r9
  66b016:	49 39 f3             	cmp    r11,rsi
  66b019:	0f 83 37 4e 00 00    	jae    66fe56 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5946>
  66b01f:	48 8b 94 24 30 01 00 	mov    rdx,QWORD PTR [rsp+0x130]
  66b026:	00 
  66b027:	4c 09 ca             	or     rdx,r9
  66b02a:	48 39 f2             	cmp    rdx,rsi
  66b02d:	0f 83 2d 4e 00 00    	jae    66fe60 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5950>
  66b033:	4c 89 84 24 c0 00 00 	mov    QWORD PTR [rsp+0xc0],r8
  66b03a:	00 
  66b03b:	4c 8b 84 24 28 01 00 	mov    r8,QWORD PTR [rsp+0x128]
  66b042:	00 
  66b043:	4d 09 c8             	or     r8,r9
  66b046:	49 39 f0             	cmp    r8,rsi
  66b049:	0f 83 e1 4d 00 00    	jae    66fe30 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5920>
  66b04f:	4c 89 bc 24 a0 00 00 	mov    QWORD PTR [rsp+0xa0],r15
  66b056:	00 
  66b057:	4c 8b bc 24 38 01 00 	mov    r15,QWORD PTR [rsp+0x138]
  66b05e:	00 
  66b05f:	4d 09 cf             	or     r15,r9
  66b062:	49 39 f7             	cmp    r15,rsi
  66b065:	0f 83 ca 4d 00 00    	jae    66fe35 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5925>
  66b06b:	48 8b 9c 24 f8 01 00 	mov    rbx,QWORD PTR [rsp+0x1f8]
  66b072:	00 
  66b073:	4c 09 cb             	or     rbx,r9
  66b076:	48 39 f3             	cmp    rbx,rsi
  66b079:	0f 83 dc 4d 00 00    	jae    66fe5b <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x594b>
  66b07f:	4c 89 ac 24 40 01 00 	mov    QWORD PTR [rsp+0x140],r13
  66b086:	00 
  66b087:	48 89 4c 24 40       	mov    QWORD PTR [rsp+0x40],rcx
  66b08c:	4d 89 e5             	mov    r13,r12
  66b08f:	4c 8b a4 24 f0 01 00 	mov    r12,QWORD PTR [rsp+0x1f0]
  66b096:	00 
  66b097:	4d 09 cc             	or     r12,r9
  66b09a:	49 39 f4             	cmp    r12,rsi
  66b09d:	0f 83 a1 4d 00 00    	jae    66fe44 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5934>
  66b0a3:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
  66b0a8:	48 8b 84 24 e8 01 00 	mov    rax,QWORD PTR [rsp+0x1e8]
  66b0af:	00 
  66b0b0:	4c 09 c8             	or     rax,r9
  66b0b3:	48 39 f0             	cmp    rax,rsi
  66b0b6:	0f 83 7e 4d 00 00    	jae    66fe3a <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x592a>
  66b0bc:	48 89 7c 24 18       	mov    QWORD PTR [rsp+0x18],rdi
  66b0c1:	48 8b bc 24 e0 01 00 	mov    rdi,QWORD PTR [rsp+0x1e0]
  66b0c8:	00 
  66b0c9:	4c 09 cf             	or     rdi,r9
  66b0cc:	48 39 f7             	cmp    rdi,rsi
  66b0cf:	0f 83 98 4d 00 00    	jae    66fe6d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x595d>
  66b0d5:	4c 0b 8c 24 d8 01 00 	or     r9,QWORD PTR [rsp+0x1d8]
  66b0dc:	00 
  66b0dd:	49 39 f1             	cmp    r9,rsi
  66b0e0:	0f 83 63 4d 00 00    	jae    66fe49 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5939>
  66b0e6:	48 8b b4 24 d0 01 00 	mov    rsi,QWORD PTR [rsp+0x1d0]
  66b0ed:	00 
  66b0ee:	42 0f b6 0c 06       	movzx  ecx,BYTE PTR [rsi+r8*1]
  66b0f3:	c1 e1 08             	shl    ecx,0x8
  66b0f6:	0f b6 14 16          	movzx  edx,BYTE PTR [rsi+rdx*1]
  66b0fa:	48 09 ca             	or     rdx,rcx
  66b0fd:	42 0f b6 0c 3e       	movzx  ecx,BYTE PTR [rsi+r15*1]
  66b102:	c1 e1 10             	shl    ecx,0x10
  66b105:	48 09 ca             	or     rdx,rcx
  66b108:	0f b6 0c 1e          	movzx  ecx,BYTE PTR [rsi+rbx*1]
  66b10c:	48 89 f3             	mov    rbx,rsi
  66b10f:	c1 e1 18             	shl    ecx,0x18
  66b112:	48 09 ca             	or     rdx,rcx
  66b115:	42 0f b6 0c 26       	movzx  ecx,BYTE PTR [rsi+r12*1]
  66b11a:	48 c1 e1 20          	shl    rcx,0x20
  66b11e:	48 09 ca             	or     rdx,rcx
  66b121:	0f b6 04 06          	movzx  eax,BYTE PTR [rsi+rax*1]
  66b125:	48 c1 e0 28          	shl    rax,0x28
  66b129:	48 09 c2             	or     rdx,rax
  66b12c:	0f b6 04 3e          	movzx  eax,BYTE PTR [rsi+rdi*1]
  66b130:	48 c1 e0 30          	shl    rax,0x30
  66b134:	48 09 c2             	or     rdx,rax
  66b137:	42 0f b6 04 0e       	movzx  eax,BYTE PTR [rsi+r9*1]
  66b13c:	48 c1 e0 38          	shl    rax,0x38
  66b140:	48 09 c2             	or     rdx,rax
  66b143:	42 0f b6 0c 36       	movzx  ecx,BYTE PTR [rsi+r14*1]
  66b148:	c1 e1 08             	shl    ecx,0x8
  66b14b:	48 8b 44 24 18       	mov    rax,QWORD PTR [rsp+0x18]
  66b150:	0f b6 04 06          	movzx  eax,BYTE PTR [rsi+rax*1]
  66b154:	48 09 c8             	or     rax,rcx
  66b157:	48 8b 4c 24 20       	mov    rcx,QWORD PTR [rsp+0x20]
  66b15c:	0f b6 0c 0e          	movzx  ecx,BYTE PTR [rsi+rcx*1]
  66b160:	c1 e1 10             	shl    ecx,0x10
  66b163:	48 09 c8             	or     rax,rcx
  66b166:	48 8b 8c 24 40 01 00 	mov    rcx,QWORD PTR [rsp+0x140]
  66b16d:	00 
  66b16e:	0f b6 0c 0e          	movzx  ecx,BYTE PTR [rsi+rcx*1]
  66b172:	c1 e1 18             	shl    ecx,0x18
  66b175:	48 09 c8             	or     rax,rcx
  66b178:	48 8b 8c 24 a0 00 00 	mov    rcx,QWORD PTR [rsp+0xa0]
  66b17f:	00 
  66b180:	0f b6 0c 0e          	movzx  ecx,BYTE PTR [rsi+rcx*1]
  66b184:	48 c1 e1 20          	shl    rcx,0x20
  66b188:	48 09 c8             	or     rax,rcx
  66b18b:	48 8b 8c 24 c0 00 00 	mov    rcx,QWORD PTR [rsp+0xc0]
  66b192:	00 
  66b193:	0f b6 0c 0e          	movzx  ecx,BYTE PTR [rsi+rcx*1]
  66b197:	48 c1 e1 28          	shl    rcx,0x28
  66b19b:	48 09 c8             	or     rax,rcx
  66b19e:	42 0f b6 0c 16       	movzx  ecx,BYTE PTR [rsi+r10*1]
  66b1a3:	48 c1 e1 30          	shl    rcx,0x30
  66b1a7:	48 09 c8             	or     rax,rcx
  66b1aa:	42 0f b6 0c 1e       	movzx  ecx,BYTE PTR [rsi+r11*1]
  66b1af:	48 c1 e1 38          	shl    rcx,0x38
  66b1b3:	48 09 c8             	or     rax,rcx
  66b1b6:	49 89 55 08          	mov    QWORD PTR [r13+0x8],rdx
  66b1ba:	49 89 45 00          	mov    QWORD PTR [r13+0x0],rax
  66b1be:	48 8b 4c 24 40       	mov    rcx,QWORD PTR [rsp+0x40]
  66b1c3:	48 ff c1             	inc    rcx
  66b1c6:	49 83 c5 10          	add    r13,0x10
  66b1ca:	48 83 f9 08          	cmp    rcx,0x8
  66b1ce:	48 8b b4 24 c0 01 00 	mov    rsi,QWORD PTR [rsp+0x1c0]
  66b1d5:	00 
  66b1d6:	4d 89 ec             	mov    r12,r13
  66b1d9:	0f 85 a1 fd ff ff    	jne    66af80 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0xa70>
  66b1df:	c5 fc 10 84 24 c0 02 	vmovups ymm0,YMMWORD PTR [rsp+0x2c0]
  66b1e6:	00 00 
  66b1e8:	c5 fc 10 8c 24 e0 02 	vmovups ymm1,YMMWORD PTR [rsp+0x2e0]
  66b1ef:	00 00 
  66b1f1:	c5 fc 10 94 24 00 03 	vmovups ymm2,YMMWORD PTR [rsp+0x300]
  66b1f8:	00 00 
  66b1fa:	c5 fe 6f 9c 24 20 03 	vmovdqu ymm3,YMMWORD PTR [rsp+0x320]
  66b201:	00 00 
  66b203:	c5 fd 7f 9c 24 60 02 	vmovdqa YMMWORD PTR [rsp+0x260],ymm3
  66b20a:	00 00 
  66b20c:	c5 fc 29 94 24 40 02 	vmovaps YMMWORD PTR [rsp+0x240],ymm2
  66b213:	00 00 
  66b215:	c5 fc 29 8c 24 20 02 	vmovaps YMMWORD PTR [rsp+0x220],ymm1
  66b21c:	00 00 
  66b21e:	c5 fc 29 84 24 00 02 	vmovaps YMMWORD PTR [rsp+0x200],ymm0
  66b225:	00 00 
  66b227:	48 8d 8c 24 f0 1a 00 	lea    rcx,[rsp+0x1af0]
  66b22e:	00 
  66b22f:	31 d2                	xor    edx,edx
  66b231:	4c 8b 7c 24 38       	mov    r15,QWORD PTR [rsp+0x38]
  66b236:	49 b9 c0 ff ff ff ff 	movabs r9,0x7fffffffffffffc0
  66b23d:	ff ff 7f 
  66b240:	4c 8d 52 01          	lea    r10,[rdx+0x1]
  66b244:	48 8d 42 02          	lea    rax,[rdx+0x2]
  66b248:	49 8d 7c 11 40       	lea    rdi,[r9+rdx*1+0x40]
  66b24d:	4c 21 d7             	and    rdi,r10
  66b250:	48 c1 e7 04          	shl    rdi,0x4
  66b254:	f3 4d 0f bc c2       	tzcnt  r8,r10
  66b259:	41 c1 e0 04          	shl    r8d,0x4
  66b25d:	c4 a1 78 28 84 04 00 	vmovaps xmm0,XMMWORD PTR [rsp+r8*1+0x200]
  66b264:	02 00 00 
  66b267:	c5 f8 57 84 3c c0 1a 	vxorps xmm0,xmm0,XMMWORD PTR [rsp+rdi*1+0x1ac0]
  66b26e:	00 00 
  66b270:	c5 f8 29 41 e0       	vmovaps XMMWORD PTR [rcx-0x20],xmm0
  66b275:	49 21 c2             	and    r10,rax
  66b278:	49 c1 e2 04          	shl    r10,0x4
  66b27c:	c4 a1 7a 7e 84 14 c8 	vmovq  xmm0,QWORD PTR [rsp+r10*1+0x1ac8]
  66b283:	1a 00 00 
  66b286:	48 83 c2 03          	add    rdx,0x3
  66b28a:	c4 a1 7a 7e 8c 14 c0 	vmovq  xmm1,QWORD PTR [rsp+r10*1+0x1ac0]
  66b291:	1a 00 00 
  66b294:	c5 f1 6c c0          	vpunpcklqdq xmm0,xmm1,xmm0
  66b298:	f3 48 0f bc f8       	tzcnt  rdi,rax
  66b29d:	c1 e7 04             	shl    edi,0x4
  66b2a0:	c5 fa 7e 8c 3c 08 02 	vmovq  xmm1,QWORD PTR [rsp+rdi*1+0x208]
  66b2a7:	00 00 
  66b2a9:	c5 fa 7e 94 3c 00 02 	vmovq  xmm2,QWORD PTR [rsp+rdi*1+0x200]
  66b2b0:	00 00 
  66b2b2:	c5 e9 6c c9          	vpunpcklqdq xmm1,xmm2,xmm1
  66b2b6:	c5 f1 ef c0          	vpxor  xmm0,xmm1,xmm0
  66b2ba:	c4 e3 f9 16 41 f8 01 	vpextrq QWORD PTR [rcx-0x8],xmm0,0x1
  66b2c1:	c5 f9 d6 41 f0       	vmovq  QWORD PTR [rcx-0x10],xmm0
  66b2c6:	48 21 d0             	and    rax,rdx
  66b2c9:	48 c1 e0 04          	shl    rax,0x4
  66b2cd:	c5 fa 7e 84 04 c8 1a 	vmovq  xmm0,QWORD PTR [rsp+rax*1+0x1ac8]
  66b2d4:	00 00 
  66b2d6:	c5 fa 7e 8c 04 c0 1a 	vmovq  xmm1,QWORD PTR [rsp+rax*1+0x1ac0]
  66b2dd:	00 00 
  66b2df:	c5 f1 6c c0          	vpunpcklqdq xmm0,xmm1,xmm0
  66b2e3:	f3 48 0f bc c2       	tzcnt  rax,rdx
  66b2e8:	c1 e0 04             	shl    eax,0x4
  66b2eb:	c5 fa 7e 8c 04 08 02 	vmovq  xmm1,QWORD PTR [rsp+rax*1+0x208]
  66b2f2:	00 00 
  66b2f4:	c5 fa 7e 94 04 00 02 	vmovq  xmm2,QWORD PTR [rsp+rax*1+0x200]
  66b2fb:	00 00 
  66b2fd:	c5 e9 6c c9          	vpunpcklqdq xmm1,xmm2,xmm1
  66b301:	c5 f1 ef c0          	vpxor  xmm0,xmm1,xmm0
  66b305:	c5 f9 7f 01          	vmovdqa XMMWORD PTR [rcx],xmm0
  66b309:	48 83 c1 30          	add    rcx,0x30
  66b30d:	48 81 fa ff 00 00 00 	cmp    rdx,0xff
  66b314:	0f 85 26 ff ff ff    	jne    66b240 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0xd30>
  66b31a:	c5 fc 10 84 24 9f 17 	vmovups ymm0,YMMWORD PTR [rsp+0x179f]
  66b321:	00 00 
  66b323:	48 8d 8c 24 e0 02 00 	lea    rcx,[rsp+0x2e0]
  66b32a:	00 
  66b32b:	c5 fc 11 81 df 00 00 	vmovups YMMWORD PTR [rcx+0xdf],ymm0
  66b332:	00 
  66b333:	c5 fc 10 84 24 80 17 	vmovups ymm0,YMMWORD PTR [rsp+0x1780]
  66b33a:	00 00 
  66b33c:	c5 fc 11 81 c0 00 00 	vmovups YMMWORD PTR [rcx+0xc0],ymm0
  66b343:	00 
  66b344:	c5 fc 10 84 24 60 17 	vmovups ymm0,YMMWORD PTR [rsp+0x1760]
  66b34b:	00 00 
  66b34d:	c5 fc 11 81 a0 00 00 	vmovups YMMWORD PTR [rcx+0xa0],ymm0
  66b354:	00 
  66b355:	c5 fc 10 84 24 40 17 	vmovups ymm0,YMMWORD PTR [rsp+0x1740]
  66b35c:	00 00 
  66b35e:	c5 fc 11 81 80 00 00 	vmovups YMMWORD PTR [rcx+0x80],ymm0
  66b365:	00 
  66b366:	c5 fc 10 84 24 c0 16 	vmovups ymm0,YMMWORD PTR [rsp+0x16c0]
  66b36d:	00 00 
  66b36f:	c5 fc 10 8c 24 e0 16 	vmovups ymm1,YMMWORD PTR [rsp+0x16e0]
  66b376:	00 00 
  66b378:	c5 fc 10 94 24 00 17 	vmovups ymm2,YMMWORD PTR [rsp+0x1700]
  66b37f:	00 00 
  66b381:	c5 fc 10 9c 24 20 17 	vmovups ymm3,YMMWORD PTR [rsp+0x1720]
  66b388:	00 00 
  66b38a:	c5 fc 11 59 60       	vmovups YMMWORD PTR [rcx+0x60],ymm3
  66b38f:	c5 fc 11 51 40       	vmovups YMMWORD PTR [rcx+0x40],ymm2
  66b394:	c5 fc 11 49 20       	vmovups YMMWORD PTR [rcx+0x20],ymm1
  66b399:	c5 fc 11 01          	vmovups YMMWORD PTR [rcx],ymm0
  66b39d:	48 8d 84 24 b0 02 00 	lea    rax,[rsp+0x2b0]
  66b3a4:	00 
  66b3a5:	c5 d9 ef e4          	vpxor  xmm4,xmm4,xmm4
  66b3a9:	c5 fa 7f 20          	vmovdqu XMMWORD PTR [rax],xmm4
  66b3ad:	c5 fc 10 84 24 9f 13 	vmovups ymm0,YMMWORD PTR [rsp+0x139f]
  66b3b4:	00 00 
  66b3b6:	c5 fc 11 81 ff 01 00 	vmovups YMMWORD PTR [rcx+0x1ff],ymm0
  66b3bd:	00 
  66b3be:	c5 fc 10 84 24 80 13 	vmovups ymm0,YMMWORD PTR [rsp+0x1380]
  66b3c5:	00 00 
  66b3c7:	c5 fc 11 81 e0 01 00 	vmovups YMMWORD PTR [rcx+0x1e0],ymm0
  66b3ce:	00 
  66b3cf:	c5 fc 10 84 24 60 13 	vmovups ymm0,YMMWORD PTR [rsp+0x1360]
  66b3d6:	00 00 
  66b3d8:	c5 fc 11 81 c0 01 00 	vmovups YMMWORD PTR [rcx+0x1c0],ymm0
  66b3df:	00 
  66b3e0:	c5 fc 10 84 24 40 13 	vmovups ymm0,YMMWORD PTR [rsp+0x1340]
  66b3e7:	00 00 
  66b3e9:	c5 fc 11 81 a0 01 00 	vmovups YMMWORD PTR [rcx+0x1a0],ymm0
  66b3f0:	00 
  66b3f1:	c5 fc 10 84 24 c0 12 	vmovups ymm0,YMMWORD PTR [rsp+0x12c0]
  66b3f8:	00 00 
  66b3fa:	c5 fe 6f 8c 24 e0 12 	vmovdqu ymm1,YMMWORD PTR [rsp+0x12e0]
  66b401:	00 00 
  66b403:	c5 fe 6f 94 24 00 13 	vmovdqu ymm2,YMMWORD PTR [rsp+0x1300]
  66b40a:	00 00 
  66b40c:	c5 fe 6f 9c 24 20 13 	vmovdqu ymm3,YMMWORD PTR [rsp+0x1320]
  66b413:	00 00 
  66b415:	c5 fe 7f 99 80 01 00 	vmovdqu YMMWORD PTR [rcx+0x180],ymm3
  66b41c:	00 
  66b41d:	c5 fe 7f 91 60 01 00 	vmovdqu YMMWORD PTR [rcx+0x160],ymm2
  66b424:	00 
  66b425:	c5 fe 7f 89 40 01 00 	vmovdqu YMMWORD PTR [rcx+0x140],ymm1
  66b42c:	00 
  66b42d:	c5 fc 11 81 20 01 00 	vmovups YMMWORD PTR [rcx+0x120],ymm0
  66b434:	00 
  66b435:	0f b6 84 24 bf 02 00 	movzx  eax,BYTE PTR [rsp+0x2bf]
  66b43c:	00 
  66b43d:	c5 f8 10 84 24 af 02 	vmovups xmm0,XMMWORD PTR [rsp+0x2af]
  66b444:	00 00 
  66b446:	c5 f8 11 81 ff 00 00 	vmovups XMMWORD PTR [rcx+0xff],xmm0
  66b44d:	00 
  66b44e:	88 81 0f 01 00 00    	mov    BYTE PTR [rcx+0x10f],al
  66b454:	48 8d 84 24 9f 02 00 	lea    rax,[rsp+0x29f]
  66b45b:	00 
  66b45c:	c5 fa 7f 20          	vmovdqu XMMWORD PTR [rax],xmm4
  66b460:	48 c7 84 24 f0 03 00 	mov    QWORD PTR [rsp+0x3f0],0x0
  66b467:	00 00 00 00 00 
  66b46c:	48 c7 84 24 f8 03 00 	mov    QWORD PTR [rsp+0x3f8],0xff
  66b473:	00 ff 00 00 00 
  66b478:	0f b6 84 24 ae 02 00 	movzx  eax,BYTE PTR [rsp+0x2ae]
  66b47f:	00 
  66b480:	88 81 2f 02 00 00    	mov    BYTE PTR [rcx+0x22f],al
  66b486:	c5 f8 10 84 24 9e 02 	vmovups xmm0,XMMWORD PTR [rsp+0x29e]
  66b48d:	00 00 
  66b48f:	c5 f8 11 81 1f 02 00 	vmovups XMMWORD PTR [rcx+0x21f],xmm0
  66b496:	00 
  66b497:	b9 20 00 00 00       	mov    ecx,0x20
  66b49c:	48 8d 94 24 02 04 00 	lea    rdx,[rsp+0x402]
  66b4a3:	00 
  66b4a4:	4c 8b 74 24 68       	mov    r14,QWORD PTR [rsp+0x68]
  66b4a9:	4c 8b a4 24 c8 01 00 	mov    r12,QWORD PTR [rsp+0x1c8]
  66b4b0:	00 
  66b4b1:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  66b4b8:	0f 1f 84 00 00 00 00 
  66b4bf:	00 
  66b4c0:	0f b6 ba de fe ff ff 	movzx  edi,BYTE PTR [rdx-0x122]
  66b4c7:	0f b6 42 fe          	movzx  eax,BYTE PTR [rdx-0x2]
  66b4cb:	c1 e7 08             	shl    edi,0x8
  66b4ce:	48 09 c7             	or     rdi,rax
  66b4d1:	48 39 f7             	cmp    rdi,rsi
  66b4d4:	0f 83 41 49 00 00    	jae    66fe1b <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x590b>
  66b4da:	0f b6 04 3b          	movzx  eax,BYTE PTR [rbx+rdi*1]
  66b4de:	c1 e0 04             	shl    eax,0x4
  66b4e1:	c4 c1 78 28 44 0e e0 	vmovaps xmm0,XMMWORD PTR [r14+rcx*1-0x20]
  66b4e8:	c5 f8 57 84 04 c0 1a 	vxorps xmm0,xmm0,XMMWORD PTR [rsp+rax*1+0x1ac0]
  66b4ef:	00 00 
  66b4f1:	c4 c1 78 29 44 0e e0 	vmovaps XMMWORD PTR [r14+rcx*1-0x20],xmm0
  66b4f8:	0f b6 ba df fe ff ff 	movzx  edi,BYTE PTR [rdx-0x121]
  66b4ff:	0f b6 42 ff          	movzx  eax,BYTE PTR [rdx-0x1]
  66b503:	c1 e7 08             	shl    edi,0x8
  66b506:	48 09 c7             	or     rdi,rax
  66b509:	48 39 f7             	cmp    rdi,rsi
  66b50c:	0f 83 09 49 00 00    	jae    66fe1b <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x590b>
  66b512:	0f b6 04 3b          	movzx  eax,BYTE PTR [rbx+rdi*1]
  66b516:	c1 e0 04             	shl    eax,0x4
  66b519:	c4 c1 78 28 44 0e f0 	vmovaps xmm0,XMMWORD PTR [r14+rcx*1-0x10]
  66b520:	c5 f8 57 84 04 c0 1a 	vxorps xmm0,xmm0,XMMWORD PTR [rsp+rax*1+0x1ac0]
  66b527:	00 00 
  66b529:	c4 c1 78 29 44 0e f0 	vmovaps XMMWORD PTR [r14+rcx*1-0x10],xmm0
  66b530:	0f b6 ba e0 fe ff ff 	movzx  edi,BYTE PTR [rdx-0x120]
  66b537:	0f b6 02             	movzx  eax,BYTE PTR [rdx]
  66b53a:	c1 e7 08             	shl    edi,0x8
  66b53d:	48 09 c7             	or     rdi,rax
  66b540:	48 39 f7             	cmp    rdi,rsi
  66b543:	0f 83 d2 48 00 00    	jae    66fe1b <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x590b>
  66b549:	0f b6 04 3b          	movzx  eax,BYTE PTR [rbx+rdi*1]
  66b54d:	c1 e0 04             	shl    eax,0x4
  66b550:	c4 c1 79 6f 04 0e    	vmovdqa xmm0,XMMWORD PTR [r14+rcx*1]
  66b556:	c5 f9 ef 84 04 c0 1a 	vpxor  xmm0,xmm0,XMMWORD PTR [rsp+rax*1+0x1ac0]
  66b55d:	00 00 
  66b55f:	c4 c1 79 7f 04 0e    	vmovdqa XMMWORD PTR [r14+rcx*1],xmm0
  66b565:	48 83 c1 30          	add    rcx,0x30
  66b569:	48 83 c2 03          	add    rdx,0x3
  66b56d:	48 81 f9 10 10 00 00 	cmp    rcx,0x1010
  66b574:	0f 85 46 ff ff ff    	jne    66b4c0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0xfb0>
  66b57a:	4d 39 fc             	cmp    r12,r15
  66b57d:	0f 85 dd f6 ff ff    	jne    66ac60 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x750>
  66b583:	bf f0 0f 00 00       	mov    edi,0xff0
  66b588:	c5 f8 77             	vzeroupper 
  66b58b:	ff 15 f7 bd 09 00    	call   QWORD PTR [rip+0x9bdf7]        # 707388 <malloc@GLIBC_2.2.5>
  66b591:	48 85 c0             	test   rax,rax
  66b594:	0f 84 95 4c 00 00    	je     67022f <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5d1f>
  66b59a:	49 89 c6             	mov    r14,rax
  66b59d:	ba f0 0f 00 00       	mov    edx,0xff0
  66b5a2:	48 89 c7             	mov    rdi,rax
  66b5a5:	48 8b 5c 24 68       	mov    rbx,QWORD PTR [rsp+0x68]
  66b5aa:	48 89 de             	mov    rsi,rbx
  66b5ad:	ff 15 f5 bd 09 00    	call   QWORD PTR [rip+0x9bdf5]        # 7073a8 <memcpy@GLIBC_2.14>
  66b5b3:	48 89 df             	mov    rdi,rbx
  66b5b6:	ff 15 dc bd 09 00    	call   QWORD PTR [rip+0x9bddc]        # 707398 <free@GLIBC_2.2.5>
  66b5bc:	48 8b 44 24 50       	mov    rax,QWORD PTR [rsp+0x50]
  66b5c1:	48 c7 00 ff 00 00 00 	mov    QWORD PTR [rax],0xff
  66b5c8:	4c 89 70 08          	mov    QWORD PTR [rax+0x8],r14
  66b5cc:	48 c7 40 10 ff 00 00 	mov    QWORD PTR [rax+0x10],0xff
  66b5d3:	00 
  66b5d4:	e9 33 48 00 00       	jmp    66fe0c <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x58fc>
  66b5d9:	4c 89 f6             	mov    rsi,r14
  66b5dc:	48 8b 54 24 28       	mov    rdx,QWORD PTR [rsp+0x28]
  66b5e1:	4d 89 f8             	mov    r8,r15
  66b5e4:	4c 8b 4c 24 70       	mov    r9,QWORD PTR [rsp+0x70]
  66b5e9:	6a 02                	push   0x2
  66b5eb:	e9 42 38 00 00       	jmp    66ee32 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4922>
  66b5f0:	8b 05 5a f7 09 00    	mov    eax,DWORD PTR [rip+0x9f75a]        # 70ad50 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1120>
  66b5f6:	85 c0                	test   eax,eax
  66b5f8:	0f 85 ee 48 00 00    	jne    66feec <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x59dc>
  66b5fe:	4c 89 74 24 60       	mov    QWORD PTR [rsp+0x60],r14
  66b603:	48 89 4c 24 30       	mov    QWORD PTR [rsp+0x30],rcx
  66b608:	48 89 7c 24 50       	mov    QWORD PTR [rsp+0x50],rdi
  66b60d:	8b 05 15 e6 09 00    	mov    eax,DWORD PTR [rip+0x9e615]        # 709c28 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290+0x930>
  66b613:	85 c0                	test   eax,eax
  66b615:	0f 85 ed 48 00 00    	jne    66ff08 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x59f8>
  66b61b:	48 8d bc 24 c0 02 00 	lea    rdi,[rsp+0x2c0]
  66b622:	00 
  66b623:	31 db                	xor    ebx,ebx
  66b625:	4c 8b 35 8c bd 09 00 	mov    r14,QWORD PTR [rip+0x9bd8c]        # 7073b8 <memset@GLIBC_2.2.5>
  66b62c:	ba 00 10 00 00       	mov    edx,0x1000
  66b631:	31 f6                	xor    esi,esi
  66b633:	41 ff d6             	call   r14
  66b636:	48 8d bc 24 c0 16 00 	lea    rdi,[rsp+0x16c0]
  66b63d:	00 
  66b63e:	ba 00 04 00 00       	mov    edx,0x400
  66b643:	31 f6                	xor    esi,esi
  66b645:	41 ff d6             	call   r14
  66b648:	48 8d bc 24 c0 12 00 	lea    rdi,[rsp+0x12c0]
  66b64f:	00 
  66b650:	ba 00 04 00 00       	mov    edx,0x400
  66b655:	31 f6                	xor    esi,esi
  66b657:	41 ff d6             	call   r14
  66b65a:	4d 85 ff             	test   r15,r15
  66b65d:	4c 89 7c 24 38       	mov    QWORD PTR [rsp+0x38],r15
  66b662:	74 21                	je     66b685 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1175>
  66b664:	48 8b 44 24 28       	mov    rax,QWORD PTR [rsp+0x28]
  66b669:	48 05 ff 03 00 00    	add    rax,0x3ff
  66b66f:	48 c1 e8 0a          	shr    rax,0xa
  66b673:	49 8d 9f ff 03 00 00 	lea    rbx,[r15+0x3ff]
  66b67a:	48 c1 eb 0a          	shr    rbx,0xa
  66b67e:	48 39 c3             	cmp    rbx,rax
  66b681:	48 0f 43 d8          	cmovae rbx,rax
  66b685:	48 83 bc 24 c0 00 00 	cmp    QWORD PTR [rsp+0xc0],0x0
  66b68c:	00 00 
  66b68e:	0f 84 a9 0a 00 00    	je     66c13d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1c2d>
  66b694:	48 8b 84 24 c0 00 00 	mov    rax,QWORD PTR [rsp+0xc0]
  66b69b:	00 
  66b69c:	48 83 c0 3f          	add    rax,0x3f
  66b6a0:	48 c1 e8 06          	shr    rax,0x6
  66b6a4:	48 39 d8             	cmp    rax,rbx
  66b6a7:	48 0f 43 c3          	cmovae rax,rbx
  66b6ab:	48 89 84 24 38 01 00 	mov    QWORD PTR [rsp+0x138],rax
  66b6b2:	00 
  66b6b3:	48 85 c0             	test   rax,rax
  66b6b6:	0f 84 81 0a 00 00    	je     66c13d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1c2d>
  66b6bc:	49 bc c0 ff ff ff ff 	movabs r12,0x7fffffffffffffc0
  66b6c3:	ff ff 7f 
  66b6c6:	31 d2                	xor    edx,edx
  66b6c8:	48 8d 1d b1 70 9a ff 	lea    rbx,[rip+0xffffffffff9a70b1]        # 12780 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1632>
  66b6cf:	4c 8d 3d ca 70 9a ff 	lea    r15,[rip+0xffffffffff9a70ca]        # 127a0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1652>
  66b6d6:	4c 8d 2d b3 70 9a ff 	lea    r13,[rip+0xffffffffff9a70b3]        # 12790 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1642>
  66b6dd:	4c 8d 35 8c 70 9a ff 	lea    r14,[rip+0xffffffffff9a708c]        # 12770 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1622>
  66b6e4:	48 8b b4 24 c0 00 00 	mov    rsi,QWORD PTR [rsp+0xc0]
  66b6eb:	00 
  66b6ec:	48 8b 44 24 30       	mov    rax,QWORD PTR [rsp+0x30]
  66b6f1:	48 89 84 24 30 01 00 	mov    QWORD PTR [rsp+0x130],rax
  66b6f8:	00 
  66b6f9:	48 8b 44 24 60       	mov    rax,QWORD PTR [rsp+0x60]
  66b6fe:	48 89 84 24 28 01 00 	mov    QWORD PTR [rsp+0x128],rax
  66b705:	00 
  66b706:	48 8b 7c 24 38       	mov    rdi,QWORD PTR [rsp+0x38]
  66b70b:	4c 8b 44 24 28       	mov    r8,QWORD PTR [rsp+0x28]
  66b710:	e9 c6 00 00 00       	jmp    66b7db <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x12cb>
  66b715:	66 66 2e 0f 1f 84 00 	data16 cs nop WORD PTR [rax+rax*1+0x0]
  66b71c:	00 00 00 00 
  66b720:	48 ff c2             	inc    rdx
  66b723:	62 71 fe 48 7f 54 24 	vmovdqu64 ZMMWORD PTR [rsp+0xb40],zmm10
  66b72a:	2d 
  66b72b:	62 71 fe 48 7f 4c 24 	vmovdqu64 ZMMWORD PTR [rsp+0xb80],zmm9
  66b732:	2e 
  66b733:	62 71 fe 48 7f 44 24 	vmovdqu64 ZMMWORD PTR [rsp+0xc40],zmm8
  66b73a:	31 
  66b73b:	62 f1 fe 48 7f 7c 24 	vmovdqu64 ZMMWORD PTR [rsp+0xc80],zmm7
  66b742:	32 
  66b743:	62 f1 fe 48 7f 74 24 	vmovdqu64 ZMMWORD PTR [rsp+0xd40],zmm6
  66b74a:	35 
  66b74b:	62 71 fe 48 7f 7c 24 	vmovdqu64 ZMMWORD PTR [rsp+0xd80],zmm15
  66b752:	36 
  66b753:	62 f1 fe 48 7f 64 24 	vmovdqu64 ZMMWORD PTR [rsp+0xe40],zmm4
  66b75a:	39 
  66b75b:	62 71 fe 48 7f 74 24 	vmovdqu64 ZMMWORD PTR [rsp+0xe80],zmm14
  66b762:	3a 
  66b763:	62 f1 fe 48 7f 5c 24 	vmovdqu64 ZMMWORD PTR [rsp+0xf40],zmm3
  66b76a:	3d 
  66b76b:	62 71 fe 48 7f 6c 24 	vmovdqu64 ZMMWORD PTR [rsp+0xf80],zmm13
  66b772:	3e 
  66b773:	62 f1 fe 48 7f 54 24 	vmovdqu64 ZMMWORD PTR [rsp+0x1040],zmm2
  66b77a:	41 
  66b77b:	62 71 fe 48 7f 64 24 	vmovdqu64 ZMMWORD PTR [rsp+0x1080],zmm12
  66b782:	42 
  66b783:	62 f1 fe 48 7f 4c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x1140],zmm1
  66b78a:	45 
  66b78b:	62 71 fe 48 7f 5c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x1180],zmm11
  66b792:	46 
  66b793:	62 f1 fe 48 7f 44 24 	vmovdqu64 ZMMWORD PTR [rsp+0x1240],zmm0
  66b79a:	49 
  66b79b:	62 f1 fe 48 7f 6c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x1280],zmm5
  66b7a2:	4a 
  66b7a3:	49 81 c0 00 fc ff ff 	add    r8,0xfffffffffffffc00
  66b7aa:	48 81 c7 00 fc ff ff 	add    rdi,0xfffffffffffffc00
  66b7b1:	48 81 84 24 28 01 00 	add    QWORD PTR [rsp+0x128],0x400
  66b7b8:	00 00 04 00 00 
  66b7bd:	48 81 84 24 30 01 00 	add    QWORD PTR [rsp+0x130],0x400
  66b7c4:	00 00 04 00 00 
  66b7c9:	48 83 c6 c0          	add    rsi,0xffffffffffffffc0
  66b7cd:	48 3b 94 24 38 01 00 	cmp    rdx,QWORD PTR [rsp+0x138]
  66b7d4:	00 
  66b7d5:	0f 84 62 09 00 00    	je     66c13d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1c2d>
  66b7db:	48 83 fe 40          	cmp    rsi,0x40
  66b7df:	b8 40 00 00 00       	mov    eax,0x40
  66b7e4:	48 89 b4 24 98 00 00 	mov    QWORD PTR [rsp+0x98],rsi
  66b7eb:	00 
  66b7ec:	48 0f 42 c6          	cmovb  rax,rsi
  66b7f0:	48 89 44 24 58       	mov    QWORD PTR [rsp+0x58],rax
  66b7f5:	49 81 f8 00 04 00 00 	cmp    r8,0x400
  66b7fc:	b8 00 04 00 00       	mov    eax,0x400
  66b801:	4c 89 44 24 78       	mov    QWORD PTR [rsp+0x78],r8
  66b806:	49 0f 42 c0          	cmovb  rax,r8
  66b80a:	c1 e8 04             	shr    eax,0x4
  66b80d:	48 81 ff 00 04 00 00 	cmp    rdi,0x400
  66b814:	b9 00 04 00 00       	mov    ecx,0x400
  66b819:	48 89 bc 24 88 00 00 	mov    QWORD PTR [rsp+0x88],rdi
  66b820:	00 
  66b821:	48 0f 42 cf          	cmovb  rcx,rdi
  66b825:	c1 e9 04             	shr    ecx,0x4
  66b828:	48 39 c8             	cmp    rax,rcx
  66b82b:	48 0f 42 c8          	cmovb  rcx,rax
  66b82f:	48 89 4c 24 20       	mov    QWORD PTR [rsp+0x20],rcx
  66b834:	48 89 d1             	mov    rcx,rdx
  66b837:	48 c1 e1 0a          	shl    rcx,0xa
  66b83b:	48 8b 74 24 28       	mov    rsi,QWORD PTR [rsp+0x28]
  66b840:	48 29 ce             	sub    rsi,rcx
  66b843:	48 81 fe 00 04 00 00 	cmp    rsi,0x400
  66b84a:	b8 00 04 00 00       	mov    eax,0x400
  66b84f:	48 0f 43 f0          	cmovae rsi,rax
  66b853:	48 89 b4 24 a0 00 00 	mov    QWORD PTR [rsp+0xa0],rsi
  66b85a:	00 
  66b85b:	48 8b 74 24 38       	mov    rsi,QWORD PTR [rsp+0x38]
  66b860:	48 29 ce             	sub    rsi,rcx
  66b863:	48 81 fe 00 04 00 00 	cmp    rsi,0x400
  66b86a:	48 0f 43 f0          	cmovae rsi,rax
  66b86e:	48 89 74 24 40       	mov    QWORD PTR [rsp+0x40],rsi
  66b873:	48 89 94 24 80 00 00 	mov    QWORD PTR [rsp+0x80],rdx
  66b87a:	00 
  66b87b:	48 c1 e2 06          	shl    rdx,0x6
  66b87f:	48 8b b4 24 c0 00 00 	mov    rsi,QWORD PTR [rsp+0xc0]
  66b886:	00 
  66b887:	48 89 94 24 90 00 00 	mov    QWORD PTR [rsp+0x90],rdx
  66b88e:	00 
  66b88f:	48 29 d6             	sub    rsi,rdx
  66b892:	48 83 fe 40          	cmp    rsi,0x40
  66b896:	b8 40 00 00 00       	mov    eax,0x40
  66b89b:	48 0f 43 f0          	cmovae rsi,rax
  66b89f:	48 8b 44 24 70       	mov    rax,QWORD PTR [rsp+0x70]
  66b8a4:	48 89 8c 24 40 01 00 	mov    QWORD PTR [rsp+0x140],rcx
  66b8ab:	00 
  66b8ac:	48 8d 3c 08          	lea    rdi,[rax+rcx*1]
  66b8b0:	48 8d 94 24 c0 16 00 	lea    rdx,[rsp+0x16c0]
  66b8b7:	00 
  66b8b8:	48 89 b4 24 20 01 00 	mov    QWORD PTR [rsp+0x120],rsi
  66b8bf:	00 
  66b8c0:	48 89 f1             	mov    rcx,rsi
  66b8c3:	c5 f8 77             	vzeroupper 
  66b8c6:	e8 75 7a 00 00       	call   673340 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate5fieldNtB2_10SmallField15decompose_block>
  66b8cb:	48 8b 84 24 a0 00 00 	mov    rax,QWORD PTR [rsp+0xa0]
  66b8d2:	00 
  66b8d3:	c1 e8 04             	shr    eax,0x4
  66b8d6:	48 8b 4c 24 40       	mov    rcx,QWORD PTR [rsp+0x40]
  66b8db:	c1 e9 04             	shr    ecx,0x4
  66b8de:	48 39 c1             	cmp    rcx,rax
  66b8e1:	48 0f 42 c1          	cmovb  rax,rcx
  66b8e5:	48 85 c0             	test   rax,rax
  66b8e8:	62 f1 fd 48 6f 35 ce 	vmovdqa64 zmm6,ZMMWORD PTR [rip+0xffffffffff9a6bce]        # 124c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1372>
  66b8ef:	6b 9a ff 
  66b8f2:	62 f1 fd 48 6f 3d 04 	vmovdqa64 zmm7,ZMMWORD PTR [rip+0xffffffffff9a6c04]        # 12500 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x13b2>
  66b8f9:	6c 9a ff 
  66b8fc:	0f 84 be 03 00 00    	je     66bcc0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x17b0>
  66b902:	48 8b 44 24 60       	mov    rax,QWORD PTR [rsp+0x60]
  66b907:	48 8b 8c 24 40 01 00 	mov    rcx,QWORD PTR [rsp+0x140]
  66b90e:	00 
  66b90f:	48 01 c8             	add    rax,rcx
  66b912:	48 89 44 24 18       	mov    QWORD PTR [rsp+0x18],rax
  66b917:	48 03 4c 24 30       	add    rcx,QWORD PTR [rsp+0x30]
  66b91c:	48 89 8c 24 40 01 00 	mov    QWORD PTR [rsp+0x140],rcx
  66b923:	00 
  66b924:	48 8b 0d 0d f3 09 00 	mov    rcx,QWORD PTR [rip+0x9f30d]        # 70ac38 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1008>
  66b92b:	48 03 0d 46 f3 09 00 	add    rcx,QWORD PTR [rip+0x9f346]        # 70ac78 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1048>
  66b932:	48 8b 94 24 30 01 00 	mov    rdx,QWORD PTR [rsp+0x130]
  66b939:	00 
  66b93a:	48 8b b4 24 28 01 00 	mov    rsi,QWORD PTR [rsp+0x128]
  66b941:	00 
  66b942:	31 ff                	xor    edi,edi
  66b944:	eb 48                	jmp    66b98e <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x147e>
  66b946:	66 2e 0f 1f 84 00 00 	cs nop WORD PTR [rax+rax*1+0x0]
  66b94d:	00 00 00 
  66b950:	48 c1 e7 07          	shl    rdi,0x7
  66b954:	62 f2 75 48 cf cb    	vgf2p8mulb zmm1,zmm1,zmm3
  66b95a:	62 f1 fd 48 7f 4c 3c 	vmovdqa64 ZMMWORD PTR [rsp+rdi*1+0x1ac0],zmm1
  66b961:	6b 
  66b962:	62 f2 7d 48 cf c2    	vgf2p8mulb zmm0,zmm0,zmm2
  66b968:	62 f1 fd 48 7f 44 3c 	vmovdqa64 ZMMWORD PTR [rsp+rdi*1+0x1b00],zmm0
  66b96f:	6c 
  66b970:	48 83 c6 10          	add    rsi,0x10
  66b974:	48 83 c2 10          	add    rdx,0x10
  66b978:	48 8b 84 24 a0 00 00 	mov    rax,QWORD PTR [rsp+0xa0]
  66b97f:	00 
  66b980:	48 89 c7             	mov    rdi,rax
  66b983:	48 3b 44 24 20       	cmp    rax,QWORD PTR [rsp+0x20]
  66b988:	0f 84 32 03 00 00    	je     66bcc0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x17b0>
  66b98e:	48 8d 47 01          	lea    rax,[rdi+0x1]
  66b992:	48 89 84 24 a0 00 00 	mov    QWORD PTR [rsp+0xa0],rax
  66b999:	00 
  66b99a:	48 89 f8             	mov    rax,rdi
  66b99d:	48 c1 e0 04          	shl    rax,0x4
  66b9a1:	4c 8b 8c 24 40 01 00 	mov    r9,QWORD PTR [rsp+0x140]
  66b9a8:	00 
  66b9a9:	4d 8b 04 01          	mov    r8,QWORD PTR [r9+rax*1]
  66b9ad:	4c 89 44 24 40       	mov    QWORD PTR [rsp+0x40],r8
  66b9b2:	4c 8b 44 24 18       	mov    r8,QWORD PTR [rsp+0x18]
  66b9b7:	c4 c1 78 10 04 00    	vmovups xmm0,XMMWORD PTR [r8+rax*1]
  66b9bd:	45 8b 04 00          	mov    r8d,DWORD PTR [r8+rax*1]
  66b9c1:	c4 c1 78 54 04 01    	vandps xmm0,xmm0,XMMWORD PTR [r9+rax*1]
  66b9c7:	c5 f8 11 84 04 c0 12 	vmovups XMMWORD PTR [rsp+rax*1+0x12c0],xmm0
  66b9ce:	00 00 
  66b9d0:	41 0f b6 c0          	movzx  eax,r8b
  66b9d4:	c1 e0 07             	shl    eax,0x7
  66b9d7:	62 f1 fe 48 6f 0c 01 	vmovdqu64 zmm1,ZMMWORD PTR [rcx+rax*1]
  66b9de:	62 f1 fe 48 6f 44 01 	vmovdqu64 zmm0,ZMMWORD PTR [rcx+rax*1+0x40]
  66b9e5:	01 
  66b9e6:	41 b9 01 00 00 00    	mov    r9d,0x1
  66b9ec:	41 b8 08 00 00 00    	mov    r8d,0x8
  66b9f2:	eb 3c                	jmp    66ba30 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1520>
  66b9f4:	62 f2 cd 48 36 d2    	vpermq zmm2,zmm6,zmm2
  66b9fa:	49 83 f3 40          	xor    r11,0x40
  66b9fe:	62 b1 fe 48 6f 1c 18 	vmovdqu64 zmm3,ZMMWORD PTR [rax+r11*1]
  66ba05:	62 f3 e5 48 43 db 4e 	vshufi64x2 zmm3,zmm3,zmm3,0x4e
  66ba0c:	62 f1 6d 48 ef c9    	vpxord zmm1,zmm2,zmm1
  66ba12:	62 f1 7d 48 70 d3 4e 	vpshufd zmm2,zmm3,0x4e
  66ba19:	62 f1 6d 48 ef c0    	vpxord zmm0,zmm2,zmm0
  66ba1f:	49 ff c1             	inc    r9
  66ba22:	49 83 c0 08          	add    r8,0x8
  66ba26:	49 83 f9 10          	cmp    r9,0x10
  66ba2a:	0f 84 10 01 00 00    	je     66bb40 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1630>
  66ba30:	42 0f b6 04 0e       	movzx  eax,BYTE PTR [rsi+r9*1]
  66ba35:	c1 e0 07             	shl    eax,0x7
  66ba38:	48 01 c8             	add    rax,rcx
  66ba3b:	45 89 ca             	mov    r10d,r9d
  66ba3e:	41 d1 ea             	shr    r10d,1
  66ba41:	41 83 e2 03          	and    r10d,0x3
  66ba45:	4d 89 c3             	mov    r11,r8
  66ba48:	4d 21 e3             	and    r11,r12
  66ba4b:	62 d1 fe 48 6f 14 03 	vmovdqu64 zmm2,ZMMWORD PTR [r11+rax*1]
  66ba52:	41 f6 c1 01          	test   r9b,0x1
  66ba56:	75 28                	jne    66ba80 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1570>
  66ba58:	4e 63 14 93          	movsxd r10,DWORD PTR [rbx+r10*4]
  66ba5c:	49 01 da             	add    r10,rbx
  66ba5f:	41 ff e2             	jmp    r10
  66ba62:	49 83 f3 40          	xor    r11,0x40
  66ba66:	62 b1 fe 48 6f 1c 18 	vmovdqu64 zmm3,ZMMWORD PTR [rax+r11*1]
  66ba6d:	e9 95 00 00 00       	jmp    66bb07 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x15f7>
  66ba72:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  66ba79:	1f 84 00 00 00 00 00 
  66ba80:	4f 63 14 96          	movsxd r10,DWORD PTR [r14+r10*4]
  66ba84:	4d 01 f2             	add    r10,r14
  66ba87:	41 ff e2             	jmp    r10
  66ba8a:	62 f1 7d 48 70 d2 4e 	vpshufd zmm2,zmm2,0x4e
  66ba91:	49 83 f3 40          	xor    r11,0x40
  66ba95:	62 b1 fe 48 6f 1c 18 	vmovdqu64 zmm3,ZMMWORD PTR [rax+r11*1]
  66ba9c:	e9 6b ff ff ff       	jmp    66ba0c <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x14fc>
  66baa1:	62 f3 ed 48 43 d2 4e 	vshufi64x2 zmm2,zmm2,zmm2,0x4e
  66baa8:	49 83 f3 40          	xor    r11,0x40
  66baac:	62 b1 fe 48 6f 1c 18 	vmovdqu64 zmm3,ZMMWORD PTR [rax+r11*1]
  66bab3:	62 f3 e5 48 43 db 4e 	vshufi64x2 zmm3,zmm3,zmm3,0x4e
  66baba:	eb 4b                	jmp    66bb07 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x15f7>
  66babc:	62 f3 ed 48 43 d2 1b 	vshufi64x2 zmm2,zmm2,zmm2,0x1b
  66bac3:	49 83 f3 40          	xor    r11,0x40
  66bac7:	62 b1 fe 48 6f 1c 18 	vmovdqu64 zmm3,ZMMWORD PTR [rax+r11*1]
  66bace:	62 f3 e5 48 43 db 1b 	vshufi64x2 zmm3,zmm3,zmm3,0x1b
  66bad5:	eb 30                	jmp    66bb07 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x15f7>
  66bad7:	62 f2 c5 48 36 d2    	vpermq zmm2,zmm7,zmm2
  66badd:	49 83 f3 40          	xor    r11,0x40
  66bae1:	62 b1 fe 48 6f 1c 18 	vmovdqu64 zmm3,ZMMWORD PTR [rax+r11*1]
  66bae8:	62 f3 e5 48 43 db 1b 	vshufi64x2 zmm3,zmm3,zmm3,0x1b
  66baef:	e9 18 ff ff ff       	jmp    66ba0c <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x14fc>
  66baf4:	62 f3 fd 48 00 d2 4e 	vpermq zmm2,zmm2,0x4e
  66bafb:	49 83 f3 40          	xor    r11,0x40
  66baff:	62 b3 fd 48 00 1c 18 	vpermq zmm3,ZMMWORD PTR [rax+r11*1],0x4e
  66bb06:	4e 
  66bb07:	62 f1 75 48 ef ca    	vpxord zmm1,zmm1,zmm2
  66bb0d:	62 f1 7d 48 ef c3    	vpxord zmm0,zmm0,zmm3
  66bb13:	49 ff c1             	inc    r9
  66bb16:	49 83 c0 08          	add    r8,0x8
  66bb1a:	49 83 f9 10          	cmp    r9,0x10
  66bb1e:	0f 85 0c ff ff ff    	jne    66ba30 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1520>
  66bb24:	eb 1a                	jmp    66bb40 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1630>
  66bb26:	62 f3 fd 48 00 d2 1b 	vpermq zmm2,zmm2,0x1b
  66bb2d:	49 83 f3 40          	xor    r11,0x40
  66bb31:	62 b3 fd 48 00 1c 18 	vpermq zmm3,ZMMWORD PTR [rax+r11*1],0x4e
  66bb38:	4e 
  66bb39:	e9 ce fe ff ff       	jmp    66ba0c <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x14fc>
  66bb3e:	66 90                	xchg   ax,ax
  66bb40:	0f b6 44 24 40       	movzx  eax,BYTE PTR [rsp+0x40]
  66bb45:	c1 e0 07             	shl    eax,0x7
  66bb48:	62 f1 fe 48 6f 1c 01 	vmovdqu64 zmm3,ZMMWORD PTR [rcx+rax*1]
  66bb4f:	62 f1 fe 48 6f 54 01 	vmovdqu64 zmm2,ZMMWORD PTR [rcx+rax*1+0x40]
  66bb56:	01 
  66bb57:	41 b9 01 00 00 00    	mov    r9d,0x1
  66bb5d:	41 ba 08 00 00 00    	mov    r10d,0x8
  66bb63:	eb 3c                	jmp    66bba1 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1691>
  66bb65:	62 f2 cd 48 36 e4    	vpermq zmm4,zmm6,zmm4
  66bb6b:	49 83 f0 40          	xor    r8,0x40
  66bb6f:	62 b1 fe 48 6f 2c 00 	vmovdqu64 zmm5,ZMMWORD PTR [rax+r8*1]
  66bb76:	62 f3 d5 48 43 ed 4e 	vshufi64x2 zmm5,zmm5,zmm5,0x4e
  66bb7d:	62 f1 5d 48 ef db    	vpxord zmm3,zmm4,zmm3
  66bb83:	62 f1 7d 48 70 e5 4e 	vpshufd zmm4,zmm5,0x4e
  66bb8a:	62 f1 5d 48 ef d2    	vpxord zmm2,zmm4,zmm2
  66bb90:	49 ff c1             	inc    r9
  66bb93:	49 83 c2 08          	add    r10,0x8
  66bb97:	49 83 f9 10          	cmp    r9,0x10
  66bb9b:	0f 84 af fd ff ff    	je     66b950 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1440>
  66bba1:	42 0f b6 04 0a       	movzx  eax,BYTE PTR [rdx+r9*1]
  66bba6:	c1 e0 07             	shl    eax,0x7
  66bba9:	48 01 c8             	add    rax,rcx
  66bbac:	45 89 cb             	mov    r11d,r9d
  66bbaf:	41 d1 eb             	shr    r11d,1
  66bbb2:	41 83 e3 03          	and    r11d,0x3
  66bbb6:	4d 89 d0             	mov    r8,r10
  66bbb9:	4d 21 e0             	and    r8,r12
  66bbbc:	62 d1 fe 48 6f 24 00 	vmovdqu64 zmm4,ZMMWORD PTR [r8+rax*1]
  66bbc3:	41 f6 c1 01          	test   r9b,0x1
  66bbc7:	75 27                	jne    66bbf0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x16e0>
  66bbc9:	4f 63 1c 9f          	movsxd r11,DWORD PTR [r15+r11*4]
  66bbcd:	4d 01 fb             	add    r11,r15
  66bbd0:	41 ff e3             	jmp    r11
  66bbd3:	49 83 f0 40          	xor    r8,0x40
  66bbd7:	62 b1 fe 48 6f 2c 00 	vmovdqu64 zmm5,ZMMWORD PTR [rax+r8*1]
  66bbde:	e9 95 00 00 00       	jmp    66bc78 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1768>
  66bbe3:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  66bbea:	84 00 00 00 00 00 
  66bbf0:	4f 63 5c 9d 00       	movsxd r11,DWORD PTR [r13+r11*4+0x0]
  66bbf5:	4d 01 eb             	add    r11,r13
  66bbf8:	41 ff e3             	jmp    r11
  66bbfb:	62 f1 7d 48 70 e4 4e 	vpshufd zmm4,zmm4,0x4e
  66bc02:	49 83 f0 40          	xor    r8,0x40
  66bc06:	62 b1 fe 48 6f 2c 00 	vmovdqu64 zmm5,ZMMWORD PTR [rax+r8*1]
  66bc0d:	e9 6b ff ff ff       	jmp    66bb7d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x166d>
  66bc12:	62 f3 dd 48 43 e4 4e 	vshufi64x2 zmm4,zmm4,zmm4,0x4e
  66bc19:	49 83 f0 40          	xor    r8,0x40
  66bc1d:	62 b1 fe 48 6f 2c 00 	vmovdqu64 zmm5,ZMMWORD PTR [rax+r8*1]
  66bc24:	62 f3 d5 48 43 ed 4e 	vshufi64x2 zmm5,zmm5,zmm5,0x4e
  66bc2b:	eb 4b                	jmp    66bc78 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1768>
  66bc2d:	62 f3 dd 48 43 e4 1b 	vshufi64x2 zmm4,zmm4,zmm4,0x1b
  66bc34:	49 83 f0 40          	xor    r8,0x40
  66bc38:	62 b1 fe 48 6f 2c 00 	vmovdqu64 zmm5,ZMMWORD PTR [rax+r8*1]
  66bc3f:	62 f3 d5 48 43 ed 1b 	vshufi64x2 zmm5,zmm5,zmm5,0x1b
  66bc46:	eb 30                	jmp    66bc78 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1768>
  66bc48:	62 f2 c5 48 36 e4    	vpermq zmm4,zmm7,zmm4
  66bc4e:	49 83 f0 40          	xor    r8,0x40
  66bc52:	62 b1 fe 48 6f 2c 00 	vmovdqu64 zmm5,ZMMWORD PTR [rax+r8*1]
  66bc59:	62 f3 d5 48 43 ed 1b 	vshufi64x2 zmm5,zmm5,zmm5,0x1b
  66bc60:	e9 18 ff ff ff       	jmp    66bb7d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x166d>
  66bc65:	62 f3 fd 48 00 e4 4e 	vpermq zmm4,zmm4,0x4e
  66bc6c:	49 83 f0 40          	xor    r8,0x40
  66bc70:	62 b3 fd 48 00 2c 00 	vpermq zmm5,ZMMWORD PTR [rax+r8*1],0x4e
  66bc77:	4e 
  66bc78:	62 f1 65 48 ef dc    	vpxord zmm3,zmm3,zmm4
  66bc7e:	62 f1 6d 48 ef d5    	vpxord zmm2,zmm2,zmm5
  66bc84:	49 ff c1             	inc    r9
  66bc87:	49 83 c2 08          	add    r10,0x8
  66bc8b:	49 83 f9 10          	cmp    r9,0x10
  66bc8f:	0f 85 0c ff ff ff    	jne    66bba1 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1691>
  66bc95:	e9 b6 fc ff ff       	jmp    66b950 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1440>
  66bc9a:	62 f3 fd 48 00 e4 1b 	vpermq zmm4,zmm4,0x1b
  66bca1:	49 83 f0 40          	xor    r8,0x40
  66bca5:	62 b3 fd 48 00 2c 00 	vpermq zmm5,ZMMWORD PTR [rax+r8*1],0x4e
  66bcac:	4e 
  66bcad:	e9 cb fe ff ff       	jmp    66bb7d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x166d>
  66bcb2:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  66bcb9:	1f 84 00 00 00 00 00 
  66bcc0:	48 8b 44 24 58       	mov    rax,QWORD PTR [rsp+0x58]
  66bcc5:	c1 e0 07             	shl    eax,0x7
  66bcc8:	48 89 44 24 58       	mov    QWORD PTR [rsp+0x58],rax
  66bccd:	48 8d bc 24 c0 02 00 	lea    rdi,[rsp+0x2c0]
  66bcd4:	00 
  66bcd5:	48 8d b4 24 c0 16 00 	lea    rsi,[rsp+0x16c0]
  66bcdc:	00 
  66bcdd:	48 8b 94 24 20 01 00 	mov    rdx,QWORD PTR [rsp+0x120]
  66bce4:	00 
  66bce5:	48 8d 8c 24 c0 12 00 	lea    rcx,[rsp+0x12c0]
  66bcec:	00 
  66bced:	49 89 d0             	mov    r8,rdx
  66bcf0:	c5 f8 77             	vzeroupper 
  66bcf3:	e8 a8 3f 01 00       	call   67fca0 <_RNvNtNtNtCs5Palesvl9wF_6mikros10univariate5tiled8affine_h16consume_h_affine>
  66bcf8:	62 71 fe 48 6f 54 24 	vmovdqu64 zmm10,ZMMWORD PTR [rsp+0x340]
  66bcff:	0d 
  66bd00:	62 71 fe 48 6f 4c 24 	vmovdqu64 zmm9,ZMMWORD PTR [rsp+0x380]
  66bd07:	0e 
  66bd08:	62 71 fe 48 6f 44 24 	vmovdqu64 zmm8,ZMMWORD PTR [rsp+0x440]
  66bd0f:	11 
  66bd10:	62 f1 fe 48 6f 7c 24 	vmovdqu64 zmm7,ZMMWORD PTR [rsp+0x480]
  66bd17:	12 
  66bd18:	62 f1 fe 48 6f 74 24 	vmovdqu64 zmm6,ZMMWORD PTR [rsp+0x540]
  66bd1f:	15 
  66bd20:	62 f1 fe 48 6f 64 24 	vmovdqu64 zmm4,ZMMWORD PTR [rsp+0x640]
  66bd27:	19 
  66bd28:	62 f1 fe 48 6f 5c 24 	vmovdqu64 zmm3,ZMMWORD PTR [rsp+0x740]
  66bd2f:	1d 
  66bd30:	62 f1 fe 48 6f 54 24 	vmovdqu64 zmm2,ZMMWORD PTR [rsp+0x840]
  66bd37:	21 
  66bd38:	62 f1 fe 48 6f 4c 24 	vmovdqu64 zmm1,ZMMWORD PTR [rsp+0x940]
  66bd3f:	25 
  66bd40:	62 f1 fe 48 6f 44 24 	vmovdqu64 zmm0,ZMMWORD PTR [rsp+0xa40]
  66bd47:	29 
  66bd48:	62 71 fe 48 6f 7c 24 	vmovdqu64 zmm15,ZMMWORD PTR [rsp+0x580]
  66bd4f:	16 
  66bd50:	62 71 fe 48 6f 74 24 	vmovdqu64 zmm14,ZMMWORD PTR [rsp+0x680]
  66bd57:	1a 
  66bd58:	62 71 fe 48 6f 6c 24 	vmovdqu64 zmm13,ZMMWORD PTR [rsp+0x780]
  66bd5f:	1e 
  66bd60:	62 71 fe 48 6f 64 24 	vmovdqu64 zmm12,ZMMWORD PTR [rsp+0x880]
  66bd67:	22 
  66bd68:	62 71 fe 48 6f 5c 24 	vmovdqu64 zmm11,ZMMWORD PTR [rsp+0x980]
  66bd6f:	26 
  66bd70:	62 f1 fe 48 6f 6c 24 	vmovdqu64 zmm5,ZMMWORD PTR [rsp+0xa80]
  66bd77:	2a 
  66bd78:	4c 8b 94 24 90 00 00 	mov    r10,QWORD PTR [rsp+0x90]
  66bd7f:	00 
  66bd80:	4c 39 94 24 c0 00 00 	cmp    QWORD PTR [rsp+0xc0],r10
  66bd87:	00 
  66bd88:	75 36                	jne    66bdc0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x18b0>
  66bd8a:	48 8b 94 24 80 00 00 	mov    rdx,QWORD PTR [rsp+0x80]
  66bd91:	00 
  66bd92:	48 8b b4 24 98 00 00 	mov    rsi,QWORD PTR [rsp+0x98]
  66bd99:	00 
  66bd9a:	48 8b bc 24 88 00 00 	mov    rdi,QWORD PTR [rsp+0x88]
  66bda1:	00 
  66bda2:	4c 8b 44 24 78       	mov    r8,QWORD PTR [rsp+0x78]
  66bda7:	4c 8b 4c 24 58       	mov    r9,QWORD PTR [rsp+0x58]
  66bdac:	e9 57 01 00 00       	jmp    66bf08 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x19f8>
  66bdb1:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  66bdb8:	0f 1f 84 00 00 00 00 
  66bdbf:	00 
  66bdc0:	31 c0                	xor    eax,eax
  66bdc2:	48 8d 8c 24 c7 16 00 	lea    rcx,[rsp+0x16c7]
  66bdc9:	00 
  66bdca:	48 8b 94 24 80 00 00 	mov    rdx,QWORD PTR [rsp+0x80]
  66bdd1:	00 
  66bdd2:	48 8b b4 24 98 00 00 	mov    rsi,QWORD PTR [rsp+0x98]
  66bdd9:	00 
  66bdda:	48 8b bc 24 88 00 00 	mov    rdi,QWORD PTR [rsp+0x88]
  66bde1:	00 
  66bde2:	4c 8b 44 24 78       	mov    r8,QWORD PTR [rsp+0x78]
  66bde7:	4c 8b 4c 24 58       	mov    r9,QWORD PTR [rsp+0x58]
  66bdec:	0f 1f 40 00          	nop    DWORD PTR [rax+0x0]
  66bdf0:	62 e2 7d 48 78 41 f9 	vpbroadcastb zmm16,BYTE PTR [rcx-0x7]
  66bdf7:	62 e1 fd 48 6f 4c 04 	vmovdqa64 zmm17,ZMMWORD PTR [rsp+rax*1+0x1ac0]
  66bdfe:	6b 
  66bdff:	62 e1 fd 48 6f 54 04 	vmovdqa64 zmm18,ZMMWORD PTR [rsp+rax*1+0x1b00]
  66be06:	6c 
  66be07:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66be0d:	62 31 2d 48 ef d3    	vpxord zmm10,zmm10,zmm19
  66be13:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66be19:	62 31 35 48 ef c8    	vpxord zmm9,zmm9,zmm16
  66be1f:	62 e2 7d 48 78 41 fa 	vpbroadcastb zmm16,BYTE PTR [rcx-0x6]
  66be26:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66be2c:	62 31 3d 48 ef c3    	vpxord zmm8,zmm8,zmm19
  66be32:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66be38:	62 b1 45 48 ef f8    	vpxord zmm7,zmm7,zmm16
  66be3e:	62 e2 7d 48 78 41 fb 	vpbroadcastb zmm16,BYTE PTR [rcx-0x5]
  66be45:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66be4b:	62 b1 4d 48 ef f3    	vpxord zmm6,zmm6,zmm19
  66be51:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66be57:	62 31 05 48 ef f8    	vpxord zmm15,zmm15,zmm16
  66be5d:	62 e2 7d 48 78 41 fc 	vpbroadcastb zmm16,BYTE PTR [rcx-0x4]
  66be64:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66be6a:	62 b1 5d 48 ef e3    	vpxord zmm4,zmm4,zmm19
  66be70:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66be76:	62 31 0d 48 ef f0    	vpxord zmm14,zmm14,zmm16
  66be7c:	62 e2 7d 48 78 41 fd 	vpbroadcastb zmm16,BYTE PTR [rcx-0x3]
  66be83:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66be89:	62 b1 65 48 ef db    	vpxord zmm3,zmm3,zmm19
  66be8f:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66be95:	62 31 15 48 ef e8    	vpxord zmm13,zmm13,zmm16
  66be9b:	62 e2 7d 48 78 41 fe 	vpbroadcastb zmm16,BYTE PTR [rcx-0x2]
  66bea2:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66bea8:	62 b1 6d 48 ef d3    	vpxord zmm2,zmm2,zmm19
  66beae:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66beb4:	62 31 1d 48 ef e0    	vpxord zmm12,zmm12,zmm16
  66beba:	62 e2 7d 48 78 41 ff 	vpbroadcastb zmm16,BYTE PTR [rcx-0x1]
  66bec1:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66bec7:	62 b1 75 48 ef cb    	vpxord zmm1,zmm1,zmm19
  66becd:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66bed3:	62 31 25 48 ef d8    	vpxord zmm11,zmm11,zmm16
  66bed9:	62 e2 7d 48 78 01    	vpbroadcastb zmm16,BYTE PTR [rcx]
  66bedf:	62 a2 75 40 cf c8    	vgf2p8mulb zmm17,zmm17,zmm16
  66bee5:	62 b1 7d 48 ef c1    	vpxord zmm0,zmm0,zmm17
  66beeb:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66bef1:	62 b1 55 48 ef e8    	vpxord zmm5,zmm5,zmm16
  66bef7:	48 83 c1 10          	add    rcx,0x10
  66befb:	48 83 e8 80          	sub    rax,0xffffffffffffff80
  66beff:	49 39 c1             	cmp    r9,rax
  66bf02:	0f 85 e8 fe ff ff    	jne    66bdf0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x18e0>
  66bf08:	62 71 fe 48 7f 54 24 	vmovdqu64 ZMMWORD PTR [rsp+0x340],zmm10
  66bf0f:	0d 
  66bf10:	62 71 fe 48 7f 4c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x380],zmm9
  66bf17:	0e 
  66bf18:	62 71 fe 48 7f 44 24 	vmovdqu64 ZMMWORD PTR [rsp+0x440],zmm8
  66bf1f:	11 
  66bf20:	62 f1 fe 48 7f 7c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x480],zmm7
  66bf27:	12 
  66bf28:	62 f1 fe 48 7f 74 24 	vmovdqu64 ZMMWORD PTR [rsp+0x540],zmm6
  66bf2f:	15 
  66bf30:	62 71 fe 48 7f 7c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x580],zmm15
  66bf37:	16 
  66bf38:	62 f1 fe 48 7f 64 24 	vmovdqu64 ZMMWORD PTR [rsp+0x640],zmm4
  66bf3f:	19 
  66bf40:	62 71 fe 48 7f 74 24 	vmovdqu64 ZMMWORD PTR [rsp+0x680],zmm14
  66bf47:	1a 
  66bf48:	62 f1 fe 48 7f 5c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x740],zmm3
  66bf4f:	1d 
  66bf50:	62 71 fe 48 7f 6c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x780],zmm13
  66bf57:	1e 
  66bf58:	62 f1 fe 48 7f 54 24 	vmovdqu64 ZMMWORD PTR [rsp+0x840],zmm2
  66bf5f:	21 
  66bf60:	62 71 fe 48 7f 64 24 	vmovdqu64 ZMMWORD PTR [rsp+0x880],zmm12
  66bf67:	22 
  66bf68:	62 f1 fe 48 7f 4c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x940],zmm1
  66bf6f:	25 
  66bf70:	62 71 fe 48 7f 5c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x980],zmm11
  66bf77:	26 
  66bf78:	62 f1 fe 48 7f 44 24 	vmovdqu64 ZMMWORD PTR [rsp+0xa40],zmm0
  66bf7f:	29 
  66bf80:	62 f1 fe 48 7f 6c 24 	vmovdqu64 ZMMWORD PTR [rsp+0xa80],zmm5
  66bf87:	2a 
  66bf88:	62 71 fe 48 6f 54 24 	vmovdqu64 zmm10,ZMMWORD PTR [rsp+0xb40]
  66bf8f:	2d 
  66bf90:	62 71 fe 48 6f 4c 24 	vmovdqu64 zmm9,ZMMWORD PTR [rsp+0xb80]
  66bf97:	2e 
  66bf98:	62 71 fe 48 6f 44 24 	vmovdqu64 zmm8,ZMMWORD PTR [rsp+0xc40]
  66bf9f:	31 
  66bfa0:	62 f1 fe 48 6f 7c 24 	vmovdqu64 zmm7,ZMMWORD PTR [rsp+0xc80]
  66bfa7:	32 
  66bfa8:	62 f1 fe 48 6f 74 24 	vmovdqu64 zmm6,ZMMWORD PTR [rsp+0xd40]
  66bfaf:	35 
  66bfb0:	62 f1 fe 48 6f 64 24 	vmovdqu64 zmm4,ZMMWORD PTR [rsp+0xe40]
  66bfb7:	39 
  66bfb8:	62 f1 fe 48 6f 5c 24 	vmovdqu64 zmm3,ZMMWORD PTR [rsp+0xf40]
  66bfbf:	3d 
  66bfc0:	62 f1 fe 48 6f 54 24 	vmovdqu64 zmm2,ZMMWORD PTR [rsp+0x1040]
  66bfc7:	41 
  66bfc8:	62 f1 fe 48 6f 4c 24 	vmovdqu64 zmm1,ZMMWORD PTR [rsp+0x1140]
  66bfcf:	45 
  66bfd0:	62 f1 fe 48 6f 44 24 	vmovdqu64 zmm0,ZMMWORD PTR [rsp+0x1240]
  66bfd7:	49 
  66bfd8:	62 71 fe 48 6f 7c 24 	vmovdqu64 zmm15,ZMMWORD PTR [rsp+0xd80]
  66bfdf:	36 
  66bfe0:	62 71 fe 48 6f 74 24 	vmovdqu64 zmm14,ZMMWORD PTR [rsp+0xe80]
  66bfe7:	3a 
  66bfe8:	62 71 fe 48 6f 6c 24 	vmovdqu64 zmm13,ZMMWORD PTR [rsp+0xf80]
  66bfef:	3e 
  66bff0:	62 71 fe 48 6f 64 24 	vmovdqu64 zmm12,ZMMWORD PTR [rsp+0x1080]
  66bff7:	42 
  66bff8:	62 71 fe 48 6f 5c 24 	vmovdqu64 zmm11,ZMMWORD PTR [rsp+0x1180]
  66bfff:	46 
  66c000:	62 f1 fe 48 6f 6c 24 	vmovdqu64 zmm5,ZMMWORD PTR [rsp+0x1280]
  66c007:	4a 
  66c008:	4c 39 94 24 c0 00 00 	cmp    QWORD PTR [rsp+0xc0],r10
  66c00f:	00 
  66c010:	0f 84 0a f7 ff ff    	je     66b720 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1210>
  66c016:	31 c0                	xor    eax,eax
  66c018:	48 8d 8c 24 cf 16 00 	lea    rcx,[rsp+0x16cf]
  66c01f:	00 
  66c020:	62 e2 7d 48 78 41 f9 	vpbroadcastb zmm16,BYTE PTR [rcx-0x7]
  66c027:	62 e1 fd 48 6f 4c 04 	vmovdqa64 zmm17,ZMMWORD PTR [rsp+rax*1+0x1ac0]
  66c02e:	6b 
  66c02f:	62 e1 fd 48 6f 54 04 	vmovdqa64 zmm18,ZMMWORD PTR [rsp+rax*1+0x1b00]
  66c036:	6c 
  66c037:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66c03d:	62 31 2d 48 ef d3    	vpxord zmm10,zmm10,zmm19
  66c043:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66c049:	62 31 35 48 ef c8    	vpxord zmm9,zmm9,zmm16
  66c04f:	62 e2 7d 48 78 41 fa 	vpbroadcastb zmm16,BYTE PTR [rcx-0x6]
  66c056:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66c05c:	62 31 3d 48 ef c3    	vpxord zmm8,zmm8,zmm19
  66c062:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66c068:	62 b1 45 48 ef f8    	vpxord zmm7,zmm7,zmm16
  66c06e:	62 e2 7d 48 78 41 fb 	vpbroadcastb zmm16,BYTE PTR [rcx-0x5]
  66c075:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66c07b:	62 b1 4d 48 ef f3    	vpxord zmm6,zmm6,zmm19
  66c081:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66c087:	62 31 05 48 ef f8    	vpxord zmm15,zmm15,zmm16
  66c08d:	62 e2 7d 48 78 41 fc 	vpbroadcastb zmm16,BYTE PTR [rcx-0x4]
  66c094:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66c09a:	62 b1 5d 48 ef e3    	vpxord zmm4,zmm4,zmm19
  66c0a0:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66c0a6:	62 31 0d 48 ef f0    	vpxord zmm14,zmm14,zmm16
  66c0ac:	62 e2 7d 48 78 41 fd 	vpbroadcastb zmm16,BYTE PTR [rcx-0x3]
  66c0b3:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66c0b9:	62 b1 65 48 ef db    	vpxord zmm3,zmm3,zmm19
  66c0bf:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66c0c5:	62 31 15 48 ef e8    	vpxord zmm13,zmm13,zmm16
  66c0cb:	62 e2 7d 48 78 41 fe 	vpbroadcastb zmm16,BYTE PTR [rcx-0x2]
  66c0d2:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66c0d8:	62 b1 6d 48 ef d3    	vpxord zmm2,zmm2,zmm19
  66c0de:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66c0e4:	62 31 1d 48 ef e0    	vpxord zmm12,zmm12,zmm16
  66c0ea:	62 e2 7d 48 78 41 ff 	vpbroadcastb zmm16,BYTE PTR [rcx-0x1]
  66c0f1:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66c0f7:	62 b1 75 48 ef cb    	vpxord zmm1,zmm1,zmm19
  66c0fd:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66c103:	62 31 25 48 ef d8    	vpxord zmm11,zmm11,zmm16
  66c109:	62 e2 7d 48 78 01    	vpbroadcastb zmm16,BYTE PTR [rcx]
  66c10f:	62 a2 75 40 cf c8    	vgf2p8mulb zmm17,zmm17,zmm16
  66c115:	62 b1 7d 48 ef c1    	vpxord zmm0,zmm0,zmm17
  66c11b:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66c121:	62 b1 55 48 ef e8    	vpxord zmm5,zmm5,zmm16
  66c127:	48 83 c1 10          	add    rcx,0x10
  66c12b:	48 83 e8 80          	sub    rax,0xffffffffffffff80
  66c12f:	49 39 c1             	cmp    r9,rax
  66c132:	0f 85 e8 fe ff ff    	jne    66c020 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1b10>
  66c138:	e9 e3 f5 ff ff       	jmp    66b720 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1210>
  66c13d:	bf f0 0f 00 00       	mov    edi,0xff0
  66c142:	c5 f8 77             	vzeroupper 
  66c145:	ff 15 3d b2 09 00    	call   QWORD PTR [rip+0x9b23d]        # 707388 <malloc@GLIBC_2.2.5>
  66c14b:	48 85 c0             	test   rax,rax
  66c14e:	0f 84 ed 40 00 00    	je     670241 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5d31>
  66c154:	b9 0f 00 00 00       	mov    ecx,0xf
  66c159:	48 8d 94 24 c0 11 00 	lea    rdx,[rsp+0x11c0]
  66c160:	00 
  66c161:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  66c168:	0f 1f 84 00 00 00 00 
  66c16f:	00 
  66c170:	0f b6 b2 00 f1 ff ff 	movzx  esi,BYTE PTR [rdx-0xf00]
  66c177:	40 88 b4 24 a0 00 00 	mov    BYTE PTR [rsp+0xa0],sil
  66c17e:	00 
  66c17f:	0f b6 ba 00 f2 ff ff 	movzx  edi,BYTE PTR [rdx-0xe00]
  66c186:	44 0f b6 82 00 f3 ff 	movzx  r8d,BYTE PTR [rdx-0xd00]
  66c18d:	ff 
  66c18e:	44 0f b6 8a 00 f4 ff 	movzx  r9d,BYTE PTR [rdx-0xc00]
  66c195:	ff 
  66c196:	44 0f b6 92 00 f5 ff 	movzx  r10d,BYTE PTR [rdx-0xb00]
  66c19d:	ff 
  66c19e:	44 0f b6 9a 00 f6 ff 	movzx  r11d,BYTE PTR [rdx-0xa00]
  66c1a5:	ff 
  66c1a6:	0f b6 9a 00 f7 ff ff 	movzx  ebx,BYTE PTR [rdx-0x900]
  66c1ad:	44 0f b6 b2 00 f8 ff 	movzx  r14d,BYTE PTR [rdx-0x800]
  66c1b4:	ff 
  66c1b5:	44 0f b6 ba 00 f9 ff 	movzx  r15d,BYTE PTR [rdx-0x700]
  66c1bc:	ff 
  66c1bd:	44 0f b6 a2 00 fa ff 	movzx  r12d,BYTE PTR [rdx-0x600]
  66c1c4:	ff 
  66c1c5:	44 0f b6 aa 00 fb ff 	movzx  r13d,BYTE PTR [rdx-0x500]
  66c1cc:	ff 
  66c1cd:	0f b6 b2 00 fc ff ff 	movzx  esi,BYTE PTR [rdx-0x400]
  66c1d4:	40 88 b4 24 c0 00 00 	mov    BYTE PTR [rsp+0xc0],sil
  66c1db:	00 
  66c1dc:	0f b6 b2 00 fd ff ff 	movzx  esi,BYTE PTR [rdx-0x300]
  66c1e3:	40 88 74 24 40       	mov    BYTE PTR [rsp+0x40],sil
  66c1e8:	0f b6 b2 00 fe ff ff 	movzx  esi,BYTE PTR [rdx-0x200]
  66c1ef:	40 88 b4 24 40 01 00 	mov    BYTE PTR [rsp+0x140],sil
  66c1f6:	00 
  66c1f7:	0f b6 b2 00 ff ff ff 	movzx  esi,BYTE PTR [rdx-0x100]
  66c1fe:	40 88 74 24 20       	mov    BYTE PTR [rsp+0x20],sil
  66c203:	0f b6 32             	movzx  esi,BYTE PTR [rdx]
  66c206:	40 88 74 24 18       	mov    BYTE PTR [rsp+0x18],sil
  66c20b:	0f b6 b4 24 a0 00 00 	movzx  esi,BYTE PTR [rsp+0xa0]
  66c212:	00 
  66c213:	40 88 74 08 f1       	mov    BYTE PTR [rax+rcx*1-0xf],sil
  66c218:	40 88 7c 08 f2       	mov    BYTE PTR [rax+rcx*1-0xe],dil
  66c21d:	44 88 44 08 f3       	mov    BYTE PTR [rax+rcx*1-0xd],r8b
  66c222:	44 88 4c 08 f4       	mov    BYTE PTR [rax+rcx*1-0xc],r9b
  66c227:	44 88 54 08 f5       	mov    BYTE PTR [rax+rcx*1-0xb],r10b
  66c22c:	44 88 5c 08 f6       	mov    BYTE PTR [rax+rcx*1-0xa],r11b
  66c231:	88 5c 08 f7          	mov    BYTE PTR [rax+rcx*1-0x9],bl
  66c235:	44 88 74 08 f8       	mov    BYTE PTR [rax+rcx*1-0x8],r14b
  66c23a:	44 88 7c 08 f9       	mov    BYTE PTR [rax+rcx*1-0x7],r15b
  66c23f:	44 88 64 08 fa       	mov    BYTE PTR [rax+rcx*1-0x6],r12b
  66c244:	44 88 6c 08 fb       	mov    BYTE PTR [rax+rcx*1-0x5],r13b
  66c249:	0f b6 b4 24 c0 00 00 	movzx  esi,BYTE PTR [rsp+0xc0]
  66c250:	00 
  66c251:	40 88 74 08 fc       	mov    BYTE PTR [rax+rcx*1-0x4],sil
  66c256:	0f b6 74 24 40       	movzx  esi,BYTE PTR [rsp+0x40]
  66c25b:	40 88 74 08 fd       	mov    BYTE PTR [rax+rcx*1-0x3],sil
  66c260:	0f b6 b4 24 40 01 00 	movzx  esi,BYTE PTR [rsp+0x140]
  66c267:	00 
  66c268:	40 88 74 08 fe       	mov    BYTE PTR [rax+rcx*1-0x2],sil
  66c26d:	0f b6 74 24 20       	movzx  esi,BYTE PTR [rsp+0x20]
  66c272:	40 88 74 08 ff       	mov    BYTE PTR [rax+rcx*1-0x1],sil
  66c277:	0f b6 74 24 18       	movzx  esi,BYTE PTR [rsp+0x18]
  66c27c:	40 88 34 08          	mov    BYTE PTR [rax+rcx*1],sil
  66c280:	48 83 c1 10          	add    rcx,0x10
  66c284:	48 ff c2             	inc    rdx
  66c287:	48 81 f9 ff 0f 00 00 	cmp    rcx,0xfff
  66c28e:	0f 85 dc fe ff ff    	jne    66c170 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1c60>
  66c294:	e9 5b 3b 00 00       	jmp    66fdf4 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x58e4>
  66c299:	4c 89 f6             	mov    rsi,r14
  66c29c:	48 8b 54 24 28       	mov    rdx,QWORD PTR [rsp+0x28]
  66c2a1:	4d 89 f8             	mov    r8,r15
  66c2a4:	4c 8b 4c 24 70       	mov    r9,QWORD PTR [rsp+0x70]
  66c2a9:	6a 00                	push   0x0
  66c2ab:	e9 82 2b 00 00       	jmp    66ee32 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4922>
  66c2b0:	8b 05 9a ea 09 00    	mov    eax,DWORD PTR [rip+0x9ea9a]        # 70ad50 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1120>
  66c2b6:	85 c0                	test   eax,eax
  66c2b8:	0f 85 54 3c 00 00    	jne    66ff12 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5a02>
  66c2be:	4c 89 74 24 60       	mov    QWORD PTR [rsp+0x60],r14
  66c2c3:	48 89 4c 24 30       	mov    QWORD PTR [rsp+0x30],rcx
  66c2c8:	48 89 7c 24 50       	mov    QWORD PTR [rsp+0x50],rdi
  66c2cd:	8b 05 55 d9 09 00    	mov    eax,DWORD PTR [rip+0x9d955]        # 709c28 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290+0x930>
  66c2d3:	85 c0                	test   eax,eax
  66c2d5:	0f 85 53 3c 00 00    	jne    66ff2e <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5a1e>
  66c2db:	48 8d bc 24 c0 02 00 	lea    rdi,[rsp+0x2c0]
  66c2e2:	00 
  66c2e3:	31 db                	xor    ebx,ebx
  66c2e5:	4c 8b 35 cc b0 09 00 	mov    r14,QWORD PTR [rip+0x9b0cc]        # 7073b8 <memset@GLIBC_2.2.5>
  66c2ec:	ba 00 10 00 00       	mov    edx,0x1000
  66c2f1:	31 f6                	xor    esi,esi
  66c2f3:	41 ff d6             	call   r14
  66c2f6:	48 8d bc 24 c0 16 00 	lea    rdi,[rsp+0x16c0]
  66c2fd:	00 
  66c2fe:	ba 00 04 00 00       	mov    edx,0x400
  66c303:	31 f6                	xor    esi,esi
  66c305:	41 ff d6             	call   r14
  66c308:	48 8d bc 24 c0 12 00 	lea    rdi,[rsp+0x12c0]
  66c30f:	00 
  66c310:	ba 00 04 00 00       	mov    edx,0x400
  66c315:	31 f6                	xor    esi,esi
  66c317:	41 ff d6             	call   r14
  66c31a:	4d 85 ff             	test   r15,r15
  66c31d:	4c 89 7c 24 38       	mov    QWORD PTR [rsp+0x38],r15
  66c322:	74 21                	je     66c345 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1e35>
  66c324:	48 8b 44 24 28       	mov    rax,QWORD PTR [rsp+0x28]
  66c329:	48 05 ff 03 00 00    	add    rax,0x3ff
  66c32f:	48 c1 e8 0a          	shr    rax,0xa
  66c333:	49 8d 9f ff 03 00 00 	lea    rbx,[r15+0x3ff]
  66c33a:	48 c1 eb 0a          	shr    rbx,0xa
  66c33e:	48 39 c3             	cmp    rbx,rax
  66c341:	48 0f 43 d8          	cmovae rbx,rax
  66c345:	48 83 bc 24 c0 00 00 	cmp    QWORD PTR [rsp+0xc0],0x0
  66c34c:	00 00 
  66c34e:	0f 84 05 38 00 00    	je     66fb59 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5649>
  66c354:	48 8b 84 24 c0 00 00 	mov    rax,QWORD PTR [rsp+0xc0]
  66c35b:	00 
  66c35c:	48 83 c0 3f          	add    rax,0x3f
  66c360:	48 c1 e8 06          	shr    rax,0x6
  66c364:	48 39 d8             	cmp    rax,rbx
  66c367:	48 0f 43 c3          	cmovae rax,rbx
  66c36b:	48 89 84 24 90 00 00 	mov    QWORD PTR [rsp+0x90],rax
  66c372:	00 
  66c373:	48 85 c0             	test   rax,rax
  66c376:	0f 84 dd 37 00 00    	je     66fb59 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5649>
  66c37c:	48 bb c0 ff ff ff ff 	movabs rbx,0x7fffffffffffffc0
  66c383:	ff ff 7f 
  66c386:	31 c9                	xor    ecx,ecx
  66c388:	48 8b 84 24 c0 00 00 	mov    rax,QWORD PTR [rsp+0xc0]
  66c38f:	00 
  66c390:	48 8b 54 24 30       	mov    rdx,QWORD PTR [rsp+0x30]
  66c395:	48 89 94 24 88 00 00 	mov    QWORD PTR [rsp+0x88],rdx
  66c39c:	00 
  66c39d:	48 8b 54 24 60       	mov    rdx,QWORD PTR [rsp+0x60]
  66c3a2:	48 89 54 24 78       	mov    QWORD PTR [rsp+0x78],rdx
  66c3a7:	48 8b 54 24 38       	mov    rdx,QWORD PTR [rsp+0x38]
  66c3ac:	48 8b 74 24 28       	mov    rsi,QWORD PTR [rsp+0x28]
  66c3b1:	eb 5c                	jmp    66c40f <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1eff>
  66c3b3:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  66c3ba:	84 00 00 00 00 00 
  66c3c0:	48 8b b4 24 98 00 00 	mov    rsi,QWORD PTR [rsp+0x98]
  66c3c7:	00 
  66c3c8:	48 81 c6 00 fc ff ff 	add    rsi,0xfffffffffffffc00
  66c3cf:	48 8b 94 24 80 00 00 	mov    rdx,QWORD PTR [rsp+0x80]
  66c3d6:	00 
  66c3d7:	48 81 c2 00 fc ff ff 	add    rdx,0xfffffffffffffc00
  66c3de:	48 81 44 24 78 00 04 	add    QWORD PTR [rsp+0x78],0x400
  66c3e5:	00 00 
  66c3e7:	48 81 84 24 88 00 00 	add    QWORD PTR [rsp+0x88],0x400
  66c3ee:	00 00 04 00 00 
  66c3f3:	48 8b 44 24 58       	mov    rax,QWORD PTR [rsp+0x58]
  66c3f8:	48 83 c0 c0          	add    rax,0xffffffffffffffc0
  66c3fc:	48 8b 4c 24 18       	mov    rcx,QWORD PTR [rsp+0x18]
  66c401:	48 3b 8c 24 90 00 00 	cmp    rcx,QWORD PTR [rsp+0x90]
  66c408:	00 
  66c409:	0f 84 4a 37 00 00    	je     66fb59 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5649>
  66c40f:	48 83 f8 40          	cmp    rax,0x40
  66c413:	41 be 40 00 00 00    	mov    r14d,0x40
  66c419:	48 89 44 24 58       	mov    QWORD PTR [rsp+0x58],rax
  66c41e:	4c 0f 42 f0          	cmovb  r14,rax
  66c422:	48 81 fe 00 04 00 00 	cmp    rsi,0x400
  66c429:	b8 00 04 00 00       	mov    eax,0x400
  66c42e:	48 89 b4 24 98 00 00 	mov    QWORD PTR [rsp+0x98],rsi
  66c435:	00 
  66c436:	48 0f 42 c6          	cmovb  rax,rsi
  66c43a:	c1 e8 04             	shr    eax,0x4
  66c43d:	48 81 fa 00 04 00 00 	cmp    rdx,0x400
  66c444:	be 00 04 00 00       	mov    esi,0x400
  66c449:	48 89 94 24 80 00 00 	mov    QWORD PTR [rsp+0x80],rdx
  66c450:	00 
  66c451:	48 0f 42 f2          	cmovb  rsi,rdx
  66c455:	c1 ee 04             	shr    esi,0x4
  66c458:	48 39 f0             	cmp    rax,rsi
  66c45b:	48 0f 42 f0          	cmovb  rsi,rax
  66c45f:	48 89 74 24 40       	mov    QWORD PTR [rsp+0x40],rsi
  66c464:	48 89 ca             	mov    rdx,rcx
  66c467:	48 c1 e2 0a          	shl    rdx,0xa
  66c46b:	4c 8b 7c 24 28       	mov    r15,QWORD PTR [rsp+0x28]
  66c470:	49 29 d7             	sub    r15,rdx
  66c473:	49 81 ff 00 04 00 00 	cmp    r15,0x400
  66c47a:	b8 00 04 00 00       	mov    eax,0x400
  66c47f:	4c 0f 43 f8          	cmovae r15,rax
  66c483:	4c 8b 64 24 38       	mov    r12,QWORD PTR [rsp+0x38]
  66c488:	49 29 d4             	sub    r12,rdx
  66c48b:	49 81 fc 00 04 00 00 	cmp    r12,0x400
  66c492:	4c 0f 43 e0          	cmovae r12,rax
  66c496:	48 89 4c 24 18       	mov    QWORD PTR [rsp+0x18],rcx
  66c49b:	48 89 c8             	mov    rax,rcx
  66c49e:	48 c1 e0 06          	shl    rax,0x6
  66c4a2:	48 8b 8c 24 c0 00 00 	mov    rcx,QWORD PTR [rsp+0xc0]
  66c4a9:	00 
  66c4aa:	48 89 84 24 a0 00 00 	mov    QWORD PTR [rsp+0xa0],rax
  66c4b1:	00 
  66c4b2:	48 29 c1             	sub    rcx,rax
  66c4b5:	48 83 f9 40          	cmp    rcx,0x40
  66c4b9:	b8 40 00 00 00       	mov    eax,0x40
  66c4be:	48 0f 43 c8          	cmovae rcx,rax
  66c4c2:	48 8b 44 24 70       	mov    rax,QWORD PTR [rsp+0x70]
  66c4c7:	49 89 d5             	mov    r13,rdx
  66c4ca:	48 8d 3c 10          	lea    rdi,[rax+rdx*1]
  66c4ce:	48 89 ce             	mov    rsi,rcx
  66c4d1:	48 8d 94 24 c0 16 00 	lea    rdx,[rsp+0x16c0]
  66c4d8:	00 
  66c4d9:	c5 f8 77             	vzeroupper 
  66c4dc:	e8 5f 6e 00 00       	call   673340 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate5fieldNtB2_10SmallField15decompose_block>
  66c4e1:	62 f1 fd 48 6f 3d 15 	vmovdqa64 zmm7,ZMMWORD PTR [rip+0xffffffffff9a6015]        # 12500 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x13b2>
  66c4e8:	60 9a ff 
  66c4eb:	62 f1 fd 48 6f 35 cb 	vmovdqa64 zmm6,ZMMWORD PTR [rip+0xffffffffff9a5fcb]        # 124c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1372>
  66c4f2:	5f 9a ff 
  66c4f5:	41 c1 ef 04          	shr    r15d,0x4
  66c4f9:	41 c1 ec 04          	shr    r12d,0x4
  66c4fd:	4d 39 fc             	cmp    r12,r15
  66c500:	4d 0f 42 fc          	cmovb  r15,r12
  66c504:	4d 85 ff             	test   r15,r15
  66c507:	0f 84 b3 03 00 00    	je     66c8c0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x23b0>
  66c50d:	48 8b 44 24 60       	mov    rax,QWORD PTR [rsp+0x60]
  66c512:	4c 01 e8             	add    rax,r13
  66c515:	48 89 44 24 20       	mov    QWORD PTR [rsp+0x20],rax
  66c51a:	4c 03 6c 24 30       	add    r13,QWORD PTR [rsp+0x30]
  66c51f:	4c 89 ac 24 40 01 00 	mov    QWORD PTR [rsp+0x140],r13
  66c526:	00 
  66c527:	48 8b 0d 0a e7 09 00 	mov    rcx,QWORD PTR [rip+0x9e70a]        # 70ac38 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1008>
  66c52e:	48 03 0d 43 e7 09 00 	add    rcx,QWORD PTR [rip+0x9e743]        # 70ac78 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1048>
  66c535:	48 8b 94 24 88 00 00 	mov    rdx,QWORD PTR [rsp+0x88]
  66c53c:	00 
  66c53d:	48 8b 74 24 78       	mov    rsi,QWORD PTR [rsp+0x78]
  66c542:	31 ff                	xor    edi,edi
  66c544:	eb 40                	jmp    66c586 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x2076>
  66c546:	66 2e 0f 1f 84 00 00 	cs nop WORD PTR [rax+rax*1+0x0]
  66c54d:	00 00 00 
  66c550:	48 c1 e7 07          	shl    rdi,0x7
  66c554:	62 f2 75 48 cf cb    	vgf2p8mulb zmm1,zmm1,zmm3
  66c55a:	62 f1 fd 48 7f 4c 3c 	vmovdqa64 ZMMWORD PTR [rsp+rdi*1+0x1ac0],zmm1
  66c561:	6b 
  66c562:	62 f2 7d 48 cf c2    	vgf2p8mulb zmm0,zmm0,zmm2
  66c568:	62 f1 fd 48 7f 44 3c 	vmovdqa64 ZMMWORD PTR [rsp+rdi*1+0x1b00],zmm0
  66c56f:	6c 
  66c570:	48 83 c6 10          	add    rsi,0x10
  66c574:	48 83 c2 10          	add    rdx,0x10
  66c578:	4c 89 c7             	mov    rdi,r8
  66c57b:	4c 3b 44 24 40       	cmp    r8,QWORD PTR [rsp+0x40]
  66c580:	0f 84 3a 03 00 00    	je     66c8c0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x23b0>
  66c586:	4c 8d 47 01          	lea    r8,[rdi+0x1]
  66c58a:	49 89 fa             	mov    r10,rdi
  66c58d:	49 c1 e2 04          	shl    r10,0x4
  66c591:	48 8b 84 24 40 01 00 	mov    rax,QWORD PTR [rsp+0x140]
  66c598:	00 
  66c599:	4e 8b 0c 10          	mov    r9,QWORD PTR [rax+r10*1]
  66c59d:	4c 8b 5c 24 20       	mov    r11,QWORD PTR [rsp+0x20]
  66c5a2:	c4 81 78 10 04 13    	vmovups xmm0,XMMWORD PTR [r11+r10*1]
  66c5a8:	47 8b 1c 13          	mov    r11d,DWORD PTR [r11+r10*1]
  66c5ac:	c4 a1 78 54 04 10    	vandps xmm0,xmm0,XMMWORD PTR [rax+r10*1]
  66c5b2:	c4 a1 78 11 84 14 c0 	vmovups XMMWORD PTR [rsp+r10*1+0x12c0],xmm0
  66c5b9:	12 00 00 
  66c5bc:	45 0f b6 d3          	movzx  r10d,r11b
  66c5c0:	41 c1 e2 07          	shl    r10d,0x7
  66c5c4:	62 b1 fe 48 6f 0c 11 	vmovdqu64 zmm1,ZMMWORD PTR [rcx+r10*1]
  66c5cb:	62 b1 fe 48 6f 44 11 	vmovdqu64 zmm0,ZMMWORD PTR [rcx+r10*1+0x40]
  66c5d2:	01 
  66c5d3:	41 ba 01 00 00 00    	mov    r10d,0x1
  66c5d9:	41 bb 08 00 00 00    	mov    r11d,0x8
  66c5df:	eb 3c                	jmp    66c61d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x210d>
  66c5e1:	62 f2 cd 48 36 d2    	vpermq zmm2,zmm6,zmm2
  66c5e7:	49 83 f4 40          	xor    r12,0x40
  66c5eb:	62 91 fe 48 6f 1c 27 	vmovdqu64 zmm3,ZMMWORD PTR [r15+r12*1]
  66c5f2:	62 f3 e5 48 43 db 4e 	vshufi64x2 zmm3,zmm3,zmm3,0x4e
  66c5f9:	62 f1 6d 48 ef c9    	vpxord zmm1,zmm2,zmm1
  66c5ff:	62 f1 7d 48 70 d3 4e 	vpshufd zmm2,zmm3,0x4e
  66c606:	62 f1 6d 48 ef c0    	vpxord zmm0,zmm2,zmm0
  66c60c:	49 ff c2             	inc    r10
  66c60f:	49 83 c3 08          	add    r11,0x8
  66c613:	49 83 fa 10          	cmp    r10,0x10
  66c617:	0f 84 23 01 00 00    	je     66c740 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x2230>
  66c61d:	46 0f b6 3c 16       	movzx  r15d,BYTE PTR [rsi+r10*1]
  66c622:	41 c1 e7 07          	shl    r15d,0x7
  66c626:	49 01 cf             	add    r15,rcx
  66c629:	45 89 d5             	mov    r13d,r10d
  66c62c:	41 d1 ed             	shr    r13d,1
  66c62f:	41 83 e5 03          	and    r13d,0x3
  66c633:	4d 89 dc             	mov    r12,r11
  66c636:	49 21 dc             	and    r12,rbx
  66c639:	62 91 fe 48 6f 14 3c 	vmovdqu64 zmm2,ZMMWORD PTR [r12+r15*1]
  66c640:	41 f6 c2 01          	test   r10b,0x1
  66c644:	75 2a                	jne    66c670 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x2160>
  66c646:	48 8d 05 b3 61 9a ff 	lea    rax,[rip+0xffffffffff9a61b3]        # 12800 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x16b2>
  66c64d:	4e 63 2c a8          	movsxd r13,DWORD PTR [rax+r13*4]
  66c651:	49 01 c5             	add    r13,rax
  66c654:	41 ff e5             	jmp    r13
  66c657:	49 83 f4 40          	xor    r12,0x40
  66c65b:	62 91 fe 48 6f 1c 27 	vmovdqu64 zmm3,ZMMWORD PTR [r15+r12*1]
  66c662:	e9 97 00 00 00       	jmp    66c6fe <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x21ee>
  66c667:	66 0f 1f 84 00 00 00 	nop    WORD PTR [rax+rax*1+0x0]
  66c66e:	00 00 
  66c670:	48 8d 05 79 61 9a ff 	lea    rax,[rip+0xffffffffff9a6179]        # 127f0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x16a2>
  66c677:	4e 63 2c a8          	movsxd r13,DWORD PTR [rax+r13*4]
  66c67b:	49 01 c5             	add    r13,rax
  66c67e:	41 ff e5             	jmp    r13
  66c681:	62 f1 7d 48 70 d2 4e 	vpshufd zmm2,zmm2,0x4e
  66c688:	49 83 f4 40          	xor    r12,0x40
  66c68c:	62 91 fe 48 6f 1c 27 	vmovdqu64 zmm3,ZMMWORD PTR [r15+r12*1]
  66c693:	e9 61 ff ff ff       	jmp    66c5f9 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x20e9>
  66c698:	62 f3 ed 48 43 d2 4e 	vshufi64x2 zmm2,zmm2,zmm2,0x4e
  66c69f:	49 83 f4 40          	xor    r12,0x40
  66c6a3:	62 91 fe 48 6f 1c 27 	vmovdqu64 zmm3,ZMMWORD PTR [r15+r12*1]
  66c6aa:	62 f3 e5 48 43 db 4e 	vshufi64x2 zmm3,zmm3,zmm3,0x4e
  66c6b1:	eb 4b                	jmp    66c6fe <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x21ee>
  66c6b3:	62 f3 ed 48 43 d2 1b 	vshufi64x2 zmm2,zmm2,zmm2,0x1b
  66c6ba:	49 83 f4 40          	xor    r12,0x40
  66c6be:	62 91 fe 48 6f 1c 27 	vmovdqu64 zmm3,ZMMWORD PTR [r15+r12*1]
  66c6c5:	62 f3 e5 48 43 db 1b 	vshufi64x2 zmm3,zmm3,zmm3,0x1b
  66c6cc:	eb 30                	jmp    66c6fe <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x21ee>
  66c6ce:	62 f2 c5 48 36 d2    	vpermq zmm2,zmm7,zmm2
  66c6d4:	49 83 f4 40          	xor    r12,0x40
  66c6d8:	62 91 fe 48 6f 1c 27 	vmovdqu64 zmm3,ZMMWORD PTR [r15+r12*1]
  66c6df:	62 f3 e5 48 43 db 1b 	vshufi64x2 zmm3,zmm3,zmm3,0x1b
  66c6e6:	e9 0e ff ff ff       	jmp    66c5f9 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x20e9>
  66c6eb:	62 f3 fd 48 00 d2 4e 	vpermq zmm2,zmm2,0x4e
  66c6f2:	49 83 f4 40          	xor    r12,0x40
  66c6f6:	62 93 fd 48 00 1c 27 	vpermq zmm3,ZMMWORD PTR [r15+r12*1],0x4e
  66c6fd:	4e 
  66c6fe:	62 f1 75 48 ef ca    	vpxord zmm1,zmm1,zmm2
  66c704:	62 f1 7d 48 ef c3    	vpxord zmm0,zmm0,zmm3
  66c70a:	49 ff c2             	inc    r10
  66c70d:	49 83 c3 08          	add    r11,0x8
  66c711:	49 83 fa 10          	cmp    r10,0x10
  66c715:	0f 85 02 ff ff ff    	jne    66c61d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x210d>
  66c71b:	eb 23                	jmp    66c740 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x2230>
  66c71d:	62 f3 fd 48 00 d2 1b 	vpermq zmm2,zmm2,0x1b
  66c724:	49 83 f4 40          	xor    r12,0x40
  66c728:	62 93 fd 48 00 1c 27 	vpermq zmm3,ZMMWORD PTR [r15+r12*1],0x4e
  66c72f:	4e 
  66c730:	e9 c4 fe ff ff       	jmp    66c5f9 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x20e9>
  66c735:	66 66 2e 0f 1f 84 00 	data16 cs nop WORD PTR [rax+rax*1+0x0]
  66c73c:	00 00 00 00 
  66c740:	45 0f b6 c9          	movzx  r9d,r9b
  66c744:	41 c1 e1 07          	shl    r9d,0x7
  66c748:	62 b1 fe 48 6f 1c 09 	vmovdqu64 zmm3,ZMMWORD PTR [rcx+r9*1]
  66c74f:	62 b1 fe 48 6f 54 09 	vmovdqu64 zmm2,ZMMWORD PTR [rcx+r9*1+0x40]
  66c756:	01 
  66c757:	41 b9 01 00 00 00    	mov    r9d,0x1
  66c75d:	41 ba 08 00 00 00    	mov    r10d,0x8
  66c763:	eb 3c                	jmp    66c7a1 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x2291>
  66c765:	62 f2 cd 48 36 e4    	vpermq zmm4,zmm6,zmm4
  66c76b:	49 83 f7 40          	xor    r15,0x40
  66c76f:	62 91 fe 48 6f 2c 3b 	vmovdqu64 zmm5,ZMMWORD PTR [r11+r15*1]
  66c776:	62 f3 d5 48 43 ed 4e 	vshufi64x2 zmm5,zmm5,zmm5,0x4e
  66c77d:	62 f1 5d 48 ef db    	vpxord zmm3,zmm4,zmm3
  66c783:	62 f1 7d 48 70 e5 4e 	vpshufd zmm4,zmm5,0x4e
  66c78a:	62 f1 5d 48 ef d2    	vpxord zmm2,zmm4,zmm2
  66c790:	49 ff c1             	inc    r9
  66c793:	49 83 c2 08          	add    r10,0x8
  66c797:	49 83 f9 10          	cmp    r9,0x10
  66c79b:	0f 84 af fd ff ff    	je     66c550 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x2040>
  66c7a1:	46 0f b6 1c 0a       	movzx  r11d,BYTE PTR [rdx+r9*1]
  66c7a6:	41 c1 e3 07          	shl    r11d,0x7
  66c7aa:	49 01 cb             	add    r11,rcx
  66c7ad:	45 89 cc             	mov    r12d,r9d
  66c7b0:	41 d1 ec             	shr    r12d,1
  66c7b3:	41 83 e4 03          	and    r12d,0x3
  66c7b7:	4d 89 d7             	mov    r15,r10
  66c7ba:	49 21 df             	and    r15,rbx
  66c7bd:	62 91 fe 48 6f 24 1f 	vmovdqu64 zmm4,ZMMWORD PTR [r15+r11*1]
  66c7c4:	41 f6 c1 01          	test   r9b,0x1
  66c7c8:	75 26                	jne    66c7f0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x22e0>
  66c7ca:	48 8d 05 4f 60 9a ff 	lea    rax,[rip+0xffffffffff9a604f]        # 12820 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x16d2>
  66c7d1:	4e 63 24 a0          	movsxd r12,DWORD PTR [rax+r12*4]
  66c7d5:	49 01 c4             	add    r12,rax
  66c7d8:	41 ff e4             	jmp    r12
  66c7db:	49 83 f7 40          	xor    r15,0x40
  66c7df:	62 91 fe 48 6f 2c 3b 	vmovdqu64 zmm5,ZMMWORD PTR [r11+r15*1]
  66c7e6:	e9 93 00 00 00       	jmp    66c87e <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x236e>
  66c7eb:	0f 1f 44 00 00       	nop    DWORD PTR [rax+rax*1+0x0]
  66c7f0:	48 8d 05 19 60 9a ff 	lea    rax,[rip+0xffffffffff9a6019]        # 12810 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x16c2>
  66c7f7:	4e 63 24 a0          	movsxd r12,DWORD PTR [rax+r12*4]
  66c7fb:	49 01 c4             	add    r12,rax
  66c7fe:	41 ff e4             	jmp    r12
  66c801:	62 f1 7d 48 70 e4 4e 	vpshufd zmm4,zmm4,0x4e
  66c808:	49 83 f7 40          	xor    r15,0x40
  66c80c:	62 91 fe 48 6f 2c 3b 	vmovdqu64 zmm5,ZMMWORD PTR [r11+r15*1]
  66c813:	e9 65 ff ff ff       	jmp    66c77d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x226d>
  66c818:	62 f3 dd 48 43 e4 4e 	vshufi64x2 zmm4,zmm4,zmm4,0x4e
  66c81f:	49 83 f7 40          	xor    r15,0x40
  66c823:	62 91 fe 48 6f 2c 3b 	vmovdqu64 zmm5,ZMMWORD PTR [r11+r15*1]
  66c82a:	62 f3 d5 48 43 ed 4e 	vshufi64x2 zmm5,zmm5,zmm5,0x4e
  66c831:	eb 4b                	jmp    66c87e <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x236e>
  66c833:	62 f3 dd 48 43 e4 1b 	vshufi64x2 zmm4,zmm4,zmm4,0x1b
  66c83a:	49 83 f7 40          	xor    r15,0x40
  66c83e:	62 91 fe 48 6f 2c 3b 	vmovdqu64 zmm5,ZMMWORD PTR [r11+r15*1]
  66c845:	62 f3 d5 48 43 ed 1b 	vshufi64x2 zmm5,zmm5,zmm5,0x1b
  66c84c:	eb 30                	jmp    66c87e <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x236e>
  66c84e:	62 f2 c5 48 36 e4    	vpermq zmm4,zmm7,zmm4
  66c854:	49 83 f7 40          	xor    r15,0x40
  66c858:	62 91 fe 48 6f 2c 3b 	vmovdqu64 zmm5,ZMMWORD PTR [r11+r15*1]
  66c85f:	62 f3 d5 48 43 ed 1b 	vshufi64x2 zmm5,zmm5,zmm5,0x1b
  66c866:	e9 12 ff ff ff       	jmp    66c77d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x226d>
  66c86b:	62 f3 fd 48 00 e4 4e 	vpermq zmm4,zmm4,0x4e
  66c872:	49 83 f7 40          	xor    r15,0x40
  66c876:	62 93 fd 48 00 2c 3b 	vpermq zmm5,ZMMWORD PTR [r11+r15*1],0x4e
  66c87d:	4e 
  66c87e:	62 f1 65 48 ef dc    	vpxord zmm3,zmm3,zmm4
  66c884:	62 f1 6d 48 ef d5    	vpxord zmm2,zmm2,zmm5
  66c88a:	49 ff c1             	inc    r9
  66c88d:	49 83 c2 08          	add    r10,0x8
  66c891:	49 83 f9 10          	cmp    r9,0x10
  66c895:	0f 85 06 ff ff ff    	jne    66c7a1 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x2291>
  66c89b:	e9 b0 fc ff ff       	jmp    66c550 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x2040>
  66c8a0:	62 f3 fd 48 00 e4 1b 	vpermq zmm4,zmm4,0x1b
  66c8a7:	49 83 f7 40          	xor    r15,0x40
  66c8ab:	62 93 fd 48 00 2c 3b 	vpermq zmm5,ZMMWORD PTR [r11+r15*1],0x4e
  66c8b2:	4e 
  66c8b3:	e9 c5 fe ff ff       	jmp    66c77d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x226d>
  66c8b8:	0f 1f 84 00 00 00 00 	nop    DWORD PTR [rax+rax*1+0x0]
  66c8bf:	00 
  66c8c0:	44 89 f0             	mov    eax,r14d
  66c8c3:	c1 e0 07             	shl    eax,0x7
  66c8c6:	41 c1 e6 04          	shl    r14d,0x4
  66c8ca:	48 ff 44 24 18       	inc    QWORD PTR [rsp+0x18]
  66c8cf:	48 c7 84 24 08 02 00 	mov    QWORD PTR [rsp+0x208],0x2
  66c8d6:	00 02 00 00 00 
  66c8db:	48 c7 84 24 10 02 00 	mov    QWORD PTR [rsp+0x210],0x0
  66c8e2:	00 00 00 00 00 
  66c8e7:	48 c7 84 24 18 02 00 	mov    QWORD PTR [rsp+0x218],0x8
  66c8ee:	00 08 00 00 00 
  66c8f3:	31 f6                	xor    esi,esi
  66c8f5:	31 c9                	xor    ecx,ecx
  66c8f7:	e9 8b 00 00 00       	jmp    66c987 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x2477>
  66c8fc:	0f 1f 40 00          	nop    DWORD PTR [rax+0x0]
  66c900:	4c 8b 6c 24 40       	mov    r13,QWORD PTR [rsp+0x40]
  66c905:	62 71 fe 48 7f 7a 02 	vmovdqu64 ZMMWORD PTR [rdx+0x80],zmm15
  66c90c:	62 71 fe 48 7f 72 03 	vmovdqu64 ZMMWORD PTR [rdx+0xc0],zmm14
  66c913:	62 71 fe 48 7f 6e 02 	vmovdqu64 ZMMWORD PTR [rsi+0x80],zmm13
  66c91a:	62 71 fe 48 7f 66 03 	vmovdqu64 ZMMWORD PTR [rsi+0xc0],zmm12
  66c921:	62 51 fe 48 7f 58 02 	vmovdqu64 ZMMWORD PTR [r8+0x80],zmm11
  66c928:	62 51 fe 48 7f 50 03 	vmovdqu64 ZMMWORD PTR [r8+0xc0],zmm10
  66c92f:	62 51 fe 48 7f 49 02 	vmovdqu64 ZMMWORD PTR [r9+0x80],zmm9
  66c936:	62 51 fe 48 7f 41 03 	vmovdqu64 ZMMWORD PTR [r9+0xc0],zmm8
  66c93d:	62 d1 fe 48 7f 7a 02 	vmovdqu64 ZMMWORD PTR [r10+0x80],zmm7
  66c944:	62 d1 fe 48 7f 72 03 	vmovdqu64 ZMMWORD PTR [r10+0xc0],zmm6
  66c94b:	62 d1 fe 48 7f 6b 02 	vmovdqu64 ZMMWORD PTR [r11+0x80],zmm5
  66c952:	62 d1 fe 48 7f 63 03 	vmovdqu64 ZMMWORD PTR [r11+0xc0],zmm4
  66c959:	62 d1 fe 48 7f 5f 02 	vmovdqu64 ZMMWORD PTR [r15+0x80],zmm3
  66c960:	62 d1 fe 48 7f 57 03 	vmovdqu64 ZMMWORD PTR [r15+0xc0],zmm2
  66c967:	62 f1 fe 48 7f 49 02 	vmovdqu64 ZMMWORD PTR [rcx+0x80],zmm1
  66c96e:	62 f1 fe 48 7f 41 03 	vmovdqu64 ZMMWORD PTR [rcx+0xc0],zmm0
  66c975:	b9 01 00 00 00       	mov    ecx,0x1
  66c97a:	41 f6 c5 01          	test   r13b,0x1
  66c97e:	40 b6 01             	mov    sil,0x1
  66c981:	0f 85 39 fa ff ff    	jne    66c3c0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1eb0>
  66c987:	48 8b bc cc 10 02 00 	mov    rdi,QWORD PTR [rsp+rcx*8+0x210]
  66c98e:	00 
  66c98f:	b9 10 00 00 00       	mov    ecx,0x10
  66c994:	48 29 f9             	sub    rcx,rdi
  66c997:	ba 00 00 00 00       	mov    edx,0x0
  66c99c:	48 0f 43 d1          	cmovae rdx,rcx
  66c9a0:	48 83 ff 10          	cmp    rdi,0x10
  66c9a4:	0f 83 75 37 00 00    	jae    67011f <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5c0f>
  66c9aa:	4c 8d 4f 01          	lea    r9,[rdi+0x1]
  66c9ae:	48 83 fa 01          	cmp    rdx,0x1
  66c9b2:	0f 84 d4 36 00 00    	je     67008c <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5b7c>
  66c9b8:	4c 8d 47 02          	lea    r8,[rdi+0x2]
  66c9bc:	48 83 fa 02          	cmp    rdx,0x2
  66c9c0:	0f 84 96 36 00 00    	je     67005c <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5b4c>
  66c9c6:	4c 8d 57 03          	lea    r10,[rdi+0x3]
  66c9ca:	48 83 fa 03          	cmp    rdx,0x3
  66c9ce:	0f 84 28 36 00 00    	je     66fffc <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5aec>
  66c9d4:	4c 8d 7f 04          	lea    r15,[rdi+0x4]
  66c9d8:	48 83 fa 04          	cmp    rdx,0x4
  66c9dc:	0f 84 3a 37 00 00    	je     67011c <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5c0c>
  66c9e2:	4c 8d 5f 05          	lea    r11,[rdi+0x5]
  66c9e6:	48 83 fa 05          	cmp    rdx,0x5
  66c9ea:	0f 84 fc 36 00 00    	je     6700ec <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5bdc>
  66c9f0:	48 8d 4f 06          	lea    rcx,[rdi+0x6]
  66c9f4:	48 83 fa 06          	cmp    rdx,0x6
  66c9f8:	0f 84 be 36 00 00    	je     6700bc <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5bac>
  66c9fe:	4c 8d 67 07          	lea    r12,[rdi+0x7]
  66ca02:	48 83 fa 07          	cmp    rdx,0x7
  66ca06:	0f 84 08 36 00 00    	je     670014 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5b04>
  66ca0c:	48 89 74 24 40       	mov    QWORD PTR [rsp+0x40],rsi
  66ca11:	48 89 fe             	mov    rsi,rdi
  66ca14:	48 c1 e6 08          	shl    rsi,0x8
  66ca18:	48 8d 94 34 c0 02 00 	lea    rdx,[rsp+rsi*1+0x2c0]
  66ca1f:	00 
  66ca20:	62 f1 fe 48 6f 44 34 	vmovdqu64 zmm0,ZMMWORD PTR [rsp+rsi*1+0x2c0]
  66ca27:	0b 
  66ca28:	49 c1 e1 08          	shl    r9,0x8
  66ca2c:	4a 8d b4 0c c0 02 00 	lea    rsi,[rsp+r9*1+0x2c0]
  66ca33:	00 
  66ca34:	62 b1 fe 48 6f 4c 0c 	vmovdqu64 zmm1,ZMMWORD PTR [rsp+r9*1+0x2c0]
  66ca3b:	0b 
  66ca3c:	49 c1 e0 08          	shl    r8,0x8
  66ca40:	62 b1 fe 48 6f 54 04 	vmovdqu64 zmm2,ZMMWORD PTR [rsp+r8*1+0x2c0]
  66ca47:	0b 
  66ca48:	4e 8d 84 04 c0 02 00 	lea    r8,[rsp+r8*1+0x2c0]
  66ca4f:	00 
  66ca50:	49 c1 e2 08          	shl    r10,0x8
  66ca54:	4e 8d 8c 14 c0 02 00 	lea    r9,[rsp+r10*1+0x2c0]
  66ca5b:	00 
  66ca5c:	62 b1 fe 48 6f 5c 14 	vmovdqu64 zmm3,ZMMWORD PTR [rsp+r10*1+0x2c0]
  66ca63:	0b 
  66ca64:	49 c1 e7 08          	shl    r15,0x8
  66ca68:	4e 8d 94 3c c0 02 00 	lea    r10,[rsp+r15*1+0x2c0]
  66ca6f:	00 
  66ca70:	62 b1 fe 48 6f 64 3c 	vmovdqu64 zmm4,ZMMWORD PTR [rsp+r15*1+0x2c0]
  66ca77:	0b 
  66ca78:	49 c1 e3 08          	shl    r11,0x8
  66ca7c:	62 b1 fe 48 6f 6c 1c 	vmovdqu64 zmm5,ZMMWORD PTR [rsp+r11*1+0x2c0]
  66ca83:	0b 
  66ca84:	48 c1 e1 08          	shl    rcx,0x8
  66ca88:	49 c1 e4 08          	shl    r12,0x8
  66ca8c:	62 71 fe 48 6f 4c 0c 	vmovdqu64 zmm9,ZMMWORD PTR [rsp+rcx*1+0x2c0]
  66ca93:	0b 
  66ca94:	62 71 fe 48 6f 44 0c 	vmovdqu64 zmm8,ZMMWORD PTR [rsp+rcx*1+0x300]
  66ca9b:	0c 
  66ca9c:	62 b1 fe 48 6f 7c 24 	vmovdqu64 zmm7,ZMMWORD PTR [rsp+r12*1+0x2c0]
  66caa3:	0b 
  66caa4:	62 b1 fe 48 6f 74 24 	vmovdqu64 zmm6,ZMMWORD PTR [rsp+r12*1+0x300]
  66caab:	0c 
  66caac:	62 71 fe 48 6f 7a 01 	vmovdqu64 zmm15,ZMMWORD PTR [rdx+0x40]
  66cab3:	62 71 fe 48 6f 76 01 	vmovdqu64 zmm14,ZMMWORD PTR [rsi+0x40]
  66caba:	62 51 fe 48 6f 68 01 	vmovdqu64 zmm13,ZMMWORD PTR [r8+0x40]
  66cac1:	62 51 fe 48 6f 61 01 	vmovdqu64 zmm12,ZMMWORD PTR [r9+0x40]
  66cac8:	4e 8d 9c 1c c0 02 00 	lea    r11,[rsp+r11*1+0x2c0]
  66cacf:	00 
  66cad0:	62 51 fe 48 6f 5a 01 	vmovdqu64 zmm11,ZMMWORD PTR [r10+0x40]
  66cad7:	62 51 fe 48 6f 53 01 	vmovdqu64 zmm10,ZMMWORD PTR [r11+0x40]
  66cade:	4c 8b bc 24 a0 00 00 	mov    r15,QWORD PTR [rsp+0xa0]
  66cae5:	00 
  66cae6:	4c 39 bc 24 c0 00 00 	cmp    QWORD PTR [rsp+0xc0],r15
  66caed:	00 
  66caee:	0f 84 3c 01 00 00    	je     66cc30 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x2720>
  66caf4:	4c 8d bc 24 c7 16 00 	lea    r15,[rsp+0x16c7]
  66cafb:	00 
  66cafc:	49 01 ff             	add    r15,rdi
  66caff:	45 31 ed             	xor    r13d,r13d
  66cb02:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  66cb09:	1f 84 00 00 00 00 00 
  66cb10:	62 82 7d 48 78 44 2f 	vpbroadcastb zmm16,BYTE PTR [r15+r13*1-0x7]
  66cb17:	f9 
  66cb18:	c4 a1 f8 90 94 2c c0 	kmovq  k2,QWORD PTR [rsp+r13*1+0x12c0]
  66cb1f:	12 00 00 
  66cb22:	c4 a1 f8 90 8c 2c c8 	kmovq  k1,QWORD PTR [rsp+r13*1+0x12c8]
  66cb29:	12 00 00 
  66cb2c:	62 a1 7f ca 6f c8    	vmovdqu8 zmm17{k2}{z},zmm16
  66cb32:	62 b1 7d 48 ef c1    	vpxord zmm0,zmm0,zmm17
  66cb38:	62 a1 7f c9 6f c0    	vmovdqu8 zmm16{k1}{z},zmm16
  66cb3e:	62 31 05 48 ef f8    	vpxord zmm15,zmm15,zmm16
  66cb44:	62 82 7d 48 78 44 2f 	vpbroadcastb zmm16,BYTE PTR [r15+r13*1-0x6]
  66cb4b:	fa 
  66cb4c:	62 a1 7f ca 6f c8    	vmovdqu8 zmm17{k2}{z},zmm16
  66cb52:	62 b1 75 48 ef c9    	vpxord zmm1,zmm1,zmm17
  66cb58:	62 a1 7f c9 6f c0    	vmovdqu8 zmm16{k1}{z},zmm16
  66cb5e:	62 31 0d 48 ef f0    	vpxord zmm14,zmm14,zmm16
  66cb64:	62 82 7d 48 78 44 2f 	vpbroadcastb zmm16,BYTE PTR [r15+r13*1-0x5]
  66cb6b:	fb 
  66cb6c:	62 a1 7f ca 6f c8    	vmovdqu8 zmm17{k2}{z},zmm16
  66cb72:	62 b1 6d 48 ef d1    	vpxord zmm2,zmm2,zmm17
  66cb78:	62 a1 7f c9 6f c0    	vmovdqu8 zmm16{k1}{z},zmm16
  66cb7e:	62 31 15 48 ef e8    	vpxord zmm13,zmm13,zmm16
  66cb84:	62 82 7d 48 78 44 2f 	vpbroadcastb zmm16,BYTE PTR [r15+r13*1-0x4]
  66cb8b:	fc 
  66cb8c:	62 a1 7f ca 6f c8    	vmovdqu8 zmm17{k2}{z},zmm16
  66cb92:	62 b1 65 48 ef d9    	vpxord zmm3,zmm3,zmm17
  66cb98:	62 a1 7f c9 6f c0    	vmovdqu8 zmm16{k1}{z},zmm16
  66cb9e:	62 31 1d 48 ef e0    	vpxord zmm12,zmm12,zmm16
  66cba4:	62 82 7d 48 78 44 2f 	vpbroadcastb zmm16,BYTE PTR [r15+r13*1-0x3]
  66cbab:	fd 
  66cbac:	62 a1 7f ca 6f c8    	vmovdqu8 zmm17{k2}{z},zmm16
  66cbb2:	62 b1 5d 48 ef e1    	vpxord zmm4,zmm4,zmm17
  66cbb8:	62 a1 7f c9 6f c0    	vmovdqu8 zmm16{k1}{z},zmm16
  66cbbe:	62 31 25 48 ef d8    	vpxord zmm11,zmm11,zmm16
  66cbc4:	62 82 7d 48 78 44 2f 	vpbroadcastb zmm16,BYTE PTR [r15+r13*1-0x2]
  66cbcb:	fe 
  66cbcc:	62 a1 7f ca 6f c8    	vmovdqu8 zmm17{k2}{z},zmm16
  66cbd2:	62 b1 55 48 ef e9    	vpxord zmm5,zmm5,zmm17
  66cbd8:	62 a1 7f c9 6f c0    	vmovdqu8 zmm16{k1}{z},zmm16
  66cbde:	62 31 2d 48 ef d0    	vpxord zmm10,zmm10,zmm16
  66cbe4:	62 82 7d 48 78 44 2f 	vpbroadcastb zmm16,BYTE PTR [r15+r13*1-0x1]
  66cbeb:	ff 
  66cbec:	62 a1 7f ca 6f c8    	vmovdqu8 zmm17{k2}{z},zmm16
  66cbf2:	62 31 35 48 ef c9    	vpxord zmm9,zmm9,zmm17
  66cbf8:	62 a1 7f c9 6f c0    	vmovdqu8 zmm16{k1}{z},zmm16
  66cbfe:	62 31 3d 48 ef c0    	vpxord zmm8,zmm8,zmm16
  66cc04:	62 82 7d 48 78 04 2f 	vpbroadcastb zmm16,BYTE PTR [r15+r13*1]
  66cc0b:	62 a1 7f ca 6f c8    	vmovdqu8 zmm17{k2}{z},zmm16
  66cc11:	62 b1 45 48 ef f9    	vpxord zmm7,zmm7,zmm17
  66cc17:	62 a1 7f c9 6f c0    	vmovdqu8 zmm16{k1}{z},zmm16
  66cc1d:	62 b1 4d 48 ef f0    	vpxord zmm6,zmm6,zmm16
  66cc23:	49 83 c5 10          	add    r13,0x10
  66cc27:	4d 39 ee             	cmp    r14,r13
  66cc2a:	0f 85 e0 fe ff ff    	jne    66cb10 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x2600>
  66cc30:	4c 8d bc 0c c0 02 00 	lea    r15,[rsp+rcx*1+0x2c0]
  66cc37:	00 
  66cc38:	4a 8d 8c 24 c0 02 00 	lea    rcx,[rsp+r12*1+0x2c0]
  66cc3f:	00 
  66cc40:	62 f1 fe 48 7f 02    	vmovdqu64 ZMMWORD PTR [rdx],zmm0
  66cc46:	62 71 fe 48 7f 7a 01 	vmovdqu64 ZMMWORD PTR [rdx+0x40],zmm15
  66cc4d:	62 f1 fe 48 7f 0e    	vmovdqu64 ZMMWORD PTR [rsi],zmm1
  66cc53:	62 71 fe 48 7f 76 01 	vmovdqu64 ZMMWORD PTR [rsi+0x40],zmm14
  66cc5a:	62 d1 fe 48 7f 10    	vmovdqu64 ZMMWORD PTR [r8],zmm2
  66cc60:	62 51 fe 48 7f 68 01 	vmovdqu64 ZMMWORD PTR [r8+0x40],zmm13
  66cc67:	62 d1 fe 48 7f 19    	vmovdqu64 ZMMWORD PTR [r9],zmm3
  66cc6d:	62 51 fe 48 7f 61 01 	vmovdqu64 ZMMWORD PTR [r9+0x40],zmm12
  66cc74:	62 d1 fe 48 7f 22    	vmovdqu64 ZMMWORD PTR [r10],zmm4
  66cc7a:	62 51 fe 48 7f 5a 01 	vmovdqu64 ZMMWORD PTR [r10+0x40],zmm11
  66cc81:	62 d1 fe 48 7f 2b    	vmovdqu64 ZMMWORD PTR [r11],zmm5
  66cc87:	62 51 fe 48 7f 53 01 	vmovdqu64 ZMMWORD PTR [r11+0x40],zmm10
  66cc8e:	62 51 fe 48 7f 0f    	vmovdqu64 ZMMWORD PTR [r15],zmm9
  66cc94:	62 51 fe 48 7f 47 01 	vmovdqu64 ZMMWORD PTR [r15+0x40],zmm8
  66cc9b:	62 f1 fe 48 7f 39    	vmovdqu64 ZMMWORD PTR [rcx],zmm7
  66cca1:	62 f1 fe 48 7f 71 01 	vmovdqu64 ZMMWORD PTR [rcx+0x40],zmm6
  66cca8:	62 71 fe 48 6f 7a 02 	vmovdqu64 zmm15,ZMMWORD PTR [rdx+0x80]
  66ccaf:	62 71 fe 48 6f 72 03 	vmovdqu64 zmm14,ZMMWORD PTR [rdx+0xc0]
  66ccb6:	62 71 fe 48 6f 6e 02 	vmovdqu64 zmm13,ZMMWORD PTR [rsi+0x80]
  66ccbd:	62 71 fe 48 6f 66 03 	vmovdqu64 zmm12,ZMMWORD PTR [rsi+0xc0]
  66ccc4:	62 51 fe 48 6f 58 02 	vmovdqu64 zmm11,ZMMWORD PTR [r8+0x80]
  66cccb:	62 51 fe 48 6f 50 03 	vmovdqu64 zmm10,ZMMWORD PTR [r8+0xc0]
  66ccd2:	62 51 fe 48 6f 49 02 	vmovdqu64 zmm9,ZMMWORD PTR [r9+0x80]
  66ccd9:	62 51 fe 48 6f 41 03 	vmovdqu64 zmm8,ZMMWORD PTR [r9+0xc0]
  66cce0:	62 d1 fe 48 6f 7a 02 	vmovdqu64 zmm7,ZMMWORD PTR [r10+0x80]
  66cce7:	62 d1 fe 48 6f 72 03 	vmovdqu64 zmm6,ZMMWORD PTR [r10+0xc0]
  66ccee:	62 d1 fe 48 6f 6b 02 	vmovdqu64 zmm5,ZMMWORD PTR [r11+0x80]
  66ccf5:	62 d1 fe 48 6f 63 03 	vmovdqu64 zmm4,ZMMWORD PTR [r11+0xc0]
  66ccfc:	62 d1 fe 48 6f 5f 02 	vmovdqu64 zmm3,ZMMWORD PTR [r15+0x80]
  66cd03:	62 d1 fe 48 6f 57 03 	vmovdqu64 zmm2,ZMMWORD PTR [r15+0xc0]
  66cd0a:	62 f1 fe 48 6f 49 02 	vmovdqu64 zmm1,ZMMWORD PTR [rcx+0x80]
  66cd11:	62 f1 fe 48 6f 41 03 	vmovdqu64 zmm0,ZMMWORD PTR [rcx+0xc0]
  66cd18:	4c 8b a4 24 a0 00 00 	mov    r12,QWORD PTR [rsp+0xa0]
  66cd1f:	00 
  66cd20:	4c 39 a4 24 c0 00 00 	cmp    QWORD PTR [rsp+0xc0],r12
  66cd27:	00 
  66cd28:	0f 84 d2 fb ff ff    	je     66c900 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x23f0>
  66cd2e:	4c 8d a4 24 c7 16 00 	lea    r12,[rsp+0x16c7]
  66cd35:	00 
  66cd36:	4c 01 e7             	add    rdi,r12
  66cd39:	45 31 e4             	xor    r12d,r12d
  66cd3c:	4c 8b 6c 24 40       	mov    r13,QWORD PTR [rsp+0x40]
  66cd41:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  66cd48:	0f 1f 84 00 00 00 00 
  66cd4f:	00 
  66cd50:	62 e2 7d 48 78 47 f9 	vpbroadcastb zmm16,BYTE PTR [rdi-0x7]
  66cd57:	62 a1 fd 48 6f 4c 24 	vmovdqa64 zmm17,ZMMWORD PTR [rsp+r12*1+0x1ac0]
  66cd5e:	6b 
  66cd5f:	62 a1 fd 48 6f 54 24 	vmovdqa64 zmm18,ZMMWORD PTR [rsp+r12*1+0x1b00]
  66cd66:	6c 
  66cd67:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66cd6d:	62 31 05 48 ef fb    	vpxord zmm15,zmm15,zmm19
  66cd73:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66cd79:	62 31 0d 48 ef f0    	vpxord zmm14,zmm14,zmm16
  66cd7f:	62 e2 7d 48 78 47 fa 	vpbroadcastb zmm16,BYTE PTR [rdi-0x6]
  66cd86:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66cd8c:	62 31 15 48 ef eb    	vpxord zmm13,zmm13,zmm19
  66cd92:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66cd98:	62 31 1d 48 ef e0    	vpxord zmm12,zmm12,zmm16
  66cd9e:	62 e2 7d 48 78 47 fb 	vpbroadcastb zmm16,BYTE PTR [rdi-0x5]
  66cda5:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66cdab:	62 31 25 48 ef db    	vpxord zmm11,zmm11,zmm19
  66cdb1:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66cdb7:	62 31 2d 48 ef d0    	vpxord zmm10,zmm10,zmm16
  66cdbd:	62 e2 7d 48 78 47 fc 	vpbroadcastb zmm16,BYTE PTR [rdi-0x4]
  66cdc4:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66cdca:	62 31 35 48 ef cb    	vpxord zmm9,zmm9,zmm19
  66cdd0:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66cdd6:	62 31 3d 48 ef c0    	vpxord zmm8,zmm8,zmm16
  66cddc:	62 e2 7d 48 78 47 fd 	vpbroadcastb zmm16,BYTE PTR [rdi-0x3]
  66cde3:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66cde9:	62 b1 45 48 ef fb    	vpxord zmm7,zmm7,zmm19
  66cdef:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66cdf5:	62 b1 4d 48 ef f0    	vpxord zmm6,zmm6,zmm16
  66cdfb:	62 e2 7d 48 78 47 fe 	vpbroadcastb zmm16,BYTE PTR [rdi-0x2]
  66ce02:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66ce08:	62 b1 55 48 ef eb    	vpxord zmm5,zmm5,zmm19
  66ce0e:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66ce14:	62 b1 5d 48 ef e0    	vpxord zmm4,zmm4,zmm16
  66ce1a:	62 e2 7d 48 78 47 ff 	vpbroadcastb zmm16,BYTE PTR [rdi-0x1]
  66ce21:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66ce27:	62 b1 65 48 ef db    	vpxord zmm3,zmm3,zmm19
  66ce2d:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66ce33:	62 b1 6d 48 ef d0    	vpxord zmm2,zmm2,zmm16
  66ce39:	62 e2 7d 48 78 07    	vpbroadcastb zmm16,BYTE PTR [rdi]
  66ce3f:	62 a2 75 40 cf c8    	vgf2p8mulb zmm17,zmm17,zmm16
  66ce45:	62 b1 75 48 ef c9    	vpxord zmm1,zmm1,zmm17
  66ce4b:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66ce51:	62 b1 7d 48 ef c0    	vpxord zmm0,zmm0,zmm16
  66ce57:	48 83 c7 10          	add    rdi,0x10
  66ce5b:	49 83 ec 80          	sub    r12,0xffffffffffffff80
  66ce5f:	4c 39 e0             	cmp    rax,r12
  66ce62:	0f 85 e8 fe ff ff    	jne    66cd50 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x2840>
  66ce68:	e9 98 fa ff ff       	jmp    66c905 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x23f5>
  66ce6d:	8b 05 dd de 09 00    	mov    eax,DWORD PTR [rip+0x9dedd]        # 70ad50 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1120>
  66ce73:	85 c0                	test   eax,eax
  66ce75:	0f 85 bd 30 00 00    	jne    66ff38 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5a28>
  66ce7b:	4d 89 f4             	mov    r12,r14
  66ce7e:	48 89 4c 24 30       	mov    QWORD PTR [rsp+0x30],rcx
  66ce83:	48 89 7c 24 50       	mov    QWORD PTR [rsp+0x50],rdi
  66ce88:	8b 05 9a cd 09 00    	mov    eax,DWORD PTR [rip+0x9cd9a]        # 709c28 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290+0x930>
  66ce8e:	85 c0                	test   eax,eax
  66ce90:	0f 85 be 30 00 00    	jne    66ff54 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5a44>
  66ce96:	48 8d bc 24 c0 02 00 	lea    rdi,[rsp+0x2c0]
  66ce9d:	00 
  66ce9e:	31 db                	xor    ebx,ebx
  66cea0:	4c 8b 35 11 a5 09 00 	mov    r14,QWORD PTR [rip+0x9a511]        # 7073b8 <memset@GLIBC_2.2.5>
  66cea7:	ba 00 10 00 00       	mov    edx,0x1000
  66ceac:	31 f6                	xor    esi,esi
  66ceae:	41 ff d6             	call   r14
  66ceb1:	48 8d bc 24 c0 16 00 	lea    rdi,[rsp+0x16c0]
  66ceb8:	00 
  66ceb9:	ba 00 04 00 00       	mov    edx,0x400
  66cebe:	31 f6                	xor    esi,esi
  66cec0:	41 ff d6             	call   r14
  66cec3:	48 8d bc 24 c0 12 00 	lea    rdi,[rsp+0x12c0]
  66ceca:	00 
  66cecb:	ba 00 04 00 00       	mov    edx,0x400
  66ced0:	31 f6                	xor    esi,esi
  66ced2:	41 ff d6             	call   r14
  66ced5:	4d 85 ff             	test   r15,r15
  66ced8:	4c 89 7c 24 38       	mov    QWORD PTR [rsp+0x38],r15
  66cedd:	74 21                	je     66cf00 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x29f0>
  66cedf:	48 8b 44 24 28       	mov    rax,QWORD PTR [rsp+0x28]
  66cee4:	48 05 ff 03 00 00    	add    rax,0x3ff
  66ceea:	48 c1 e8 0a          	shr    rax,0xa
  66ceee:	49 8d 9f ff 03 00 00 	lea    rbx,[r15+0x3ff]
  66cef5:	48 c1 eb 0a          	shr    rbx,0xa
  66cef9:	48 39 c3             	cmp    rbx,rax
  66cefc:	48 0f 43 d8          	cmovae rbx,rax
  66cf00:	4d 89 e6             	mov    r14,r12
  66cf03:	48 83 bc 24 c0 00 00 	cmp    QWORD PTR [rsp+0xc0],0x0
  66cf0a:	00 00 
  66cf0c:	0f 84 eb 0a 00 00    	je     66d9fd <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x34ed>
  66cf12:	48 8b 84 24 c0 00 00 	mov    rax,QWORD PTR [rsp+0xc0]
  66cf19:	00 
  66cf1a:	48 83 c0 3f          	add    rax,0x3f
  66cf1e:	48 c1 e8 06          	shr    rax,0x6
  66cf22:	48 39 d8             	cmp    rax,rbx
  66cf25:	48 0f 43 c3          	cmovae rax,rbx
  66cf29:	48 89 84 24 80 00 00 	mov    QWORD PTR [rsp+0x80],rax
  66cf30:	00 
  66cf31:	48 85 c0             	test   rax,rax
  66cf34:	0f 84 c3 0a 00 00    	je     66d9fd <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x34ed>
  66cf3a:	31 f6                	xor    esi,esi
  66cf3c:	48 8b bc 24 c0 00 00 	mov    rdi,QWORD PTR [rsp+0xc0]
  66cf43:	00 
  66cf44:	4c 8b 44 24 38       	mov    r8,QWORD PTR [rsp+0x38]
  66cf49:	4c 8b 4c 24 28       	mov    r9,QWORD PTR [rsp+0x28]
  66cf4e:	e9 c3 00 00 00       	jmp    66d016 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x2b06>
  66cf53:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  66cf5a:	84 00 00 00 00 00 
  66cf60:	48 ff c6             	inc    rsi
  66cf63:	62 71 fe 48 7f 54 24 	vmovdqu64 ZMMWORD PTR [rsp+0xb40],zmm10
  66cf6a:	2d 
  66cf6b:	62 71 fe 48 7f 4c 24 	vmovdqu64 ZMMWORD PTR [rsp+0xb80],zmm9
  66cf72:	2e 
  66cf73:	62 71 fe 48 7f 44 24 	vmovdqu64 ZMMWORD PTR [rsp+0xc40],zmm8
  66cf7a:	31 
  66cf7b:	62 f1 fe 48 7f 7c 24 	vmovdqu64 ZMMWORD PTR [rsp+0xc80],zmm7
  66cf82:	32 
  66cf83:	62 f1 fe 48 7f 74 24 	vmovdqu64 ZMMWORD PTR [rsp+0xd40],zmm6
  66cf8a:	35 
  66cf8b:	62 71 fe 48 7f 7c 24 	vmovdqu64 ZMMWORD PTR [rsp+0xd80],zmm15
  66cf92:	36 
  66cf93:	62 f1 fe 48 7f 64 24 	vmovdqu64 ZMMWORD PTR [rsp+0xe40],zmm4
  66cf9a:	39 
  66cf9b:	62 71 fe 48 7f 74 24 	vmovdqu64 ZMMWORD PTR [rsp+0xe80],zmm14
  66cfa2:	3a 
  66cfa3:	62 f1 fe 48 7f 5c 24 	vmovdqu64 ZMMWORD PTR [rsp+0xf40],zmm3
  66cfaa:	3d 
  66cfab:	62 71 fe 48 7f 6c 24 	vmovdqu64 ZMMWORD PTR [rsp+0xf80],zmm13
  66cfb2:	3e 
  66cfb3:	62 f1 fe 48 7f 54 24 	vmovdqu64 ZMMWORD PTR [rsp+0x1040],zmm2
  66cfba:	41 
  66cfbb:	62 71 fe 48 7f 64 24 	vmovdqu64 ZMMWORD PTR [rsp+0x1080],zmm12
  66cfc2:	42 
  66cfc3:	62 f1 fe 48 7f 4c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x1140],zmm1
  66cfca:	45 
  66cfcb:	62 71 fe 48 7f 5c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x1180],zmm11
  66cfd2:	46 
  66cfd3:	62 f1 fe 48 7f 44 24 	vmovdqu64 ZMMWORD PTR [rsp+0x1240],zmm0
  66cfda:	49 
  66cfdb:	62 f1 fe 48 7f 6c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x1280],zmm5
  66cfe2:	4a 
  66cfe3:	49 81 c1 00 fc ff ff 	add    r9,0xfffffffffffffc00
  66cfea:	49 81 c0 00 fc ff ff 	add    r8,0xfffffffffffffc00
  66cff1:	49 81 c6 00 04 00 00 	add    r14,0x400
  66cff8:	48 81 c3 00 04 00 00 	add    rbx,0x400
  66cfff:	48 89 5c 24 30       	mov    QWORD PTR [rsp+0x30],rbx
  66d004:	48 83 c7 c0          	add    rdi,0xffffffffffffffc0
  66d008:	48 3b b4 24 80 00 00 	cmp    rsi,QWORD PTR [rsp+0x80]
  66d00f:	00 
  66d010:	0f 84 e7 09 00 00    	je     66d9fd <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x34ed>
  66d016:	48 83 ff 40          	cmp    rdi,0x40
  66d01a:	41 bd 40 00 00 00    	mov    r13d,0x40
  66d020:	48 89 7c 24 40       	mov    QWORD PTR [rsp+0x40],rdi
  66d025:	4c 0f 42 ef          	cmovb  r13,rdi
  66d029:	49 81 f8 00 04 00 00 	cmp    r8,0x400
  66d030:	b8 00 04 00 00       	mov    eax,0x400
  66d035:	4c 89 84 24 40 01 00 	mov    QWORD PTR [rsp+0x140],r8
  66d03c:	00 
  66d03d:	49 0f 42 c0          	cmovb  rax,r8
  66d041:	c1 e8 04             	shr    eax,0x4
  66d044:	49 81 f9 00 04 00 00 	cmp    r9,0x400
  66d04b:	41 bf 00 04 00 00    	mov    r15d,0x400
  66d051:	4c 89 4c 24 20       	mov    QWORD PTR [rsp+0x20],r9
  66d056:	4d 0f 42 f9          	cmovb  r15,r9
  66d05a:	41 c1 ef 04          	shr    r15d,0x4
  66d05e:	4c 39 f8             	cmp    rax,r15
  66d061:	4c 0f 42 f8          	cmovb  r15,rax
  66d065:	48 89 f7             	mov    rdi,rsi
  66d068:	48 c1 e7 0a          	shl    rdi,0xa
  66d06c:	48 8b 5c 24 28       	mov    rbx,QWORD PTR [rsp+0x28]
  66d071:	48 29 fb             	sub    rbx,rdi
  66d074:	48 81 fb 00 04 00 00 	cmp    rbx,0x400
  66d07b:	b8 00 04 00 00       	mov    eax,0x400
  66d080:	48 0f 43 d8          	cmovae rbx,rax
  66d084:	4c 8b 64 24 38       	mov    r12,QWORD PTR [rsp+0x38]
  66d089:	49 29 fc             	sub    r12,rdi
  66d08c:	49 81 fc 00 04 00 00 	cmp    r12,0x400
  66d093:	4c 0f 43 e0          	cmovae r12,rax
  66d097:	48 89 b4 24 a0 00 00 	mov    QWORD PTR [rsp+0xa0],rsi
  66d09e:	00 
  66d09f:	48 c1 e6 06          	shl    rsi,0x6
  66d0a3:	48 8b 8c 24 c0 00 00 	mov    rcx,QWORD PTR [rsp+0xc0]
  66d0aa:	00 
  66d0ab:	48 89 74 24 18       	mov    QWORD PTR [rsp+0x18],rsi
  66d0b0:	48 29 f1             	sub    rcx,rsi
  66d0b3:	48 83 f9 40          	cmp    rcx,0x40
  66d0b7:	b8 40 00 00 00       	mov    eax,0x40
  66d0bc:	48 0f 43 c8          	cmovae rcx,rax
  66d0c0:	48 03 7c 24 70       	add    rdi,QWORD PTR [rsp+0x70]
  66d0c5:	48 89 ce             	mov    rsi,rcx
  66d0c8:	48 8d 94 24 c0 16 00 	lea    rdx,[rsp+0x16c0]
  66d0cf:	00 
  66d0d0:	48 89 4c 24 58       	mov    QWORD PTR [rsp+0x58],rcx
  66d0d5:	c5 f8 77             	vzeroupper 
  66d0d8:	e8 63 62 00 00       	call   673340 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate5fieldNtB2_10SmallField15decompose_block>
  66d0dd:	c1 eb 04             	shr    ebx,0x4
  66d0e0:	41 c1 ec 04          	shr    r12d,0x4
  66d0e4:	49 39 dc             	cmp    r12,rbx
  66d0e7:	49 0f 42 dc          	cmovb  rbx,r12
  66d0eb:	48 85 db             	test   rbx,rbx
  66d0ee:	48 8b 5c 24 30       	mov    rbx,QWORD PTR [rsp+0x30]
  66d0f3:	62 71 fd 48 6f 0d c3 	vmovdqa64 zmm9,ZMMWORD PTR [rip+0xffffffffff9a53c3]        # 124c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1372>
  66d0fa:	53 9a ff 
  66d0fd:	62 71 fd 48 6f 15 f9 	vmovdqa64 zmm10,ZMMWORD PTR [rip+0xffffffffff9a53f9]        # 12500 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x13b2>
  66d104:	53 9a ff 
  66d107:	0f 84 94 04 00 00    	je     66d5a1 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x3091>
  66d10d:	48 8b 05 24 db 09 00 	mov    rax,QWORD PTR [rip+0x9db24]        # 70ac38 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1008>
  66d114:	48 03 05 5d db 09 00 	add    rax,QWORD PTR [rip+0x9db5d]        # 70ac78 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1048>
  66d11b:	b9 08 00 00 00       	mov    ecx,0x8
  66d120:	c4 c1 78 10 44 0e f8 	vmovups xmm0,XMMWORD PTR [r14+rcx*1-0x8]
  66d127:	48 8b 54 0b f8       	mov    rdx,QWORD PTR [rbx+rcx*1-0x8]
  66d12c:	49 8b 74 0e f8       	mov    rsi,QWORD PTR [r14+rcx*1-0x8]
  66d131:	c5 f8 54 44 0b f8    	vandps xmm0,xmm0,XMMWORD PTR [rbx+rcx*1-0x8]
  66d137:	c5 f8 11 84 0c b8 12 	vmovups XMMWORD PTR [rsp+rcx*1+0x12b8],xmm0
  66d13e:	00 00 
  66d140:	40 0f b6 fe          	movzx  edi,sil
  66d144:	c1 e7 07             	shl    edi,0x7
  66d147:	44 0f b6 c2          	movzx  r8d,dl
  66d14b:	41 c1 e0 07          	shl    r8d,0x7
  66d14f:	41 89 f1             	mov    r9d,esi
  66d152:	41 d1 e9             	shr    r9d,1
  66d155:	41 81 e1 80 7f 00 00 	and    r9d,0x7f80
  66d15c:	41 89 d2             	mov    r10d,edx
  66d15f:	41 d1 ea             	shr    r10d,1
  66d162:	41 81 e2 80 7f 00 00 	and    r10d,0x7f80
  66d169:	62 b1 7d 48 70 04 08 	vpshufd zmm0,ZMMWORD PTR [rax+r9*1],0x4e
  66d170:	4e 
  66d171:	62 f1 7d 48 ef 04 38 	vpxord zmm0,zmm0,ZMMWORD PTR [rax+rdi*1]
  66d178:	62 b1 7d 48 70 0c 10 	vpshufd zmm1,ZMMWORD PTR [rax+r10*1],0x4e
  66d17f:	4e 
  66d180:	62 b1 75 48 ef 0c 00 	vpxord zmm1,zmm1,ZMMWORD PTR [rax+r8*1]
  66d187:	62 b1 7d 48 70 54 08 	vpshufd zmm2,ZMMWORD PTR [rax+r9*1+0x40],0x4e
  66d18e:	01 4e 
  66d190:	62 f1 6d 48 ef 54 38 	vpxord zmm2,zmm2,ZMMWORD PTR [rax+rdi*1+0x40]
  66d197:	01 
  66d198:	62 b1 7d 48 70 5c 10 	vpshufd zmm3,ZMMWORD PTR [rax+r10*1+0x40],0x4e
  66d19f:	01 4e 
  66d1a1:	62 b1 65 48 ef 5c 00 	vpxord zmm3,zmm3,ZMMWORD PTR [rax+r8*1+0x40]
  66d1a8:	01 
  66d1a9:	89 f7                	mov    edi,esi
  66d1ab:	c1 ef 09             	shr    edi,0x9
  66d1ae:	81 e7 80 7f 00 00    	and    edi,0x7f80
  66d1b4:	41 89 d0             	mov    r8d,edx
  66d1b7:	41 c1 e8 09          	shr    r8d,0x9
  66d1bb:	41 81 e0 80 7f 00 00 	and    r8d,0x7f80
  66d1c2:	62 f3 fd 48 00 24 38 	vpermq zmm4,ZMMWORD PTR [rax+rdi*1],0x4e
  66d1c9:	4e 
  66d1ca:	62 b3 fd 48 00 2c 00 	vpermq zmm5,ZMMWORD PTR [rax+r8*1],0x4e
  66d1d1:	4e 
  66d1d2:	62 f3 fd 48 00 74 38 	vpermq zmm6,ZMMWORD PTR [rax+rdi*1+0x40],0x4e
  66d1d9:	01 4e 
  66d1db:	62 b3 fd 48 00 7c 00 	vpermq zmm7,ZMMWORD PTR [rax+r8*1+0x40],0x4e
  66d1e2:	01 4e 
  66d1e4:	89 f7                	mov    edi,esi
  66d1e6:	c1 ef 11             	shr    edi,0x11
  66d1e9:	83 e7 80             	and    edi,0xffffff80
  66d1ec:	41 89 d0             	mov    r8d,edx
  66d1ef:	41 c1 e8 11          	shr    r8d,0x11
  66d1f3:	41 83 e0 80          	and    r8d,0xffffff80
  66d1f7:	62 73 fd 48 00 04 38 	vpermq zmm8,ZMMWORD PTR [rax+rdi*1],0x1b
  66d1fe:	1b 
  66d1ff:	62 73 7d 48 25 c4 96 	vpternlogd zmm8,zmm0,zmm4,0x96
  66d206:	62 b3 fd 48 00 04 00 	vpermq zmm0,ZMMWORD PTR [rax+r8*1],0x1b
  66d20d:	1b 
  66d20e:	62 f3 75 48 25 c5 96 	vpternlogd zmm0,zmm1,zmm5,0x96
  66d215:	62 f3 fd 48 00 4c 38 	vpermq zmm1,ZMMWORD PTR [rax+rdi*1+0x40],0x1b
  66d21c:	01 1b 
  66d21e:	62 f3 6d 48 25 ce 96 	vpternlogd zmm1,zmm2,zmm6,0x96
  66d225:	62 b3 fd 48 00 54 00 	vpermq zmm2,ZMMWORD PTR [rax+r8*1+0x40],0x1b
  66d22c:	01 1b 
  66d22e:	62 f3 65 48 25 d7 96 	vpternlogd zmm2,zmm3,zmm7,0x96
  66d235:	48 c1 ee 19          	shr    rsi,0x19
  66d239:	81 e6 80 7f 00 00    	and    esi,0x7f80
  66d23f:	48 c1 ea 19          	shr    rdx,0x19
  66d243:	81 e2 80 7f 00 00    	and    edx,0x7f80
  66d249:	62 f1 fe 48 6f 1c 30 	vmovdqu64 zmm3,ZMMWORD PTR [rax+rsi*1]
  66d250:	62 f1 fe 48 6f 64 30 	vmovdqu64 zmm4,ZMMWORD PTR [rax+rsi*1+0x40]
  66d257:	01 
  66d258:	62 f3 e5 48 43 db 4e 	vshufi64x2 zmm3,zmm3,zmm3,0x4e
  66d25f:	62 f1 fe 48 6f 2c 10 	vmovdqu64 zmm5,ZMMWORD PTR [rax+rdx*1]
  66d266:	62 f1 fe 48 6f 74 10 	vmovdqu64 zmm6,ZMMWORD PTR [rax+rdx*1+0x40]
  66d26d:	01 
  66d26e:	62 f3 d5 48 43 ed 4e 	vshufi64x2 zmm5,zmm5,zmm5,0x4e
  66d275:	62 f3 dd 48 43 e4 4e 	vshufi64x2 zmm4,zmm4,zmm4,0x4e
  66d27c:	62 f3 cd 48 43 f6 4e 	vshufi64x2 zmm6,zmm6,zmm6,0x4e
  66d283:	41 0f b6 54 0e fd    	movzx  edx,BYTE PTR [r14+rcx*1-0x3]
  66d289:	c1 e2 07             	shl    edx,0x7
  66d28c:	0f b6 74 0b fd       	movzx  esi,BYTE PTR [rbx+rcx*1-0x3]
  66d291:	c1 e6 07             	shl    esi,0x7
  66d294:	62 f2 b5 48 36 3c 10 	vpermq zmm7,zmm9,ZMMWORD PTR [rax+rdx*1]
  66d29b:	62 f3 3d 48 25 fb 96 	vpternlogd zmm7,zmm8,zmm3,0x96
  66d2a2:	62 f2 b5 48 36 1c 30 	vpermq zmm3,zmm9,ZMMWORD PTR [rax+rsi*1]
  66d2a9:	62 f3 7d 48 25 dd 96 	vpternlogd zmm3,zmm0,zmm5,0x96
  66d2b0:	62 f2 b5 48 36 44 10 	vpermq zmm0,zmm9,ZMMWORD PTR [rax+rdx*1+0x40]
  66d2b7:	01 
  66d2b8:	62 f3 75 48 25 c4 96 	vpternlogd zmm0,zmm1,zmm4,0x96
  66d2bf:	62 f2 b5 48 36 4c 30 	vpermq zmm1,zmm9,ZMMWORD PTR [rax+rsi*1+0x40]
  66d2c6:	01 
  66d2c7:	62 f3 6d 48 25 ce 96 	vpternlogd zmm1,zmm2,zmm6,0x96
  66d2ce:	41 0f b6 54 0e fe    	movzx  edx,BYTE PTR [r14+rcx*1-0x2]
  66d2d4:	c1 e2 07             	shl    edx,0x7
  66d2d7:	0f b6 74 0b fe       	movzx  esi,BYTE PTR [rbx+rcx*1-0x2]
  66d2dc:	c1 e6 07             	shl    esi,0x7
  66d2df:	62 f1 fe 48 6f 14 10 	vmovdqu64 zmm2,ZMMWORD PTR [rax+rdx*1]
  66d2e6:	62 f1 fe 48 6f 64 10 	vmovdqu64 zmm4,ZMMWORD PTR [rax+rdx*1+0x40]
  66d2ed:	01 
  66d2ee:	62 f1 fe 48 6f 2c 30 	vmovdqu64 zmm5,ZMMWORD PTR [rax+rsi*1]
  66d2f5:	62 f1 fe 48 6f 74 30 	vmovdqu64 zmm6,ZMMWORD PTR [rax+rsi*1+0x40]
  66d2fc:	01 
  66d2fd:	62 f3 ed 48 43 d2 1b 	vshufi64x2 zmm2,zmm2,zmm2,0x1b
  66d304:	62 f3 d5 48 43 ed 1b 	vshufi64x2 zmm5,zmm5,zmm5,0x1b
  66d30b:	62 f3 dd 48 43 e4 1b 	vshufi64x2 zmm4,zmm4,zmm4,0x1b
  66d312:	41 0f b6 54 0e ff    	movzx  edx,BYTE PTR [r14+rcx*1-0x1]
  66d318:	c1 e2 07             	shl    edx,0x7
  66d31b:	0f b6 74 0b ff       	movzx  esi,BYTE PTR [rbx+rcx*1-0x1]
  66d320:	c1 e6 07             	shl    esi,0x7
  66d323:	62 72 ad 48 36 04 10 	vpermq zmm8,zmm10,ZMMWORD PTR [rax+rdx*1]
  66d32a:	62 73 45 48 25 c2 96 	vpternlogd zmm8,zmm7,zmm2,0x96
  66d331:	62 f2 ad 48 36 14 30 	vpermq zmm2,zmm10,ZMMWORD PTR [rax+rsi*1]
  66d338:	62 f3 cd 48 43 f6 1b 	vshufi64x2 zmm6,zmm6,zmm6,0x1b
  66d33f:	62 f3 65 48 25 d5 96 	vpternlogd zmm2,zmm3,zmm5,0x96
  66d346:	62 f2 ad 48 36 5c 10 	vpermq zmm3,zmm10,ZMMWORD PTR [rax+rdx*1+0x40]
  66d34d:	01 
  66d34e:	62 f2 ad 48 36 6c 30 	vpermq zmm5,zmm10,ZMMWORD PTR [rax+rsi*1+0x40]
  66d355:	01 
  66d356:	62 f3 7d 48 25 dc 96 	vpternlogd zmm3,zmm0,zmm4,0x96
  66d35d:	62 f3 75 48 25 ee 96 	vpternlogd zmm5,zmm1,zmm6,0x96
  66d364:	41 0f b6 14 0e       	movzx  edx,BYTE PTR [r14+rcx*1]
  66d369:	c1 e2 07             	shl    edx,0x7
  66d36c:	0f b6 34 0b          	movzx  esi,BYTE PTR [rbx+rcx*1]
  66d370:	c1 e6 07             	shl    esi,0x7
  66d373:	41 0f b6 7c 0e 01    	movzx  edi,BYTE PTR [r14+rcx*1+0x1]
  66d379:	c1 e7 07             	shl    edi,0x7
  66d37c:	44 0f b6 44 0b 01    	movzx  r8d,BYTE PTR [rbx+rcx*1+0x1]
  66d382:	41 c1 e0 07          	shl    r8d,0x7
  66d386:	62 f1 7d 48 70 44 38 	vpshufd zmm0,ZMMWORD PTR [rax+rdi*1+0x40],0x4e
  66d38d:	01 4e 
  66d38f:	62 f3 3d 48 25 44 10 	vpternlogd zmm0,zmm8,ZMMWORD PTR [rax+rdx*1+0x40],0x96
  66d396:	01 96 
  66d398:	62 b1 7d 48 70 4c 00 	vpshufd zmm1,ZMMWORD PTR [rax+r8*1+0x40],0x4e
  66d39f:	01 4e 
  66d3a1:	62 f3 6d 48 25 4c 30 	vpternlogd zmm1,zmm2,ZMMWORD PTR [rax+rsi*1+0x40],0x96
  66d3a8:	01 96 
  66d3aa:	62 f1 7d 48 70 14 38 	vpshufd zmm2,ZMMWORD PTR [rax+rdi*1],0x4e
  66d3b1:	4e 
  66d3b2:	62 f3 65 48 25 14 10 	vpternlogd zmm2,zmm3,ZMMWORD PTR [rax+rdx*1],0x96
  66d3b9:	96 
  66d3ba:	62 b1 7d 48 70 1c 00 	vpshufd zmm3,ZMMWORD PTR [rax+r8*1],0x4e
  66d3c1:	4e 
  66d3c2:	62 f3 55 48 25 1c 30 	vpternlogd zmm3,zmm5,ZMMWORD PTR [rax+rsi*1],0x96
  66d3c9:	96 
  66d3ca:	41 0f b6 54 0e 02    	movzx  edx,BYTE PTR [r14+rcx*1+0x2]
  66d3d0:	c1 e2 07             	shl    edx,0x7
  66d3d3:	0f b6 74 0b 02       	movzx  esi,BYTE PTR [rbx+rcx*1+0x2]
  66d3d8:	c1 e6 07             	shl    esi,0x7
  66d3db:	62 f3 fd 48 00 64 10 	vpermq zmm4,ZMMWORD PTR [rax+rdx*1+0x40],0x4e
  66d3e2:	01 4e 
  66d3e4:	62 f3 fd 48 00 6c 30 	vpermq zmm5,ZMMWORD PTR [rax+rsi*1+0x40],0x4e
  66d3eb:	01 4e 
  66d3ed:	62 f3 fd 48 00 34 10 	vpermq zmm6,ZMMWORD PTR [rax+rdx*1],0x4e
  66d3f4:	4e 
  66d3f5:	62 f3 fd 48 00 3c 30 	vpermq zmm7,ZMMWORD PTR [rax+rsi*1],0x4e
  66d3fc:	4e 
  66d3fd:	41 0f b6 54 0e 03    	movzx  edx,BYTE PTR [r14+rcx*1+0x3]
  66d403:	c1 e2 07             	shl    edx,0x7
  66d406:	0f b6 74 0b 03       	movzx  esi,BYTE PTR [rbx+rcx*1+0x3]
  66d40b:	c1 e6 07             	shl    esi,0x7
  66d40e:	62 73 fd 48 00 44 10 	vpermq zmm8,ZMMWORD PTR [rax+rdx*1+0x40],0x1b
  66d415:	01 1b 
  66d417:	62 73 7d 48 25 c4 96 	vpternlogd zmm8,zmm0,zmm4,0x96
  66d41e:	62 f3 fd 48 00 44 30 	vpermq zmm0,ZMMWORD PTR [rax+rsi*1+0x40],0x1b
  66d425:	01 1b 
  66d427:	62 f3 75 48 25 c5 96 	vpternlogd zmm0,zmm1,zmm5,0x96
  66d42e:	62 f3 fd 48 00 0c 10 	vpermq zmm1,ZMMWORD PTR [rax+rdx*1],0x1b
  66d435:	1b 
  66d436:	62 f3 6d 48 25 ce 96 	vpternlogd zmm1,zmm2,zmm6,0x96
  66d43d:	62 f3 fd 48 00 14 30 	vpermq zmm2,ZMMWORD PTR [rax+rsi*1],0x1b
  66d444:	1b 
  66d445:	62 f3 65 48 25 d7 96 	vpternlogd zmm2,zmm3,zmm7,0x96
  66d44c:	41 0f b6 54 0e 04    	movzx  edx,BYTE PTR [r14+rcx*1+0x4]
  66d452:	c1 e2 07             	shl    edx,0x7
  66d455:	0f b6 74 0b 04       	movzx  esi,BYTE PTR [rbx+rcx*1+0x4]
  66d45a:	c1 e6 07             	shl    esi,0x7
  66d45d:	62 f1 fe 48 6f 1c 10 	vmovdqu64 zmm3,ZMMWORD PTR [rax+rdx*1]
  66d464:	62 f1 fe 48 6f 64 10 	vmovdqu64 zmm4,ZMMWORD PTR [rax+rdx*1+0x40]
  66d46b:	01 
  66d46c:	62 f1 fe 48 6f 2c 30 	vmovdqu64 zmm5,ZMMWORD PTR [rax+rsi*1]
  66d473:	62 f1 fe 48 6f 74 30 	vmovdqu64 zmm6,ZMMWORD PTR [rax+rsi*1+0x40]
  66d47a:	01 
  66d47b:	62 f3 dd 48 43 e4 4e 	vshufi64x2 zmm4,zmm4,zmm4,0x4e
  66d482:	62 f3 cd 48 43 f6 4e 	vshufi64x2 zmm6,zmm6,zmm6,0x4e
  66d489:	62 f3 e5 48 43 db 4e 	vshufi64x2 zmm3,zmm3,zmm3,0x4e
  66d490:	41 0f b6 54 0e 05    	movzx  edx,BYTE PTR [r14+rcx*1+0x5]
  66d496:	c1 e2 07             	shl    edx,0x7
  66d499:	0f b6 74 0b 05       	movzx  esi,BYTE PTR [rbx+rcx*1+0x5]
  66d49e:	c1 e6 07             	shl    esi,0x7
  66d4a1:	62 f2 b5 48 36 7c 10 	vpermq zmm7,zmm9,ZMMWORD PTR [rax+rdx*1+0x40]
  66d4a8:	01 
  66d4a9:	62 f3 3d 48 25 fc 96 	vpternlogd zmm7,zmm8,zmm4,0x96
  66d4b0:	62 f2 b5 48 36 64 30 	vpermq zmm4,zmm9,ZMMWORD PTR [rax+rsi*1+0x40]
  66d4b7:	01 
  66d4b8:	62 f3 d5 48 43 ed 4e 	vshufi64x2 zmm5,zmm5,zmm5,0x4e
  66d4bf:	62 f3 7d 48 25 e6 96 	vpternlogd zmm4,zmm0,zmm6,0x96
  66d4c6:	62 f2 b5 48 36 04 10 	vpermq zmm0,zmm9,ZMMWORD PTR [rax+rdx*1]
  66d4cd:	62 f2 b5 48 36 34 30 	vpermq zmm6,zmm9,ZMMWORD PTR [rax+rsi*1]
  66d4d4:	62 f3 75 48 25 c3 96 	vpternlogd zmm0,zmm1,zmm3,0x96
  66d4db:	62 f3 6d 48 25 f5 96 	vpternlogd zmm6,zmm2,zmm5,0x96
  66d4e2:	41 0f b6 54 0e 06    	movzx  edx,BYTE PTR [r14+rcx*1+0x6]
  66d4e8:	c1 e2 07             	shl    edx,0x7
  66d4eb:	0f b6 74 0b 06       	movzx  esi,BYTE PTR [rbx+rcx*1+0x6]
  66d4f0:	c1 e6 07             	shl    esi,0x7
  66d4f3:	62 f1 fe 48 6f 0c 10 	vmovdqu64 zmm1,ZMMWORD PTR [rax+rdx*1]
  66d4fa:	62 f1 fe 48 6f 54 10 	vmovdqu64 zmm2,ZMMWORD PTR [rax+rdx*1+0x40]
  66d501:	01 
  66d502:	62 f1 fe 48 6f 1c 30 	vmovdqu64 zmm3,ZMMWORD PTR [rax+rsi*1]
  66d509:	62 f3 ed 48 43 d2 1b 	vshufi64x2 zmm2,zmm2,zmm2,0x1b
  66d510:	62 f1 fe 48 6f 6c 30 	vmovdqu64 zmm5,ZMMWORD PTR [rax+rsi*1+0x40]
  66d517:	01 
  66d518:	62 f3 d5 48 43 ed 1b 	vshufi64x2 zmm5,zmm5,zmm5,0x1b
  66d51f:	62 f3 f5 48 43 c9 1b 	vshufi64x2 zmm1,zmm1,zmm1,0x1b
  66d526:	62 f3 e5 48 43 db 1b 	vshufi64x2 zmm3,zmm3,zmm3,0x1b
  66d52d:	41 0f b6 54 0e 07    	movzx  edx,BYTE PTR [r14+rcx*1+0x7]
  66d533:	c1 e2 07             	shl    edx,0x7
  66d536:	0f b6 74 0b 07       	movzx  esi,BYTE PTR [rbx+rcx*1+0x7]
  66d53b:	c1 e6 07             	shl    esi,0x7
  66d53e:	62 72 ad 48 36 44 10 	vpermq zmm8,zmm10,ZMMWORD PTR [rax+rdx*1+0x40]
  66d545:	01 
  66d546:	62 73 45 48 25 c2 96 	vpternlogd zmm8,zmm7,zmm2,0x96
  66d54d:	62 f2 ad 48 36 54 30 	vpermq zmm2,zmm10,ZMMWORD PTR [rax+rsi*1+0x40]
  66d554:	01 
  66d555:	62 f3 5d 48 25 d5 96 	vpternlogd zmm2,zmm4,zmm5,0x96
  66d55c:	62 f2 ad 48 36 24 10 	vpermq zmm4,zmm10,ZMMWORD PTR [rax+rdx*1]
  66d563:	62 f2 3d 48 cf d2    	vgf2p8mulb zmm2,zmm8,zmm2
  66d569:	62 f3 7d 48 25 e1 96 	vpternlogd zmm4,zmm0,zmm1,0x96
  66d570:	62 f2 ad 48 36 04 30 	vpermq zmm0,zmm10,ZMMWORD PTR [rax+rsi*1]
  66d577:	62 f3 4d 48 25 c3 96 	vpternlogd zmm0,zmm6,zmm3,0x96
  66d57e:	62 f2 5d 48 cf c0    	vgf2p8mulb zmm0,zmm4,zmm0
  66d584:	62 f1 fd 48 7f 54 cc 	vmovdqa64 ZMMWORD PTR [rsp+rcx*8+0x1a80],zmm2
  66d58b:	6a 
  66d58c:	62 f1 fd 48 7f 44 cc 	vmovdqa64 ZMMWORD PTR [rsp+rcx*8+0x1ac0],zmm0
  66d593:	6b 
  66d594:	48 83 c1 10          	add    rcx,0x10
  66d598:	49 ff cf             	dec    r15
  66d59b:	0f 85 7f fb ff ff    	jne    66d120 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x2c10>
  66d5a1:	41 c1 e5 07          	shl    r13d,0x7
  66d5a5:	48 8d bc 24 c0 02 00 	lea    rdi,[rsp+0x2c0]
  66d5ac:	00 
  66d5ad:	48 8d b4 24 c0 16 00 	lea    rsi,[rsp+0x16c0]
  66d5b4:	00 
  66d5b5:	48 8b 54 24 58       	mov    rdx,QWORD PTR [rsp+0x58]
  66d5ba:	48 8d 8c 24 c0 12 00 	lea    rcx,[rsp+0x12c0]
  66d5c1:	00 
  66d5c2:	49 89 d0             	mov    r8,rdx
  66d5c5:	c5 f8 77             	vzeroupper 
  66d5c8:	e8 d3 26 01 00       	call   67fca0 <_RNvNtNtNtCs5Palesvl9wF_6mikros10univariate5tiled8affine_h16consume_h_affine>
  66d5cd:	62 71 fe 48 6f 54 24 	vmovdqu64 zmm10,ZMMWORD PTR [rsp+0x340]
  66d5d4:	0d 
  66d5d5:	62 71 fe 48 6f 4c 24 	vmovdqu64 zmm9,ZMMWORD PTR [rsp+0x380]
  66d5dc:	0e 
  66d5dd:	62 71 fe 48 6f 44 24 	vmovdqu64 zmm8,ZMMWORD PTR [rsp+0x440]
  66d5e4:	11 
  66d5e5:	62 f1 fe 48 6f 7c 24 	vmovdqu64 zmm7,ZMMWORD PTR [rsp+0x480]
  66d5ec:	12 
  66d5ed:	62 f1 fe 48 6f 74 24 	vmovdqu64 zmm6,ZMMWORD PTR [rsp+0x540]
  66d5f4:	15 
  66d5f5:	62 f1 fe 48 6f 64 24 	vmovdqu64 zmm4,ZMMWORD PTR [rsp+0x640]
  66d5fc:	19 
  66d5fd:	62 f1 fe 48 6f 5c 24 	vmovdqu64 zmm3,ZMMWORD PTR [rsp+0x740]
  66d604:	1d 
  66d605:	62 f1 fe 48 6f 54 24 	vmovdqu64 zmm2,ZMMWORD PTR [rsp+0x840]
  66d60c:	21 
  66d60d:	62 f1 fe 48 6f 4c 24 	vmovdqu64 zmm1,ZMMWORD PTR [rsp+0x940]
  66d614:	25 
  66d615:	62 f1 fe 48 6f 44 24 	vmovdqu64 zmm0,ZMMWORD PTR [rsp+0xa40]
  66d61c:	29 
  66d61d:	62 71 fe 48 6f 7c 24 	vmovdqu64 zmm15,ZMMWORD PTR [rsp+0x580]
  66d624:	16 
  66d625:	62 71 fe 48 6f 74 24 	vmovdqu64 zmm14,ZMMWORD PTR [rsp+0x680]
  66d62c:	1a 
  66d62d:	62 71 fe 48 6f 6c 24 	vmovdqu64 zmm13,ZMMWORD PTR [rsp+0x780]
  66d634:	1e 
  66d635:	62 71 fe 48 6f 64 24 	vmovdqu64 zmm12,ZMMWORD PTR [rsp+0x880]
  66d63c:	22 
  66d63d:	62 71 fe 48 6f 5c 24 	vmovdqu64 zmm11,ZMMWORD PTR [rsp+0x980]
  66d644:	26 
  66d645:	62 f1 fe 48 6f 6c 24 	vmovdqu64 zmm5,ZMMWORD PTR [rsp+0xa80]
  66d64c:	2a 
  66d64d:	4c 8b 54 24 18       	mov    r10,QWORD PTR [rsp+0x18]
  66d652:	4c 39 94 24 c0 00 00 	cmp    QWORD PTR [rsp+0xc0],r10
  66d659:	00 
  66d65a:	75 24                	jne    66d680 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x3170>
  66d65c:	48 8b b4 24 a0 00 00 	mov    rsi,QWORD PTR [rsp+0xa0]
  66d663:	00 
  66d664:	48 8b 7c 24 40       	mov    rdi,QWORD PTR [rsp+0x40]
  66d669:	4c 8b 84 24 40 01 00 	mov    r8,QWORD PTR [rsp+0x140]
  66d670:	00 
  66d671:	4c 8b 4c 24 20       	mov    r9,QWORD PTR [rsp+0x20]
  66d676:	e9 4d 01 00 00       	jmp    66d7c8 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x32b8>
  66d67b:	0f 1f 44 00 00       	nop    DWORD PTR [rax+rax*1+0x0]
  66d680:	31 c0                	xor    eax,eax
  66d682:	48 8d 8c 24 c7 16 00 	lea    rcx,[rsp+0x16c7]
  66d689:	00 
  66d68a:	48 8b b4 24 a0 00 00 	mov    rsi,QWORD PTR [rsp+0xa0]
  66d691:	00 
  66d692:	48 8b 7c 24 40       	mov    rdi,QWORD PTR [rsp+0x40]
  66d697:	4c 8b 84 24 40 01 00 	mov    r8,QWORD PTR [rsp+0x140]
  66d69e:	00 
  66d69f:	4c 8b 4c 24 20       	mov    r9,QWORD PTR [rsp+0x20]
  66d6a4:	66 66 66 2e 0f 1f 84 	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  66d6ab:	00 00 00 00 00 
  66d6b0:	62 e2 7d 48 78 41 f9 	vpbroadcastb zmm16,BYTE PTR [rcx-0x7]
  66d6b7:	62 e1 fd 48 6f 4c 04 	vmovdqa64 zmm17,ZMMWORD PTR [rsp+rax*1+0x1ac0]
  66d6be:	6b 
  66d6bf:	62 e1 fd 48 6f 54 04 	vmovdqa64 zmm18,ZMMWORD PTR [rsp+rax*1+0x1b00]
  66d6c6:	6c 
  66d6c7:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66d6cd:	62 31 2d 48 ef d3    	vpxord zmm10,zmm10,zmm19
  66d6d3:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66d6d9:	62 31 35 48 ef c8    	vpxord zmm9,zmm9,zmm16
  66d6df:	62 e2 7d 48 78 41 fa 	vpbroadcastb zmm16,BYTE PTR [rcx-0x6]
  66d6e6:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66d6ec:	62 31 3d 48 ef c3    	vpxord zmm8,zmm8,zmm19
  66d6f2:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66d6f8:	62 b1 45 48 ef f8    	vpxord zmm7,zmm7,zmm16
  66d6fe:	62 e2 7d 48 78 41 fb 	vpbroadcastb zmm16,BYTE PTR [rcx-0x5]
  66d705:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66d70b:	62 b1 4d 48 ef f3    	vpxord zmm6,zmm6,zmm19
  66d711:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66d717:	62 31 05 48 ef f8    	vpxord zmm15,zmm15,zmm16
  66d71d:	62 e2 7d 48 78 41 fc 	vpbroadcastb zmm16,BYTE PTR [rcx-0x4]
  66d724:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66d72a:	62 b1 5d 48 ef e3    	vpxord zmm4,zmm4,zmm19
  66d730:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66d736:	62 31 0d 48 ef f0    	vpxord zmm14,zmm14,zmm16
  66d73c:	62 e2 7d 48 78 41 fd 	vpbroadcastb zmm16,BYTE PTR [rcx-0x3]
  66d743:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66d749:	62 b1 65 48 ef db    	vpxord zmm3,zmm3,zmm19
  66d74f:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66d755:	62 31 15 48 ef e8    	vpxord zmm13,zmm13,zmm16
  66d75b:	62 e2 7d 48 78 41 fe 	vpbroadcastb zmm16,BYTE PTR [rcx-0x2]
  66d762:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66d768:	62 b1 6d 48 ef d3    	vpxord zmm2,zmm2,zmm19
  66d76e:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66d774:	62 31 1d 48 ef e0    	vpxord zmm12,zmm12,zmm16
  66d77a:	62 e2 7d 48 78 41 ff 	vpbroadcastb zmm16,BYTE PTR [rcx-0x1]
  66d781:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66d787:	62 b1 75 48 ef cb    	vpxord zmm1,zmm1,zmm19
  66d78d:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66d793:	62 31 25 48 ef d8    	vpxord zmm11,zmm11,zmm16
  66d799:	62 e2 7d 48 78 01    	vpbroadcastb zmm16,BYTE PTR [rcx]
  66d79f:	62 a2 75 40 cf c8    	vgf2p8mulb zmm17,zmm17,zmm16
  66d7a5:	62 b1 7d 48 ef c1    	vpxord zmm0,zmm0,zmm17
  66d7ab:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66d7b1:	62 b1 55 48 ef e8    	vpxord zmm5,zmm5,zmm16
  66d7b7:	48 83 c1 10          	add    rcx,0x10
  66d7bb:	48 83 e8 80          	sub    rax,0xffffffffffffff80
  66d7bf:	49 39 c5             	cmp    r13,rax
  66d7c2:	0f 85 e8 fe ff ff    	jne    66d6b0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x31a0>
  66d7c8:	62 71 fe 48 7f 54 24 	vmovdqu64 ZMMWORD PTR [rsp+0x340],zmm10
  66d7cf:	0d 
  66d7d0:	62 71 fe 48 7f 4c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x380],zmm9
  66d7d7:	0e 
  66d7d8:	62 71 fe 48 7f 44 24 	vmovdqu64 ZMMWORD PTR [rsp+0x440],zmm8
  66d7df:	11 
  66d7e0:	62 f1 fe 48 7f 7c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x480],zmm7
  66d7e7:	12 
  66d7e8:	62 f1 fe 48 7f 74 24 	vmovdqu64 ZMMWORD PTR [rsp+0x540],zmm6
  66d7ef:	15 
  66d7f0:	62 71 fe 48 7f 7c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x580],zmm15
  66d7f7:	16 
  66d7f8:	62 f1 fe 48 7f 64 24 	vmovdqu64 ZMMWORD PTR [rsp+0x640],zmm4
  66d7ff:	19 
  66d800:	62 71 fe 48 7f 74 24 	vmovdqu64 ZMMWORD PTR [rsp+0x680],zmm14
  66d807:	1a 
  66d808:	62 f1 fe 48 7f 5c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x740],zmm3
  66d80f:	1d 
  66d810:	62 71 fe 48 7f 6c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x780],zmm13
  66d817:	1e 
  66d818:	62 f1 fe 48 7f 54 24 	vmovdqu64 ZMMWORD PTR [rsp+0x840],zmm2
  66d81f:	21 
  66d820:	62 71 fe 48 7f 64 24 	vmovdqu64 ZMMWORD PTR [rsp+0x880],zmm12
  66d827:	22 
  66d828:	62 f1 fe 48 7f 4c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x940],zmm1
  66d82f:	25 
  66d830:	62 71 fe 48 7f 5c 24 	vmovdqu64 ZMMWORD PTR [rsp+0x980],zmm11
  66d837:	26 
  66d838:	62 f1 fe 48 7f 44 24 	vmovdqu64 ZMMWORD PTR [rsp+0xa40],zmm0
  66d83f:	29 
  66d840:	62 f1 fe 48 7f 6c 24 	vmovdqu64 ZMMWORD PTR [rsp+0xa80],zmm5
  66d847:	2a 
  66d848:	62 71 fe 48 6f 54 24 	vmovdqu64 zmm10,ZMMWORD PTR [rsp+0xb40]
  66d84f:	2d 
  66d850:	62 71 fe 48 6f 4c 24 	vmovdqu64 zmm9,ZMMWORD PTR [rsp+0xb80]
  66d857:	2e 
  66d858:	62 71 fe 48 6f 44 24 	vmovdqu64 zmm8,ZMMWORD PTR [rsp+0xc40]
  66d85f:	31 
  66d860:	62 f1 fe 48 6f 7c 24 	vmovdqu64 zmm7,ZMMWORD PTR [rsp+0xc80]
  66d867:	32 
  66d868:	62 f1 fe 48 6f 74 24 	vmovdqu64 zmm6,ZMMWORD PTR [rsp+0xd40]
  66d86f:	35 
  66d870:	62 f1 fe 48 6f 64 24 	vmovdqu64 zmm4,ZMMWORD PTR [rsp+0xe40]
  66d877:	39 
  66d878:	62 f1 fe 48 6f 5c 24 	vmovdqu64 zmm3,ZMMWORD PTR [rsp+0xf40]
  66d87f:	3d 
  66d880:	62 f1 fe 48 6f 54 24 	vmovdqu64 zmm2,ZMMWORD PTR [rsp+0x1040]
  66d887:	41 
  66d888:	62 f1 fe 48 6f 4c 24 	vmovdqu64 zmm1,ZMMWORD PTR [rsp+0x1140]
  66d88f:	45 
  66d890:	62 f1 fe 48 6f 44 24 	vmovdqu64 zmm0,ZMMWORD PTR [rsp+0x1240]
  66d897:	49 
  66d898:	62 71 fe 48 6f 7c 24 	vmovdqu64 zmm15,ZMMWORD PTR [rsp+0xd80]
  66d89f:	36 
  66d8a0:	62 71 fe 48 6f 74 24 	vmovdqu64 zmm14,ZMMWORD PTR [rsp+0xe80]
  66d8a7:	3a 
  66d8a8:	62 71 fe 48 6f 6c 24 	vmovdqu64 zmm13,ZMMWORD PTR [rsp+0xf80]
  66d8af:	3e 
  66d8b0:	62 71 fe 48 6f 64 24 	vmovdqu64 zmm12,ZMMWORD PTR [rsp+0x1080]
  66d8b7:	42 
  66d8b8:	62 71 fe 48 6f 5c 24 	vmovdqu64 zmm11,ZMMWORD PTR [rsp+0x1180]
  66d8bf:	46 
  66d8c0:	62 f1 fe 48 6f 6c 24 	vmovdqu64 zmm5,ZMMWORD PTR [rsp+0x1280]
  66d8c7:	4a 
  66d8c8:	4c 39 94 24 c0 00 00 	cmp    QWORD PTR [rsp+0xc0],r10
  66d8cf:	00 
  66d8d0:	0f 84 8a f6 ff ff    	je     66cf60 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x2a50>
  66d8d6:	31 c0                	xor    eax,eax
  66d8d8:	48 8d 8c 24 cf 16 00 	lea    rcx,[rsp+0x16cf]
  66d8df:	00 
  66d8e0:	62 e2 7d 48 78 41 f9 	vpbroadcastb zmm16,BYTE PTR [rcx-0x7]
  66d8e7:	62 e1 fd 48 6f 4c 04 	vmovdqa64 zmm17,ZMMWORD PTR [rsp+rax*1+0x1ac0]
  66d8ee:	6b 
  66d8ef:	62 e1 fd 48 6f 54 04 	vmovdqa64 zmm18,ZMMWORD PTR [rsp+rax*1+0x1b00]
  66d8f6:	6c 
  66d8f7:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66d8fd:	62 31 2d 48 ef d3    	vpxord zmm10,zmm10,zmm19
  66d903:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66d909:	62 31 35 48 ef c8    	vpxord zmm9,zmm9,zmm16
  66d90f:	62 e2 7d 48 78 41 fa 	vpbroadcastb zmm16,BYTE PTR [rcx-0x6]
  66d916:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66d91c:	62 31 3d 48 ef c3    	vpxord zmm8,zmm8,zmm19
  66d922:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66d928:	62 b1 45 48 ef f8    	vpxord zmm7,zmm7,zmm16
  66d92e:	62 e2 7d 48 78 41 fb 	vpbroadcastb zmm16,BYTE PTR [rcx-0x5]
  66d935:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66d93b:	62 b1 4d 48 ef f3    	vpxord zmm6,zmm6,zmm19
  66d941:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66d947:	62 31 05 48 ef f8    	vpxord zmm15,zmm15,zmm16
  66d94d:	62 e2 7d 48 78 41 fc 	vpbroadcastb zmm16,BYTE PTR [rcx-0x4]
  66d954:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66d95a:	62 b1 5d 48 ef e3    	vpxord zmm4,zmm4,zmm19
  66d960:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66d966:	62 31 0d 48 ef f0    	vpxord zmm14,zmm14,zmm16
  66d96c:	62 e2 7d 48 78 41 fd 	vpbroadcastb zmm16,BYTE PTR [rcx-0x3]
  66d973:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66d979:	62 b1 65 48 ef db    	vpxord zmm3,zmm3,zmm19
  66d97f:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66d985:	62 31 15 48 ef e8    	vpxord zmm13,zmm13,zmm16
  66d98b:	62 e2 7d 48 78 41 fe 	vpbroadcastb zmm16,BYTE PTR [rcx-0x2]
  66d992:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66d998:	62 b1 6d 48 ef d3    	vpxord zmm2,zmm2,zmm19
  66d99e:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66d9a4:	62 31 1d 48 ef e0    	vpxord zmm12,zmm12,zmm16
  66d9aa:	62 e2 7d 48 78 41 ff 	vpbroadcastb zmm16,BYTE PTR [rcx-0x1]
  66d9b1:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66d9b7:	62 b1 75 48 ef cb    	vpxord zmm1,zmm1,zmm19
  66d9bd:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66d9c3:	62 31 25 48 ef d8    	vpxord zmm11,zmm11,zmm16
  66d9c9:	62 e2 7d 48 78 01    	vpbroadcastb zmm16,BYTE PTR [rcx]
  66d9cf:	62 a2 75 40 cf c8    	vgf2p8mulb zmm17,zmm17,zmm16
  66d9d5:	62 b1 7d 48 ef c1    	vpxord zmm0,zmm0,zmm17
  66d9db:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66d9e1:	62 b1 55 48 ef e8    	vpxord zmm5,zmm5,zmm16
  66d9e7:	48 83 c1 10          	add    rcx,0x10
  66d9eb:	48 83 e8 80          	sub    rax,0xffffffffffffff80
  66d9ef:	49 39 c5             	cmp    r13,rax
  66d9f2:	0f 85 e8 fe ff ff    	jne    66d8e0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x33d0>
  66d9f8:	e9 63 f5 ff ff       	jmp    66cf60 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x2a50>
  66d9fd:	bf f0 0f 00 00       	mov    edi,0xff0
  66da02:	c5 f8 77             	vzeroupper 
  66da05:	ff 15 7d 99 09 00    	call   QWORD PTR [rip+0x9997d]        # 707388 <malloc@GLIBC_2.2.5>
  66da0b:	48 85 c0             	test   rax,rax
  66da0e:	0f 84 3d 28 00 00    	je     670251 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5d41>
  66da14:	b9 0f 00 00 00       	mov    ecx,0xf
  66da19:	48 8d 94 24 c0 11 00 	lea    rdx,[rsp+0x11c0]
  66da20:	00 
  66da21:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  66da28:	0f 1f 84 00 00 00 00 
  66da2f:	00 
  66da30:	0f b6 b2 00 f1 ff ff 	movzx  esi,BYTE PTR [rdx-0xf00]
  66da37:	40 88 b4 24 a0 00 00 	mov    BYTE PTR [rsp+0xa0],sil
  66da3e:	00 
  66da3f:	0f b6 ba 00 f2 ff ff 	movzx  edi,BYTE PTR [rdx-0xe00]
  66da46:	44 0f b6 82 00 f3 ff 	movzx  r8d,BYTE PTR [rdx-0xd00]
  66da4d:	ff 
  66da4e:	44 0f b6 8a 00 f4 ff 	movzx  r9d,BYTE PTR [rdx-0xc00]
  66da55:	ff 
  66da56:	44 0f b6 92 00 f5 ff 	movzx  r10d,BYTE PTR [rdx-0xb00]
  66da5d:	ff 
  66da5e:	44 0f b6 9a 00 f6 ff 	movzx  r11d,BYTE PTR [rdx-0xa00]
  66da65:	ff 
  66da66:	0f b6 9a 00 f7 ff ff 	movzx  ebx,BYTE PTR [rdx-0x900]
  66da6d:	44 0f b6 b2 00 f8 ff 	movzx  r14d,BYTE PTR [rdx-0x800]
  66da74:	ff 
  66da75:	44 0f b6 ba 00 f9 ff 	movzx  r15d,BYTE PTR [rdx-0x700]
  66da7c:	ff 
  66da7d:	44 0f b6 a2 00 fa ff 	movzx  r12d,BYTE PTR [rdx-0x600]
  66da84:	ff 
  66da85:	44 0f b6 aa 00 fb ff 	movzx  r13d,BYTE PTR [rdx-0x500]
  66da8c:	ff 
  66da8d:	0f b6 b2 00 fc ff ff 	movzx  esi,BYTE PTR [rdx-0x400]
  66da94:	40 88 b4 24 c0 00 00 	mov    BYTE PTR [rsp+0xc0],sil
  66da9b:	00 
  66da9c:	0f b6 b2 00 fd ff ff 	movzx  esi,BYTE PTR [rdx-0x300]
  66daa3:	40 88 74 24 40       	mov    BYTE PTR [rsp+0x40],sil
  66daa8:	0f b6 b2 00 fe ff ff 	movzx  esi,BYTE PTR [rdx-0x200]
  66daaf:	40 88 b4 24 40 01 00 	mov    BYTE PTR [rsp+0x140],sil
  66dab6:	00 
  66dab7:	0f b6 b2 00 ff ff ff 	movzx  esi,BYTE PTR [rdx-0x100]
  66dabe:	40 88 74 24 20       	mov    BYTE PTR [rsp+0x20],sil
  66dac3:	0f b6 32             	movzx  esi,BYTE PTR [rdx]
  66dac6:	40 88 74 24 18       	mov    BYTE PTR [rsp+0x18],sil
  66dacb:	0f b6 b4 24 a0 00 00 	movzx  esi,BYTE PTR [rsp+0xa0]
  66dad2:	00 
  66dad3:	40 88 74 08 f1       	mov    BYTE PTR [rax+rcx*1-0xf],sil
  66dad8:	40 88 7c 08 f2       	mov    BYTE PTR [rax+rcx*1-0xe],dil
  66dadd:	44 88 44 08 f3       	mov    BYTE PTR [rax+rcx*1-0xd],r8b
  66dae2:	44 88 4c 08 f4       	mov    BYTE PTR [rax+rcx*1-0xc],r9b
  66dae7:	44 88 54 08 f5       	mov    BYTE PTR [rax+rcx*1-0xb],r10b
  66daec:	44 88 5c 08 f6       	mov    BYTE PTR [rax+rcx*1-0xa],r11b
  66daf1:	88 5c 08 f7          	mov    BYTE PTR [rax+rcx*1-0x9],bl
  66daf5:	44 88 74 08 f8       	mov    BYTE PTR [rax+rcx*1-0x8],r14b
  66dafa:	44 88 7c 08 f9       	mov    BYTE PTR [rax+rcx*1-0x7],r15b
  66daff:	44 88 64 08 fa       	mov    BYTE PTR [rax+rcx*1-0x6],r12b
  66db04:	44 88 6c 08 fb       	mov    BYTE PTR [rax+rcx*1-0x5],r13b
  66db09:	0f b6 b4 24 c0 00 00 	movzx  esi,BYTE PTR [rsp+0xc0]
  66db10:	00 
  66db11:	40 88 74 08 fc       	mov    BYTE PTR [rax+rcx*1-0x4],sil
  66db16:	0f b6 74 24 40       	movzx  esi,BYTE PTR [rsp+0x40]
  66db1b:	40 88 74 08 fd       	mov    BYTE PTR [rax+rcx*1-0x3],sil
  66db20:	0f b6 b4 24 40 01 00 	movzx  esi,BYTE PTR [rsp+0x140]
  66db27:	00 
  66db28:	40 88 74 08 fe       	mov    BYTE PTR [rax+rcx*1-0x2],sil
  66db2d:	0f b6 74 24 20       	movzx  esi,BYTE PTR [rsp+0x20]
  66db32:	40 88 74 08 ff       	mov    BYTE PTR [rax+rcx*1-0x1],sil
  66db37:	0f b6 74 24 18       	movzx  esi,BYTE PTR [rsp+0x18]
  66db3c:	40 88 34 08          	mov    BYTE PTR [rax+rcx*1],sil
  66db40:	48 83 c1 10          	add    rcx,0x10
  66db44:	48 ff c2             	inc    rdx
  66db47:	48 81 f9 ff 0f 00 00 	cmp    rcx,0xfff
  66db4e:	0f 85 dc fe ff ff    	jne    66da30 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x3520>
  66db54:	e9 9b 22 00 00       	jmp    66fdf4 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x58e4>
  66db59:	8b 05 c9 c0 09 00    	mov    eax,DWORD PTR [rip+0x9c0c9]        # 709c28 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290+0x930>
  66db5f:	85 c0                	test   eax,eax
  66db61:	0f 85 f7 23 00 00    	jne    66ff5e <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5a4e>
  66db67:	48 89 4c 24 30       	mov    QWORD PTR [rsp+0x30],rcx
  66db6c:	8b 05 de d1 09 00    	mov    eax,DWORD PTR [rip+0x9d1de]        # 70ad50 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1120>
  66db72:	85 c0                	test   eax,eax
  66db74:	0f 85 0e 24 00 00    	jne    66ff88 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5a78>
  66db7a:	48 89 7c 24 50       	mov    QWORD PTR [rsp+0x50],rdi
  66db7f:	48 8b 44 24 28       	mov    rax,QWORD PTR [rsp+0x28]
  66db84:	48 c1 e8 04          	shr    rax,0x4
  66db88:	49 c1 ef 04          	shr    r15,0x4
  66db8c:	49 39 c7             	cmp    r15,rax
  66db8f:	4c 0f 43 f8          	cmovae r15,rax
  66db93:	48 8b 84 24 c0 00 00 	mov    rax,QWORD PTR [rsp+0xc0]
  66db9a:	00 
  66db9b:	4c 39 f8             	cmp    rax,r15
  66db9e:	4c 0f 42 f8          	cmovb  r15,rax
  66dba2:	48 8d bc 24 c0 1a 00 	lea    rdi,[rsp+0x1ac0]
  66dba9:	00 
  66dbaa:	ba 00 10 00 00       	mov    edx,0x1000
  66dbaf:	31 f6                	xor    esi,esi
  66dbb1:	ff 15 01 98 09 00    	call   QWORD PTR [rip+0x99801]        # 7073b8 <memset@GLIBC_2.2.5>
  66dbb7:	4d 85 ff             	test   r15,r15
  66dbba:	0f 84 b8 04 00 00    	je     66e078 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x3b68>
  66dbc0:	31 db                	xor    ebx,ebx
  66dbc2:	4c 89 74 24 60       	mov    QWORD PTR [rsp+0x60],r14
  66dbc7:	4c 89 7c 24 38       	mov    QWORD PTR [rsp+0x38],r15
  66dbcc:	0f 1f 40 00          	nop    DWORD PTR [rax+0x0]
  66dbd0:	49 89 dc             	mov    r12,rbx
  66dbd3:	49 c1 e4 04          	shl    r12,0x4
  66dbd7:	4d 01 e6             	add    r14,r12
  66dbda:	48 8b 44 24 30       	mov    rax,QWORD PTR [rsp+0x30]
  66dbdf:	4e 8d 3c 20          	lea    r15,[rax+r12*1]
  66dbe3:	4c 8d ac 24 c0 02 00 	lea    r13,[rsp+0x2c0]
  66dbea:	00 
  66dbeb:	4c 89 ef             	mov    rdi,r13
  66dbee:	4c 89 f6             	mov    rsi,r14
  66dbf1:	c5 f8 77             	vzeroupper 
  66dbf4:	e8 07 66 00 00       	call   674200 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate6domainNtB2_6Domain15eval_row_native>
  66dbf9:	62 f1 7c 48 10 44 24 	vmovups zmm0,ZMMWORD PTR [rsp+0x340]
  66dc00:	0d 
  66dc01:	62 f1 7c 48 29 44 24 	vmovaps ZMMWORD PTR [rsp+0xc0],zmm0
  66dc08:	03 
  66dc09:	c5 f8 10 84 24 a0 03 	vmovups xmm0,XMMWORD PTR [rsp+0x3a0]
  66dc10:	00 00 
  66dc12:	c5 fc 29 84 24 a0 00 	vmovaps YMMWORD PTR [rsp+0xa0],ymm0
  66dc19:	00 00 
  66dc1b:	48 8b 84 24 b0 03 00 	mov    rax,QWORD PTR [rsp+0x3b0]
  66dc22:	00 
  66dc23:	89 c1                	mov    ecx,eax
  66dc25:	c1 e9 08             	shr    ecx,0x8
  66dc28:	c4 e1 f9 6e c0       	vmovq  xmm0,rax
  66dc2d:	c4 e3 79 22 84 24 b8 	vpinsrd xmm0,xmm0,DWORD PTR [rsp+0x3b8],0x2
  66dc34:	03 00 00 02 
  66dc38:	c5 f9 c4 8c 24 bc 03 	vpinsrw xmm1,xmm0,WORD PTR [rsp+0x3bc],0x6
  66dc3f:	00 00 06 
  66dc42:	c4 e3 71 20 d1 01    	vpinsrb xmm2,xmm1,ecx,0x1
  66dc48:	89 c1                	mov    ecx,eax
  66dc4a:	c1 e9 10             	shr    ecx,0x10
  66dc4d:	c4 e3 69 20 d1 02    	vpinsrb xmm2,xmm2,ecx,0x2
  66dc53:	89 c1                	mov    ecx,eax
  66dc55:	c1 e9 18             	shr    ecx,0x18
  66dc58:	c4 e3 69 20 d1 03    	vpinsrb xmm2,xmm2,ecx,0x3
  66dc5e:	48 89 c1             	mov    rcx,rax
  66dc61:	48 c1 e9 20          	shr    rcx,0x20
  66dc65:	c4 e3 69 20 d1 04    	vpinsrb xmm2,xmm2,ecx,0x4
  66dc6b:	48 89 c1             	mov    rcx,rax
  66dc6e:	48 c1 e9 28          	shr    rcx,0x28
  66dc72:	c4 e3 69 20 d1 05    	vpinsrb xmm2,xmm2,ecx,0x5
  66dc78:	48 89 c1             	mov    rcx,rax
  66dc7b:	48 c1 e9 30          	shr    rcx,0x30
  66dc7f:	c4 e3 69 20 d1 06    	vpinsrb xmm2,xmm2,ecx,0x6
  66dc85:	48 c1 e8 38          	shr    rax,0x38
  66dc89:	c4 e3 69 20 d0 07    	vpinsrb xmm2,xmm2,eax,0x7
  66dc8f:	c4 e3 69 02 c0 04    	vpblendd xmm0,xmm2,xmm0,0x4
  66dc95:	c4 e3 79 0e c1 40    	vpblendw xmm0,xmm0,xmm1,0x40
  66dc9b:	c4 e3 79 20 84 24 be 	vpinsrb xmm0,xmm0,BYTE PTR [rsp+0x3be],0xe
  66dca2:	03 00 00 0e 
  66dca6:	c5 f9 7f 44 24 40    	vmovdqa XMMWORD PTR [rsp+0x40],xmm0
  66dcac:	c5 fc 10 84 24 80 03 	vmovups ymm0,YMMWORD PTR [rsp+0x380]
  66dcb3:	00 00 
  66dcb5:	62 f1 7c 48 29 44 24 	vmovaps ZMMWORD PTR [rsp+0x140],zmm0
  66dcbc:	05 
  66dcbd:	4c 89 ef             	mov    rdi,r13
  66dcc0:	4c 89 fe             	mov    rsi,r15
  66dcc3:	c5 f8 77             	vzeroupper 
  66dcc6:	e8 35 65 00 00       	call   674200 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate6domainNtB2_6Domain15eval_row_native>
  66dccb:	48 8b 3d 3e b6 09 00 	mov    rdi,QWORD PTR [rip+0x9b63e]        # 709310 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290+0x18>
  66dcd2:	48 83 ff 0f          	cmp    rdi,0xf
  66dcd6:	0f 86 ff 24 00 00    	jbe    6701db <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5ccb>
  66dcdc:	48 8b 05 25 b6 09 00 	mov    rax,QWORD PTR [rip+0x9b625]        # 709308 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290+0x10>
  66dce3:	4c 8b 54 24 70       	mov    r10,QWORD PTR [rsp+0x70]
  66dce8:	4b 8b 0c 22          	mov    rcx,QWORD PTR [r10+r12*1]
  66dcec:	48 89 cf             	mov    rdi,rcx
  66dcef:	48 89 ce             	mov    rsi,rcx
  66dcf2:	48 89 ca             	mov    rdx,rcx
  66dcf5:	49 89 c9             	mov    r9,rcx
  66dcf8:	41 89 c8             	mov    r8d,ecx
  66dcfb:	41 c1 e8 04          	shr    r8d,0x4
  66dcff:	41 81 e0 f0 0f 00 00 	and    r8d,0xff0
  66dd06:	c4 a1 79 6f 84 00 00 	vmovdqa xmm0,XMMWORD PTR [rax+r8*1+0x1000]
  66dd0d:	10 00 00 
  66dd10:	44 0f b6 c1          	movzx  r8d,cl
  66dd14:	41 c1 e0 04          	shl    r8d,0x4
  66dd18:	c4 a1 79 ef 04 00    	vpxor  xmm0,xmm0,XMMWORD PTR [rax+r8*1]
  66dd1e:	41 89 c8             	mov    r8d,ecx
  66dd21:	c1 e9 14             	shr    ecx,0x14
  66dd24:	83 e1 f0             	and    ecx,0xfffffff0
  66dd27:	c5 f9 6f 8c 08 00 30 	vmovdqa xmm1,XMMWORD PTR [rax+rcx*1+0x3000]
  66dd2e:	00 00 
  66dd30:	41 c1 e8 0c          	shr    r8d,0xc
  66dd34:	41 81 e0 f0 0f 00 00 	and    r8d,0xff0
  66dd3b:	62 b3 fd 08 25 8c 00 	vpternlogq xmm1,xmm0,XMMWORD PTR [rax+r8*1+0x2000],0x96
  66dd42:	00 20 00 00 96 
  66dd47:	4f 8b 44 22 08       	mov    r8,QWORD PTR [r10+r12*1+0x8]
  66dd4c:	4c 89 c1             	mov    rcx,r8
  66dd4f:	48 c1 ea 28          	shr    rdx,0x28
  66dd53:	0f b6 d2             	movzx  edx,dl
  66dd56:	c1 e2 04             	shl    edx,0x4
  66dd59:	c5 f9 6f 84 10 00 50 	vmovdqa xmm0,XMMWORD PTR [rax+rdx*1+0x5000]
  66dd60:	00 00 
  66dd62:	4c 89 c2             	mov    rdx,r8
  66dd65:	49 c1 e9 20          	shr    r9,0x20
  66dd69:	45 0f b6 c9          	movzx  r9d,r9b
  66dd6d:	41 c1 e1 04          	shl    r9d,0x4
  66dd71:	62 b3 f5 08 25 84 08 	vpternlogq xmm0,xmm1,XMMWORD PTR [rax+r9*1+0x4000],0x96
  66dd78:	00 40 00 00 96 
  66dd7d:	4d 89 c1             	mov    r9,r8
  66dd80:	48 c1 ef 38          	shr    rdi,0x38
  66dd84:	c1 e7 04             	shl    edi,0x4
  66dd87:	c5 f9 6f 8c 38 00 70 	vmovdqa xmm1,XMMWORD PTR [rax+rdi*1+0x7000]
  66dd8e:	00 00 
  66dd90:	4c 89 c7             	mov    rdi,r8
  66dd93:	48 c1 ee 30          	shr    rsi,0x30
  66dd97:	40 0f b6 f6          	movzx  esi,sil
  66dd9b:	c1 e6 04             	shl    esi,0x4
  66dd9e:	62 f3 fd 08 25 8c 30 	vpternlogq xmm1,xmm0,XMMWORD PTR [rax+rsi*1+0x6000],0x96
  66dda5:	00 60 00 00 96 
  66ddaa:	44 89 c6             	mov    esi,r8d
  66ddad:	c1 ee 04             	shr    esi,0x4
  66ddb0:	81 e6 f0 0f 00 00    	and    esi,0xff0
  66ddb6:	c5 f9 6f 84 30 00 90 	vmovdqa xmm0,XMMWORD PTR [rax+rsi*1+0x9000]
  66ddbd:	00 00 
  66ddbf:	41 0f b6 f0          	movzx  esi,r8b
  66ddc3:	c1 e6 04             	shl    esi,0x4
  66ddc6:	62 f3 f5 08 25 84 30 	vpternlogq xmm0,xmm1,XMMWORD PTR [rax+rsi*1+0x8000],0x96
  66ddcd:	00 80 00 00 96 
  66ddd2:	44 89 c6             	mov    esi,r8d
  66ddd5:	41 c1 e8 14          	shr    r8d,0x14
  66ddd9:	41 83 e0 f0          	and    r8d,0xfffffff0
  66dddd:	c4 a1 79 6f 8c 00 00 	vmovdqa xmm1,XMMWORD PTR [rax+r8*1+0xb000]
  66dde4:	b0 00 00 
  66dde7:	c1 ee 0c             	shr    esi,0xc
  66ddea:	81 e6 f0 0f 00 00    	and    esi,0xff0
  66ddf0:	62 f3 fd 08 25 8c 30 	vpternlogq xmm1,xmm0,XMMWORD PTR [rax+rsi*1+0xa000],0x96
  66ddf7:	00 a0 00 00 96 
  66ddfc:	49 c1 e9 28          	shr    r9,0x28
  66de00:	41 0f b6 f1          	movzx  esi,r9b
  66de04:	c1 e6 04             	shl    esi,0x4
  66de07:	c5 f9 6f 84 30 00 d0 	vmovdqa xmm0,XMMWORD PTR [rax+rsi*1+0xd000]
  66de0e:	00 00 
  66de10:	48 c1 ef 20          	shr    rdi,0x20
  66de14:	40 0f b6 f7          	movzx  esi,dil
  66de18:	c1 e6 04             	shl    esi,0x4
  66de1b:	62 f3 f5 08 25 84 30 	vpternlogq xmm0,xmm1,XMMWORD PTR [rax+rsi*1+0xc000],0x96
  66de22:	00 c0 00 00 96 
  66de27:	48 c1 e9 38          	shr    rcx,0x38
  66de2b:	c1 e1 04             	shl    ecx,0x4
  66de2e:	c5 f9 6f 8c 08 00 f0 	vmovdqa xmm1,XMMWORD PTR [rax+rcx*1+0xf000]
  66de35:	00 00 
  66de37:	48 c1 ea 30          	shr    rdx,0x30
  66de3b:	0f b6 ca             	movzx  ecx,dl
  66de3e:	c1 e1 04             	shl    ecx,0x4
  66de41:	62 f3 fd 08 25 8c 08 	vpternlogq xmm1,xmm0,XMMWORD PTR [rax+rcx*1+0xe000],0x96
  66de48:	00 e0 00 00 96 
  66de4d:	48 8b 84 24 b0 03 00 	mov    rax,QWORD PTR [rsp+0x3b0]
  66de54:	00 
  66de55:	89 c1                	mov    ecx,eax
  66de57:	c1 e9 08             	shr    ecx,0x8
  66de5a:	c4 e1 f9 6e c0       	vmovq  xmm0,rax
  66de5f:	c4 e3 79 22 84 24 b8 	vpinsrd xmm0,xmm0,DWORD PTR [rsp+0x3b8],0x2
  66de66:	03 00 00 02 
  66de6a:	c5 f9 c4 94 24 bc 03 	vpinsrw xmm2,xmm0,WORD PTR [rsp+0x3bc],0x6
  66de71:	00 00 06 
  66de74:	c4 e3 69 20 d9 01    	vpinsrb xmm3,xmm2,ecx,0x1
  66de7a:	89 c1                	mov    ecx,eax
  66de7c:	c1 e9 10             	shr    ecx,0x10
  66de7f:	c4 e3 61 20 d9 02    	vpinsrb xmm3,xmm3,ecx,0x2
  66de85:	89 c1                	mov    ecx,eax
  66de87:	c1 e9 18             	shr    ecx,0x18
  66de8a:	c4 e3 61 20 d9 03    	vpinsrb xmm3,xmm3,ecx,0x3
  66de90:	48 89 c1             	mov    rcx,rax
  66de93:	48 c1 e9 20          	shr    rcx,0x20
  66de97:	c4 e3 61 20 d9 04    	vpinsrb xmm3,xmm3,ecx,0x4
  66de9d:	48 89 c1             	mov    rcx,rax
  66dea0:	48 c1 e9 28          	shr    rcx,0x28
  66dea4:	c4 e3 61 20 d9 05    	vpinsrb xmm3,xmm3,ecx,0x5
  66deaa:	48 89 c1             	mov    rcx,rax
  66dead:	48 c1 e9 30          	shr    rcx,0x30
  66deb1:	c4 e3 61 20 d9 06    	vpinsrb xmm3,xmm3,ecx,0x6
  66deb7:	62 f1 fe 48 6f 64 24 	vmovdqu64 zmm4,ZMMWORD PTR [rsp+0x340]
  66debe:	0d 
  66debf:	48 c1 e8 38          	shr    rax,0x38
  66dec3:	c4 e3 61 20 d8 07    	vpinsrb xmm3,xmm3,eax,0x7
  66dec9:	c4 e3 61 02 c0 04    	vpblendd xmm0,xmm3,xmm0,0x4
  66decf:	c5 fa 6f 9c 24 a0 03 	vmovdqu xmm3,XMMWORD PTR [rsp+0x3a0]
  66ded6:	00 00 
  66ded8:	c4 e3 79 0e c2 40    	vpblendw xmm0,xmm0,xmm2,0x40
  66dede:	c4 e3 79 20 94 24 be 	vpinsrb xmm2,xmm0,BYTE PTR [rsp+0x3be],0xe
  66dee5:	03 00 00 0e 
  66dee9:	c5 fe 6f ac 24 80 03 	vmovdqu ymm5,YMMWORD PTR [rsp+0x380]
  66def0:	00 00 
  66def2:	c5 fa 7f 8c 24 e0 02 	vmovdqu XMMWORD PTR [rsp+0x2e0],xmm1
  66def9:	00 00 
  66defb:	49 8b 07             	mov    rax,QWORD PTR [r15]
  66defe:	49 8b 4f 08          	mov    rcx,QWORD PTR [r15+0x8]
  66df02:	49 23 06             	and    rax,QWORD PTR [r14]
  66df05:	49 23 4e 08          	and    rcx,QWORD PTR [r14+0x8]
  66df09:	c5 fd 6f 84 24 a0 00 	vmovdqa ymm0,YMMWORD PTR [rsp+0xa0]
  66df10:	00 00 
  66df12:	c4 e3 7d 38 44 24 40 	vinserti128 ymm0,ymm0,XMMWORD PTR [rsp+0x40],0x1
  66df19:	01 
  66df1a:	62 f1 fd 48 6f 4c 24 	vmovdqa64 zmm1,ZMMWORD PTR [rsp+0x140]
  66df21:	05 
  66df22:	62 f3 f5 48 3a c8 01 	vinserti64x4 zmm1,zmm1,ymm0,0x1
  66df29:	48 ff c3             	inc    rbx
  66df2c:	62 f2 5d 48 cf 44 24 	vgf2p8mulb zmm0,zmm4,ZMMWORD PTR [rsp+0xc0]
  66df33:	03 
  66df34:	c4 e3 65 38 d2 01    	vinserti128 ymm2,ymm3,xmm2,0x1
  66df3a:	62 f3 d5 48 3a d2 01 	vinserti64x4 zmm2,zmm5,ymm2,0x1
  66df41:	62 f1 fd 48 6f 1d f5 	vmovdqa64 zmm3,ZMMWORD PTR [rip+0xffffffffff9a45f5]        # 12540 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x13f2>
  66df48:	45 9a ff 
  66df4b:	62 f1 ed 48 db d3    	vpandq zmm2,zmm2,zmm3
  66df51:	62 f1 f5 48 db cb    	vpandq zmm1,zmm1,zmm3
  66df57:	62 f2 75 48 cf ca    	vgf2p8mulb zmm1,zmm1,zmm2
  66df5d:	48 8d 94 24 f0 02 00 	lea    rdx,[rsp+0x2f0]
  66df64:	00 
  66df65:	c5 e9 ef d2          	vpxor  xmm2,xmm2,xmm2
  66df69:	c5 fa 7f 12          	vmovdqu XMMWORD PTR [rdx],xmm2
  66df6d:	c4 e1 fb 92 c8       	kmovq  k1,rax
  66df72:	c4 e1 fb 92 d1       	kmovq  k2,rcx
  66df77:	48 8d 84 24 e1 02 00 	lea    rax,[rsp+0x2e1]
  66df7e:	00 
  66df7f:	31 c9                	xor    ecx,ecx
  66df81:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  66df88:	0f 1f 84 00 00 00 00 
  66df8f:	00 
  66df90:	62 f2 7d 48 78 50 ff 	vpbroadcastb zmm2,BYTE PTR [rax-0x1]
  66df97:	62 f1 7f c9 6f da    	vmovdqu8 zmm3{k1}{z},zmm2
  66df9d:	62 f1 65 48 ef 5c 0c 	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x1ac0]
  66dfa4:	6b 
  66dfa5:	62 f1 fe 48 7f 5c 0c 	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x1ac0],zmm3
  66dfac:	6b 
  66dfad:	62 f1 7f ca 6f da    	vmovdqu8 zmm3{k2}{z},zmm2
  66dfb3:	62 f1 65 48 ef 5c 0c 	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x1b00]
  66dfba:	6c 
  66dfbb:	62 f1 fe 48 7f 5c 0c 	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x1b00],zmm3
  66dfc2:	6c 
  66dfc3:	62 f2 7d 48 cf da    	vgf2p8mulb zmm3,zmm0,zmm2
  66dfc9:	62 f1 65 48 ef 5c 0c 	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x1b40]
  66dfd0:	6d 
  66dfd1:	62 f1 fe 48 7f 5c 0c 	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x1b40],zmm3
  66dfd8:	6d 
  66dfd9:	62 f2 75 48 cf d2    	vgf2p8mulb zmm2,zmm1,zmm2
  66dfdf:	62 f1 6d 48 ef 54 0c 	vpxord zmm2,zmm2,ZMMWORD PTR [rsp+rcx*1+0x1b80]
  66dfe6:	6e 
  66dfe7:	62 f1 fe 48 7f 54 0c 	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x1b80],zmm2
  66dfee:	6e 
  66dfef:	62 f2 7d 48 78 10    	vpbroadcastb zmm2,BYTE PTR [rax]
  66dff5:	62 f1 7f c9 6f da    	vmovdqu8 zmm3{k1}{z},zmm2
  66dffb:	62 f1 65 48 ef 5c 0c 	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x1bc0]
  66e002:	6f 
  66e003:	62 f1 fe 48 7f 5c 0c 	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x1bc0],zmm3
  66e00a:	6f 
  66e00b:	62 f1 7f ca 6f da    	vmovdqu8 zmm3{k2}{z},zmm2
  66e011:	62 f1 65 48 ef 5c 0c 	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x1c00]
  66e018:	70 
  66e019:	62 f1 fe 48 7f 5c 0c 	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x1c00],zmm3
  66e020:	70 
  66e021:	62 f2 7d 48 cf da    	vgf2p8mulb zmm3,zmm0,zmm2
  66e027:	62 f1 65 48 ef 5c 0c 	vpxord zmm3,zmm3,ZMMWORD PTR [rsp+rcx*1+0x1c40]
  66e02e:	71 
  66e02f:	62 f1 fe 48 7f 5c 0c 	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x1c40],zmm3
  66e036:	71 
  66e037:	62 f2 75 48 cf d2    	vgf2p8mulb zmm2,zmm1,zmm2
  66e03d:	62 f1 6d 48 ef 54 0c 	vpxord zmm2,zmm2,ZMMWORD PTR [rsp+rcx*1+0x1c80]
  66e044:	72 
  66e045:	62 f1 fe 48 7f 54 0c 	vmovdqu64 ZMMWORD PTR [rsp+rcx*1+0x1c80],zmm2
  66e04c:	72 
  66e04d:	48 81 c1 00 02 00 00 	add    rcx,0x200
  66e054:	48 83 c0 02          	add    rax,0x2
  66e058:	48 81 f9 00 10 00 00 	cmp    rcx,0x1000
  66e05f:	0f 85 2b ff ff ff    	jne    66df90 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x3a80>
  66e065:	4c 8b 7c 24 38       	mov    r15,QWORD PTR [rsp+0x38]
  66e06a:	4c 39 fb             	cmp    rbx,r15
  66e06d:	4c 8b 74 24 60       	mov    r14,QWORD PTR [rsp+0x60]
  66e072:	0f 85 58 fb ff ff    	jne    66dbd0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x36c0>
  66e078:	bf f0 0f 00 00       	mov    edi,0xff0
  66e07d:	c5 f8 77             	vzeroupper 
  66e080:	ff 15 02 93 09 00    	call   QWORD PTR [rip+0x99302]        # 707388 <malloc@GLIBC_2.2.5>
  66e086:	48 85 c0             	test   rax,rax
  66e089:	0f 84 d2 21 00 00    	je     670261 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5d51>
  66e08f:	b9 0f 00 00 00       	mov    ecx,0xf
  66e094:	48 8d 94 24 c0 29 00 	lea    rdx,[rsp+0x29c0]
  66e09b:	00 
  66e09c:	0f 1f 40 00          	nop    DWORD PTR [rax+0x0]
  66e0a0:	0f b6 b2 00 f1 ff ff 	movzx  esi,BYTE PTR [rdx-0xf00]
  66e0a7:	40 88 b4 24 a0 00 00 	mov    BYTE PTR [rsp+0xa0],sil
  66e0ae:	00 
  66e0af:	0f b6 ba 00 f2 ff ff 	movzx  edi,BYTE PTR [rdx-0xe00]
  66e0b6:	44 0f b6 82 00 f3 ff 	movzx  r8d,BYTE PTR [rdx-0xd00]
  66e0bd:	ff 
  66e0be:	44 0f b6 8a 00 f4 ff 	movzx  r9d,BYTE PTR [rdx-0xc00]
  66e0c5:	ff 
  66e0c6:	44 0f b6 92 00 f5 ff 	movzx  r10d,BYTE PTR [rdx-0xb00]
  66e0cd:	ff 
  66e0ce:	44 0f b6 9a 00 f6 ff 	movzx  r11d,BYTE PTR [rdx-0xa00]
  66e0d5:	ff 
  66e0d6:	0f b6 9a 00 f7 ff ff 	movzx  ebx,BYTE PTR [rdx-0x900]
  66e0dd:	44 0f b6 b2 00 f8 ff 	movzx  r14d,BYTE PTR [rdx-0x800]
  66e0e4:	ff 
  66e0e5:	44 0f b6 ba 00 f9 ff 	movzx  r15d,BYTE PTR [rdx-0x700]
  66e0ec:	ff 
  66e0ed:	44 0f b6 a2 00 fa ff 	movzx  r12d,BYTE PTR [rdx-0x600]
  66e0f4:	ff 
  66e0f5:	44 0f b6 aa 00 fb ff 	movzx  r13d,BYTE PTR [rdx-0x500]
  66e0fc:	ff 
  66e0fd:	0f b6 b2 00 fc ff ff 	movzx  esi,BYTE PTR [rdx-0x400]
  66e104:	40 88 b4 24 c0 00 00 	mov    BYTE PTR [rsp+0xc0],sil
  66e10b:	00 
  66e10c:	0f b6 b2 00 fd ff ff 	movzx  esi,BYTE PTR [rdx-0x300]
  66e113:	40 88 74 24 40       	mov    BYTE PTR [rsp+0x40],sil
  66e118:	0f b6 b2 00 fe ff ff 	movzx  esi,BYTE PTR [rdx-0x200]
  66e11f:	40 88 b4 24 40 01 00 	mov    BYTE PTR [rsp+0x140],sil
  66e126:	00 
  66e127:	0f b6 b2 00 ff ff ff 	movzx  esi,BYTE PTR [rdx-0x100]
  66e12e:	40 88 74 24 20       	mov    BYTE PTR [rsp+0x20],sil
  66e133:	0f b6 32             	movzx  esi,BYTE PTR [rdx]
  66e136:	40 88 74 24 18       	mov    BYTE PTR [rsp+0x18],sil
  66e13b:	0f b6 b4 24 a0 00 00 	movzx  esi,BYTE PTR [rsp+0xa0]
  66e142:	00 
  66e143:	40 88 74 08 f1       	mov    BYTE PTR [rax+rcx*1-0xf],sil
  66e148:	40 88 7c 08 f2       	mov    BYTE PTR [rax+rcx*1-0xe],dil
  66e14d:	44 88 44 08 f3       	mov    BYTE PTR [rax+rcx*1-0xd],r8b
  66e152:	44 88 4c 08 f4       	mov    BYTE PTR [rax+rcx*1-0xc],r9b
  66e157:	44 88 54 08 f5       	mov    BYTE PTR [rax+rcx*1-0xb],r10b
  66e15c:	44 88 5c 08 f6       	mov    BYTE PTR [rax+rcx*1-0xa],r11b
  66e161:	88 5c 08 f7          	mov    BYTE PTR [rax+rcx*1-0x9],bl
  66e165:	44 88 74 08 f8       	mov    BYTE PTR [rax+rcx*1-0x8],r14b
  66e16a:	44 88 7c 08 f9       	mov    BYTE PTR [rax+rcx*1-0x7],r15b
  66e16f:	44 88 64 08 fa       	mov    BYTE PTR [rax+rcx*1-0x6],r12b
  66e174:	44 88 6c 08 fb       	mov    BYTE PTR [rax+rcx*1-0x5],r13b
  66e179:	0f b6 b4 24 c0 00 00 	movzx  esi,BYTE PTR [rsp+0xc0]
  66e180:	00 
  66e181:	40 88 74 08 fc       	mov    BYTE PTR [rax+rcx*1-0x4],sil
  66e186:	0f b6 74 24 40       	movzx  esi,BYTE PTR [rsp+0x40]
  66e18b:	40 88 74 08 fd       	mov    BYTE PTR [rax+rcx*1-0x3],sil
  66e190:	0f b6 b4 24 40 01 00 	movzx  esi,BYTE PTR [rsp+0x140]
  66e197:	00 
  66e198:	40 88 74 08 fe       	mov    BYTE PTR [rax+rcx*1-0x2],sil
  66e19d:	0f b6 74 24 20       	movzx  esi,BYTE PTR [rsp+0x20]
  66e1a2:	40 88 74 08 ff       	mov    BYTE PTR [rax+rcx*1-0x1],sil
  66e1a7:	0f b6 74 24 18       	movzx  esi,BYTE PTR [rsp+0x18]
  66e1ac:	40 88 34 08          	mov    BYTE PTR [rax+rcx*1],sil
  66e1b0:	48 83 c1 10          	add    rcx,0x10
  66e1b4:	48 ff c2             	inc    rdx
  66e1b7:	48 81 f9 ff 0f 00 00 	cmp    rcx,0xfff
  66e1be:	0f 85 dc fe ff ff    	jne    66e0a0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x3b90>
  66e1c4:	e9 2b 1c 00 00       	jmp    66fdf4 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x58e4>
  66e1c9:	8b 05 81 cb 09 00    	mov    eax,DWORD PTR [rip+0x9cb81]        # 70ad50 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1120>
  66e1cf:	85 c0                	test   eax,eax
  66e1d1:	0f 85 c1 1d 00 00    	jne    66ff98 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5a88>
  66e1d7:	4d 89 f4             	mov    r12,r14
  66e1da:	48 89 4c 24 30       	mov    QWORD PTR [rsp+0x30],rcx
  66e1df:	48 89 7c 24 50       	mov    QWORD PTR [rsp+0x50],rdi
  66e1e4:	8b 05 3e ba 09 00    	mov    eax,DWORD PTR [rip+0x9ba3e]        # 709c28 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290+0x930>
  66e1ea:	85 c0                	test   eax,eax
  66e1ec:	0f 85 c2 1d 00 00    	jne    66ffb4 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5aa4>
  66e1f2:	48 8d bc 24 c0 02 00 	lea    rdi,[rsp+0x2c0]
  66e1f9:	00 
  66e1fa:	31 db                	xor    ebx,ebx
  66e1fc:	4c 8b 35 b5 91 09 00 	mov    r14,QWORD PTR [rip+0x991b5]        # 7073b8 <memset@GLIBC_2.2.5>
  66e203:	ba 00 10 00 00       	mov    edx,0x1000
  66e208:	31 f6                	xor    esi,esi
  66e20a:	41 ff d6             	call   r14
  66e20d:	48 8d bc 24 c0 16 00 	lea    rdi,[rsp+0x16c0]
  66e214:	00 
  66e215:	ba 00 04 00 00       	mov    edx,0x400
  66e21a:	31 f6                	xor    esi,esi
  66e21c:	41 ff d6             	call   r14
  66e21f:	48 8d bc 24 c0 12 00 	lea    rdi,[rsp+0x12c0]
  66e226:	00 
  66e227:	ba 00 04 00 00       	mov    edx,0x400
  66e22c:	31 f6                	xor    esi,esi
  66e22e:	41 ff d6             	call   r14
  66e231:	4d 85 ff             	test   r15,r15
  66e234:	4c 89 7c 24 38       	mov    QWORD PTR [rsp+0x38],r15
  66e239:	74 21                	je     66e25c <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x3d4c>
  66e23b:	48 8b 44 24 28       	mov    rax,QWORD PTR [rsp+0x28]
  66e240:	48 05 ff 03 00 00    	add    rax,0x3ff
  66e246:	48 c1 e8 0a          	shr    rax,0xa
  66e24a:	49 8d 9f ff 03 00 00 	lea    rbx,[r15+0x3ff]
  66e251:	48 c1 eb 0a          	shr    rbx,0xa
  66e255:	48 39 c3             	cmp    rbx,rax
  66e258:	48 0f 43 d8          	cmovae rbx,rax
  66e25c:	4d 89 e6             	mov    r14,r12
  66e25f:	48 83 bc 24 c0 00 00 	cmp    QWORD PTR [rsp+0xc0],0x0
  66e266:	00 00 
  66e268:	0f 84 95 17 00 00    	je     66fa03 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x54f3>
  66e26e:	48 8b 84 24 c0 00 00 	mov    rax,QWORD PTR [rsp+0xc0]
  66e275:	00 
  66e276:	48 83 c0 3f          	add    rax,0x3f
  66e27a:	48 c1 e8 06          	shr    rax,0x6
  66e27e:	48 39 d8             	cmp    rax,rbx
  66e281:	48 0f 43 c3          	cmovae rax,rbx
  66e285:	48 89 44 24 58       	mov    QWORD PTR [rsp+0x58],rax
  66e28a:	48 85 c0             	test   rax,rax
  66e28d:	0f 84 70 17 00 00    	je     66fa03 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x54f3>
  66e293:	31 c9                	xor    ecx,ecx
  66e295:	48 8b 84 24 c0 00 00 	mov    rax,QWORD PTR [rsp+0xc0]
  66e29c:	00 
  66e29d:	48 8b 54 24 38       	mov    rdx,QWORD PTR [rsp+0x38]
  66e2a2:	48 8b 74 24 28       	mov    rsi,QWORD PTR [rsp+0x28]
  66e2a7:	eb 56                	jmp    66e2ff <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x3def>
  66e2a9:	0f 1f 80 00 00 00 00 	nop    DWORD PTR [rax+0x0]
  66e2b0:	48 8b 5c 24 30       	mov    rbx,QWORD PTR [rsp+0x30]
  66e2b5:	48 8b 4c 24 40       	mov    rcx,QWORD PTR [rsp+0x40]
  66e2ba:	48 ff c1             	inc    rcx
  66e2bd:	48 8b 74 24 18       	mov    rsi,QWORD PTR [rsp+0x18]
  66e2c2:	48 81 c6 00 fc ff ff 	add    rsi,0xfffffffffffffc00
  66e2c9:	48 8b 54 24 20       	mov    rdx,QWORD PTR [rsp+0x20]
  66e2ce:	48 81 c2 00 fc ff ff 	add    rdx,0xfffffffffffffc00
  66e2d5:	49 81 c6 00 04 00 00 	add    r14,0x400
  66e2dc:	48 81 c3 00 04 00 00 	add    rbx,0x400
  66e2e3:	48 89 5c 24 30       	mov    QWORD PTR [rsp+0x30],rbx
  66e2e8:	48 8b 84 24 40 01 00 	mov    rax,QWORD PTR [rsp+0x140]
  66e2ef:	00 
  66e2f0:	48 83 c0 c0          	add    rax,0xffffffffffffffc0
  66e2f4:	48 3b 4c 24 58       	cmp    rcx,QWORD PTR [rsp+0x58]
  66e2f9:	0f 84 04 17 00 00    	je     66fa03 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x54f3>
  66e2ff:	48 83 f8 40          	cmp    rax,0x40
  66e303:	41 bc 40 00 00 00    	mov    r12d,0x40
  66e309:	48 89 84 24 40 01 00 	mov    QWORD PTR [rsp+0x140],rax
  66e310:	00 
  66e311:	4c 0f 42 e0          	cmovb  r12,rax
  66e315:	48 81 fa 00 04 00 00 	cmp    rdx,0x400
  66e31c:	b8 00 04 00 00       	mov    eax,0x400
  66e321:	48 89 54 24 20       	mov    QWORD PTR [rsp+0x20],rdx
  66e326:	48 0f 42 c2          	cmovb  rax,rdx
  66e32a:	c1 e8 04             	shr    eax,0x4
  66e32d:	48 81 fe 00 04 00 00 	cmp    rsi,0x400
  66e334:	bb 00 04 00 00       	mov    ebx,0x400
  66e339:	48 89 74 24 18       	mov    QWORD PTR [rsp+0x18],rsi
  66e33e:	48 0f 42 de          	cmovb  rbx,rsi
  66e342:	c1 eb 04             	shr    ebx,0x4
  66e345:	48 39 d8             	cmp    rax,rbx
  66e348:	48 0f 42 d8          	cmovb  rbx,rax
  66e34c:	48 89 cf             	mov    rdi,rcx
  66e34f:	48 c1 e7 0a          	shl    rdi,0xa
  66e353:	4c 8b 7c 24 28       	mov    r15,QWORD PTR [rsp+0x28]
  66e358:	49 29 ff             	sub    r15,rdi
  66e35b:	49 81 ff 00 04 00 00 	cmp    r15,0x400
  66e362:	b8 00 04 00 00       	mov    eax,0x400
  66e367:	4c 0f 43 f8          	cmovae r15,rax
  66e36b:	4d 89 f5             	mov    r13,r14
  66e36e:	4c 8b 74 24 38       	mov    r14,QWORD PTR [rsp+0x38]
  66e373:	49 29 fe             	sub    r14,rdi
  66e376:	49 81 fe 00 04 00 00 	cmp    r14,0x400
  66e37d:	4c 0f 43 f0          	cmovae r14,rax
  66e381:	48 89 4c 24 40       	mov    QWORD PTR [rsp+0x40],rcx
  66e386:	48 89 c8             	mov    rax,rcx
  66e389:	48 c1 e0 06          	shl    rax,0x6
  66e38d:	48 8b 8c 24 c0 00 00 	mov    rcx,QWORD PTR [rsp+0xc0]
  66e394:	00 
  66e395:	48 89 84 24 a0 00 00 	mov    QWORD PTR [rsp+0xa0],rax
  66e39c:	00 
  66e39d:	48 29 c1             	sub    rcx,rax
  66e3a0:	48 83 f9 40          	cmp    rcx,0x40
  66e3a4:	b8 40 00 00 00       	mov    eax,0x40
  66e3a9:	48 0f 43 c8          	cmovae rcx,rax
  66e3ad:	48 03 7c 24 70       	add    rdi,QWORD PTR [rsp+0x70]
  66e3b2:	48 89 ce             	mov    rsi,rcx
  66e3b5:	48 8d 94 24 c0 16 00 	lea    rdx,[rsp+0x16c0]
  66e3bc:	00 
  66e3bd:	c5 f8 77             	vzeroupper 
  66e3c0:	e8 7b 4f 00 00       	call   673340 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate5fieldNtB2_10SmallField15decompose_block>
  66e3c5:	62 71 fd 48 6f 15 31 	vmovdqa64 zmm10,ZMMWORD PTR [rip+0xffffffffff9a4131]        # 12500 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x13b2>
  66e3cc:	41 9a ff 
  66e3cf:	62 71 fd 48 6f 0d e7 	vmovdqa64 zmm9,ZMMWORD PTR [rip+0xffffffffff9a40e7]        # 124c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1372>
  66e3d6:	40 9a ff 
  66e3d9:	41 c1 ef 04          	shr    r15d,0x4
  66e3dd:	41 c1 ee 04          	shr    r14d,0x4
  66e3e1:	4d 39 fe             	cmp    r14,r15
  66e3e4:	4d 0f 42 fe          	cmovb  r15,r14
  66e3e8:	4d 89 ee             	mov    r14,r13
  66e3eb:	4d 85 ff             	test   r15,r15
  66e3ee:	4c 8b 5c 24 30       	mov    r11,QWORD PTR [rsp+0x30]
  66e3f3:	0f 84 a3 04 00 00    	je     66e89c <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x438c>
  66e3f9:	48 8b 05 38 c8 09 00 	mov    rax,QWORD PTR [rip+0x9c838]        # 70ac38 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1008>
  66e400:	48 03 05 71 c8 09 00 	add    rax,QWORD PTR [rip+0x9c871]        # 70ac78 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1048>
  66e407:	b9 08 00 00 00       	mov    ecx,0x8
  66e40c:	0f 1f 40 00          	nop    DWORD PTR [rax+0x0]
  66e410:	c4 c1 78 10 44 0e f8 	vmovups xmm0,XMMWORD PTR [r14+rcx*1-0x8]
  66e417:	49 8b 54 0b f8       	mov    rdx,QWORD PTR [r11+rcx*1-0x8]
  66e41c:	49 8b 74 0e f8       	mov    rsi,QWORD PTR [r14+rcx*1-0x8]
  66e421:	c4 c1 78 54 44 0b f8 	vandps xmm0,xmm0,XMMWORD PTR [r11+rcx*1-0x8]
  66e428:	c5 f8 11 84 0c b8 12 	vmovups XMMWORD PTR [rsp+rcx*1+0x12b8],xmm0
  66e42f:	00 00 
  66e431:	40 0f b6 fe          	movzx  edi,sil
  66e435:	c1 e7 07             	shl    edi,0x7
  66e438:	44 0f b6 c2          	movzx  r8d,dl
  66e43c:	41 c1 e0 07          	shl    r8d,0x7
  66e440:	41 89 f1             	mov    r9d,esi
  66e443:	41 d1 e9             	shr    r9d,1
  66e446:	41 81 e1 80 7f 00 00 	and    r9d,0x7f80
  66e44d:	41 89 d2             	mov    r10d,edx
  66e450:	41 d1 ea             	shr    r10d,1
  66e453:	41 81 e2 80 7f 00 00 	and    r10d,0x7f80
  66e45a:	62 b1 7d 48 70 04 08 	vpshufd zmm0,ZMMWORD PTR [rax+r9*1],0x4e
  66e461:	4e 
  66e462:	62 f1 7d 48 ef 04 38 	vpxord zmm0,zmm0,ZMMWORD PTR [rax+rdi*1]
  66e469:	62 b1 7d 48 70 0c 10 	vpshufd zmm1,ZMMWORD PTR [rax+r10*1],0x4e
  66e470:	4e 
  66e471:	62 b1 75 48 ef 0c 00 	vpxord zmm1,zmm1,ZMMWORD PTR [rax+r8*1]
  66e478:	62 b1 7d 48 70 54 08 	vpshufd zmm2,ZMMWORD PTR [rax+r9*1+0x40],0x4e
  66e47f:	01 4e 
  66e481:	62 f1 6d 48 ef 54 38 	vpxord zmm2,zmm2,ZMMWORD PTR [rax+rdi*1+0x40]
  66e488:	01 
  66e489:	62 b1 7d 48 70 5c 10 	vpshufd zmm3,ZMMWORD PTR [rax+r10*1+0x40],0x4e
  66e490:	01 4e 
  66e492:	62 b1 65 48 ef 5c 00 	vpxord zmm3,zmm3,ZMMWORD PTR [rax+r8*1+0x40]
  66e499:	01 
  66e49a:	89 f7                	mov    edi,esi
  66e49c:	c1 ef 09             	shr    edi,0x9
  66e49f:	81 e7 80 7f 00 00    	and    edi,0x7f80
  66e4a5:	41 89 d0             	mov    r8d,edx
  66e4a8:	41 c1 e8 09          	shr    r8d,0x9
  66e4ac:	41 81 e0 80 7f 00 00 	and    r8d,0x7f80
  66e4b3:	62 f3 fd 48 00 24 38 	vpermq zmm4,ZMMWORD PTR [rax+rdi*1],0x4e
  66e4ba:	4e 
  66e4bb:	62 b3 fd 48 00 2c 00 	vpermq zmm5,ZMMWORD PTR [rax+r8*1],0x4e
  66e4c2:	4e 
  66e4c3:	62 f3 fd 48 00 74 38 	vpermq zmm6,ZMMWORD PTR [rax+rdi*1+0x40],0x4e
  66e4ca:	01 4e 
  66e4cc:	62 b3 fd 48 00 7c 00 	vpermq zmm7,ZMMWORD PTR [rax+r8*1+0x40],0x4e
  66e4d3:	01 4e 
  66e4d5:	89 f7                	mov    edi,esi
  66e4d7:	c1 ef 11             	shr    edi,0x11
  66e4da:	83 e7 80             	and    edi,0xffffff80
  66e4dd:	41 89 d0             	mov    r8d,edx
  66e4e0:	41 c1 e8 11          	shr    r8d,0x11
  66e4e4:	41 83 e0 80          	and    r8d,0xffffff80
  66e4e8:	62 73 fd 48 00 04 38 	vpermq zmm8,ZMMWORD PTR [rax+rdi*1],0x1b
  66e4ef:	1b 
  66e4f0:	62 73 7d 48 25 c4 96 	vpternlogd zmm8,zmm0,zmm4,0x96
  66e4f7:	62 b3 fd 48 00 04 00 	vpermq zmm0,ZMMWORD PTR [rax+r8*1],0x1b
  66e4fe:	1b 
  66e4ff:	62 f3 75 48 25 c5 96 	vpternlogd zmm0,zmm1,zmm5,0x96
  66e506:	62 f3 fd 48 00 4c 38 	vpermq zmm1,ZMMWORD PTR [rax+rdi*1+0x40],0x1b
  66e50d:	01 1b 
  66e50f:	62 f3 6d 48 25 ce 96 	vpternlogd zmm1,zmm2,zmm6,0x96
  66e516:	62 b3 fd 48 00 54 00 	vpermq zmm2,ZMMWORD PTR [rax+r8*1+0x40],0x1b
  66e51d:	01 1b 
  66e51f:	62 f3 65 48 25 d7 96 	vpternlogd zmm2,zmm3,zmm7,0x96
  66e526:	48 c1 ee 19          	shr    rsi,0x19
  66e52a:	81 e6 80 7f 00 00    	and    esi,0x7f80
  66e530:	48 c1 ea 19          	shr    rdx,0x19
  66e534:	81 e2 80 7f 00 00    	and    edx,0x7f80
  66e53a:	62 f1 fe 48 6f 1c 30 	vmovdqu64 zmm3,ZMMWORD PTR [rax+rsi*1]
  66e541:	62 f1 fe 48 6f 64 30 	vmovdqu64 zmm4,ZMMWORD PTR [rax+rsi*1+0x40]
  66e548:	01 
  66e549:	62 f3 e5 48 43 db 4e 	vshufi64x2 zmm3,zmm3,zmm3,0x4e
  66e550:	62 f1 fe 48 6f 2c 10 	vmovdqu64 zmm5,ZMMWORD PTR [rax+rdx*1]
  66e557:	62 f1 fe 48 6f 74 10 	vmovdqu64 zmm6,ZMMWORD PTR [rax+rdx*1+0x40]
  66e55e:	01 
  66e55f:	62 f3 d5 48 43 ed 4e 	vshufi64x2 zmm5,zmm5,zmm5,0x4e
  66e566:	62 f3 dd 48 43 e4 4e 	vshufi64x2 zmm4,zmm4,zmm4,0x4e
  66e56d:	62 f3 cd 48 43 f6 4e 	vshufi64x2 zmm6,zmm6,zmm6,0x4e
  66e574:	41 0f b6 54 0e fd    	movzx  edx,BYTE PTR [r14+rcx*1-0x3]
  66e57a:	c1 e2 07             	shl    edx,0x7
  66e57d:	41 0f b6 74 0b fd    	movzx  esi,BYTE PTR [r11+rcx*1-0x3]
  66e583:	c1 e6 07             	shl    esi,0x7
  66e586:	62 f2 b5 48 36 3c 10 	vpermq zmm7,zmm9,ZMMWORD PTR [rax+rdx*1]
  66e58d:	62 f3 3d 48 25 fb 96 	vpternlogd zmm7,zmm8,zmm3,0x96
  66e594:	62 f2 b5 48 36 1c 30 	vpermq zmm3,zmm9,ZMMWORD PTR [rax+rsi*1]
  66e59b:	62 f3 7d 48 25 dd 96 	vpternlogd zmm3,zmm0,zmm5,0x96
  66e5a2:	62 f2 b5 48 36 44 10 	vpermq zmm0,zmm9,ZMMWORD PTR [rax+rdx*1+0x40]
  66e5a9:	01 
  66e5aa:	62 f3 75 48 25 c4 96 	vpternlogd zmm0,zmm1,zmm4,0x96
  66e5b1:	62 f2 b5 48 36 4c 30 	vpermq zmm1,zmm9,ZMMWORD PTR [rax+rsi*1+0x40]
  66e5b8:	01 
  66e5b9:	62 f3 6d 48 25 ce 96 	vpternlogd zmm1,zmm2,zmm6,0x96
  66e5c0:	41 0f b6 54 0e fe    	movzx  edx,BYTE PTR [r14+rcx*1-0x2]
  66e5c6:	c1 e2 07             	shl    edx,0x7
  66e5c9:	41 0f b6 74 0b fe    	movzx  esi,BYTE PTR [r11+rcx*1-0x2]
  66e5cf:	c1 e6 07             	shl    esi,0x7
  66e5d2:	62 f1 fe 48 6f 14 10 	vmovdqu64 zmm2,ZMMWORD PTR [rax+rdx*1]
  66e5d9:	62 f1 fe 48 6f 64 10 	vmovdqu64 zmm4,ZMMWORD PTR [rax+rdx*1+0x40]
  66e5e0:	01 
  66e5e1:	62 f1 fe 48 6f 2c 30 	vmovdqu64 zmm5,ZMMWORD PTR [rax+rsi*1]
  66e5e8:	62 f1 fe 48 6f 74 30 	vmovdqu64 zmm6,ZMMWORD PTR [rax+rsi*1+0x40]
  66e5ef:	01 
  66e5f0:	62 f3 ed 48 43 d2 1b 	vshufi64x2 zmm2,zmm2,zmm2,0x1b
  66e5f7:	62 f3 d5 48 43 ed 1b 	vshufi64x2 zmm5,zmm5,zmm5,0x1b
  66e5fe:	62 f3 dd 48 43 e4 1b 	vshufi64x2 zmm4,zmm4,zmm4,0x1b
  66e605:	41 0f b6 54 0e ff    	movzx  edx,BYTE PTR [r14+rcx*1-0x1]
  66e60b:	c1 e2 07             	shl    edx,0x7
  66e60e:	41 0f b6 74 0b ff    	movzx  esi,BYTE PTR [r11+rcx*1-0x1]
  66e614:	c1 e6 07             	shl    esi,0x7
  66e617:	62 72 ad 48 36 04 10 	vpermq zmm8,zmm10,ZMMWORD PTR [rax+rdx*1]
  66e61e:	62 73 45 48 25 c2 96 	vpternlogd zmm8,zmm7,zmm2,0x96
  66e625:	62 f2 ad 48 36 14 30 	vpermq zmm2,zmm10,ZMMWORD PTR [rax+rsi*1]
  66e62c:	62 f3 cd 48 43 f6 1b 	vshufi64x2 zmm6,zmm6,zmm6,0x1b
  66e633:	62 f3 65 48 25 d5 96 	vpternlogd zmm2,zmm3,zmm5,0x96
  66e63a:	62 f2 ad 48 36 5c 10 	vpermq zmm3,zmm10,ZMMWORD PTR [rax+rdx*1+0x40]
  66e641:	01 
  66e642:	62 f2 ad 48 36 6c 30 	vpermq zmm5,zmm10,ZMMWORD PTR [rax+rsi*1+0x40]
  66e649:	01 
  66e64a:	62 f3 7d 48 25 dc 96 	vpternlogd zmm3,zmm0,zmm4,0x96
  66e651:	62 f3 75 48 25 ee 96 	vpternlogd zmm5,zmm1,zmm6,0x96
  66e658:	41 0f b6 14 0e       	movzx  edx,BYTE PTR [r14+rcx*1]
  66e65d:	c1 e2 07             	shl    edx,0x7
  66e660:	41 0f b6 34 0b       	movzx  esi,BYTE PTR [r11+rcx*1]
  66e665:	c1 e6 07             	shl    esi,0x7
  66e668:	41 0f b6 7c 0e 01    	movzx  edi,BYTE PTR [r14+rcx*1+0x1]
  66e66e:	c1 e7 07             	shl    edi,0x7
  66e671:	45 0f b6 44 0b 01    	movzx  r8d,BYTE PTR [r11+rcx*1+0x1]
  66e677:	41 c1 e0 07          	shl    r8d,0x7
  66e67b:	62 f1 7d 48 70 44 38 	vpshufd zmm0,ZMMWORD PTR [rax+rdi*1+0x40],0x4e
  66e682:	01 4e 
  66e684:	62 f3 3d 48 25 44 10 	vpternlogd zmm0,zmm8,ZMMWORD PTR [rax+rdx*1+0x40],0x96
  66e68b:	01 96 
  66e68d:	62 b1 7d 48 70 4c 00 	vpshufd zmm1,ZMMWORD PTR [rax+r8*1+0x40],0x4e
  66e694:	01 4e 
  66e696:	62 f3 6d 48 25 4c 30 	vpternlogd zmm1,zmm2,ZMMWORD PTR [rax+rsi*1+0x40],0x96
  66e69d:	01 96 
  66e69f:	62 f1 7d 48 70 14 38 	vpshufd zmm2,ZMMWORD PTR [rax+rdi*1],0x4e
  66e6a6:	4e 
  66e6a7:	62 f3 65 48 25 14 10 	vpternlogd zmm2,zmm3,ZMMWORD PTR [rax+rdx*1],0x96
  66e6ae:	96 
  66e6af:	62 b1 7d 48 70 1c 00 	vpshufd zmm3,ZMMWORD PTR [rax+r8*1],0x4e
  66e6b6:	4e 
  66e6b7:	62 f3 55 48 25 1c 30 	vpternlogd zmm3,zmm5,ZMMWORD PTR [rax+rsi*1],0x96
  66e6be:	96 
  66e6bf:	41 0f b6 54 0e 02    	movzx  edx,BYTE PTR [r14+rcx*1+0x2]
  66e6c5:	c1 e2 07             	shl    edx,0x7
  66e6c8:	41 0f b6 74 0b 02    	movzx  esi,BYTE PTR [r11+rcx*1+0x2]
  66e6ce:	c1 e6 07             	shl    esi,0x7
  66e6d1:	62 f3 fd 48 00 64 10 	vpermq zmm4,ZMMWORD PTR [rax+rdx*1+0x40],0x4e
  66e6d8:	01 4e 
  66e6da:	62 f3 fd 48 00 6c 30 	vpermq zmm5,ZMMWORD PTR [rax+rsi*1+0x40],0x4e
  66e6e1:	01 4e 
  66e6e3:	62 f3 fd 48 00 34 10 	vpermq zmm6,ZMMWORD PTR [rax+rdx*1],0x4e
  66e6ea:	4e 
  66e6eb:	62 f3 fd 48 00 3c 30 	vpermq zmm7,ZMMWORD PTR [rax+rsi*1],0x4e
  66e6f2:	4e 
  66e6f3:	41 0f b6 54 0e 03    	movzx  edx,BYTE PTR [r14+rcx*1+0x3]
  66e6f9:	c1 e2 07             	shl    edx,0x7
  66e6fc:	41 0f b6 74 0b 03    	movzx  esi,BYTE PTR [r11+rcx*1+0x3]
  66e702:	c1 e6 07             	shl    esi,0x7
  66e705:	62 73 fd 48 00 44 10 	vpermq zmm8,ZMMWORD PTR [rax+rdx*1+0x40],0x1b
  66e70c:	01 1b 
  66e70e:	62 73 7d 48 25 c4 96 	vpternlogd zmm8,zmm0,zmm4,0x96
  66e715:	62 f3 fd 48 00 44 30 	vpermq zmm0,ZMMWORD PTR [rax+rsi*1+0x40],0x1b
  66e71c:	01 1b 
  66e71e:	62 f3 75 48 25 c5 96 	vpternlogd zmm0,zmm1,zmm5,0x96
  66e725:	62 f3 fd 48 00 0c 10 	vpermq zmm1,ZMMWORD PTR [rax+rdx*1],0x1b
  66e72c:	1b 
  66e72d:	62 f3 6d 48 25 ce 96 	vpternlogd zmm1,zmm2,zmm6,0x96
  66e734:	62 f3 fd 48 00 14 30 	vpermq zmm2,ZMMWORD PTR [rax+rsi*1],0x1b
  66e73b:	1b 
  66e73c:	62 f3 65 48 25 d7 96 	vpternlogd zmm2,zmm3,zmm7,0x96
  66e743:	41 0f b6 54 0e 04    	movzx  edx,BYTE PTR [r14+rcx*1+0x4]
  66e749:	c1 e2 07             	shl    edx,0x7
  66e74c:	41 0f b6 74 0b 04    	movzx  esi,BYTE PTR [r11+rcx*1+0x4]
  66e752:	c1 e6 07             	shl    esi,0x7
  66e755:	62 f1 fe 48 6f 1c 10 	vmovdqu64 zmm3,ZMMWORD PTR [rax+rdx*1]
  66e75c:	62 f1 fe 48 6f 64 10 	vmovdqu64 zmm4,ZMMWORD PTR [rax+rdx*1+0x40]
  66e763:	01 
  66e764:	62 f1 fe 48 6f 2c 30 	vmovdqu64 zmm5,ZMMWORD PTR [rax+rsi*1]
  66e76b:	62 f1 fe 48 6f 74 30 	vmovdqu64 zmm6,ZMMWORD PTR [rax+rsi*1+0x40]
  66e772:	01 
  66e773:	62 f3 dd 48 43 e4 4e 	vshufi64x2 zmm4,zmm4,zmm4,0x4e
  66e77a:	62 f3 cd 48 43 f6 4e 	vshufi64x2 zmm6,zmm6,zmm6,0x4e
  66e781:	62 f3 e5 48 43 db 4e 	vshufi64x2 zmm3,zmm3,zmm3,0x4e
  66e788:	41 0f b6 54 0e 05    	movzx  edx,BYTE PTR [r14+rcx*1+0x5]
  66e78e:	c1 e2 07             	shl    edx,0x7
  66e791:	41 0f b6 74 0b 05    	movzx  esi,BYTE PTR [r11+rcx*1+0x5]
  66e797:	c1 e6 07             	shl    esi,0x7
  66e79a:	62 f2 b5 48 36 7c 10 	vpermq zmm7,zmm9,ZMMWORD PTR [rax+rdx*1+0x40]
  66e7a1:	01 
  66e7a2:	62 f3 3d 48 25 fc 96 	vpternlogd zmm7,zmm8,zmm4,0x96
  66e7a9:	62 f2 b5 48 36 64 30 	vpermq zmm4,zmm9,ZMMWORD PTR [rax+rsi*1+0x40]
  66e7b0:	01 
  66e7b1:	62 f3 d5 48 43 ed 4e 	vshufi64x2 zmm5,zmm5,zmm5,0x4e
  66e7b8:	62 f3 7d 48 25 e6 96 	vpternlogd zmm4,zmm0,zmm6,0x96
  66e7bf:	62 f2 b5 48 36 04 10 	vpermq zmm0,zmm9,ZMMWORD PTR [rax+rdx*1]
  66e7c6:	62 f2 b5 48 36 34 30 	vpermq zmm6,zmm9,ZMMWORD PTR [rax+rsi*1]
  66e7cd:	62 f3 75 48 25 c3 96 	vpternlogd zmm0,zmm1,zmm3,0x96
  66e7d4:	62 f3 6d 48 25 f5 96 	vpternlogd zmm6,zmm2,zmm5,0x96
  66e7db:	41 0f b6 54 0e 06    	movzx  edx,BYTE PTR [r14+rcx*1+0x6]
  66e7e1:	c1 e2 07             	shl    edx,0x7
  66e7e4:	41 0f b6 74 0b 06    	movzx  esi,BYTE PTR [r11+rcx*1+0x6]
  66e7ea:	c1 e6 07             	shl    esi,0x7
  66e7ed:	62 f1 fe 48 6f 0c 10 	vmovdqu64 zmm1,ZMMWORD PTR [rax+rdx*1]
  66e7f4:	62 f1 fe 48 6f 54 10 	vmovdqu64 zmm2,ZMMWORD PTR [rax+rdx*1+0x40]
  66e7fb:	01 
  66e7fc:	62 f1 fe 48 6f 1c 30 	vmovdqu64 zmm3,ZMMWORD PTR [rax+rsi*1]
  66e803:	62 f3 ed 48 43 d2 1b 	vshufi64x2 zmm2,zmm2,zmm2,0x1b
  66e80a:	62 f1 fe 48 6f 6c 30 	vmovdqu64 zmm5,ZMMWORD PTR [rax+rsi*1+0x40]
  66e811:	01 
  66e812:	62 f3 d5 48 43 ed 1b 	vshufi64x2 zmm5,zmm5,zmm5,0x1b
  66e819:	62 f3 f5 48 43 c9 1b 	vshufi64x2 zmm1,zmm1,zmm1,0x1b
  66e820:	62 f3 e5 48 43 db 1b 	vshufi64x2 zmm3,zmm3,zmm3,0x1b
  66e827:	41 0f b6 54 0e 07    	movzx  edx,BYTE PTR [r14+rcx*1+0x7]
  66e82d:	c1 e2 07             	shl    edx,0x7
  66e830:	41 0f b6 74 0b 07    	movzx  esi,BYTE PTR [r11+rcx*1+0x7]
  66e836:	c1 e6 07             	shl    esi,0x7
  66e839:	62 72 ad 48 36 44 10 	vpermq zmm8,zmm10,ZMMWORD PTR [rax+rdx*1+0x40]
  66e840:	01 
  66e841:	62 73 45 48 25 c2 96 	vpternlogd zmm8,zmm7,zmm2,0x96
  66e848:	62 f2 ad 48 36 54 30 	vpermq zmm2,zmm10,ZMMWORD PTR [rax+rsi*1+0x40]
  66e84f:	01 
  66e850:	62 f3 5d 48 25 d5 96 	vpternlogd zmm2,zmm4,zmm5,0x96
  66e857:	62 f2 ad 48 36 24 10 	vpermq zmm4,zmm10,ZMMWORD PTR [rax+rdx*1]
  66e85e:	62 f2 3d 48 cf d2    	vgf2p8mulb zmm2,zmm8,zmm2
  66e864:	62 f3 7d 48 25 e1 96 	vpternlogd zmm4,zmm0,zmm1,0x96
  66e86b:	62 f2 ad 48 36 04 30 	vpermq zmm0,zmm10,ZMMWORD PTR [rax+rsi*1]
  66e872:	62 f3 4d 48 25 c3 96 	vpternlogd zmm0,zmm6,zmm3,0x96
  66e879:	62 f2 5d 48 cf c0    	vgf2p8mulb zmm0,zmm4,zmm0
  66e87f:	62 f1 fd 48 7f 54 cc 	vmovdqa64 ZMMWORD PTR [rsp+rcx*8+0x1a80],zmm2
  66e886:	6a 
  66e887:	62 f1 fd 48 7f 44 cc 	vmovdqa64 ZMMWORD PTR [rsp+rcx*8+0x1ac0],zmm0
  66e88e:	6b 
  66e88f:	48 83 c1 10          	add    rcx,0x10
  66e893:	48 ff cb             	dec    rbx
  66e896:	0f 85 74 fb ff ff    	jne    66e410 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x3f00>
  66e89c:	48 c7 84 24 08 02 00 	mov    QWORD PTR [rsp+0x208],0x2
  66e8a3:	00 02 00 00 00 
  66e8a8:	48 c7 84 24 10 02 00 	mov    QWORD PTR [rsp+0x210],0x0
  66e8af:	00 00 00 00 00 
  66e8b4:	48 c7 84 24 18 02 00 	mov    QWORD PTR [rsp+0x218],0x8
  66e8bb:	00 08 00 00 00 
  66e8c0:	48 8b 84 24 a0 00 00 	mov    rax,QWORD PTR [rsp+0xa0]
  66e8c7:	00 
  66e8c8:	48 39 84 24 c0 00 00 	cmp    QWORD PTR [rsp+0xc0],rax
  66e8cf:	00 
  66e8d0:	0f 84 da f9 ff ff    	je     66e2b0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x3da0>
  66e8d6:	41 c1 e4 04          	shl    r12d,0x4
  66e8da:	31 c0                	xor    eax,eax
  66e8dc:	31 c9                	xor    ecx,ecx
  66e8de:	48 8b 5c 24 30       	mov    rbx,QWORD PTR [rsp+0x30]
  66e8e3:	4c 8d bc 24 c3 16 00 	lea    r15,[rsp+0x16c3]
  66e8ea:	00 
  66e8eb:	4c 8d ac 24 c7 16 00 	lea    r13,[rsp+0x16c7]
  66e8f2:	00 
  66e8f3:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  66e8fa:	84 00 00 00 00 00 
  66e900:	48 8b bc cc 10 02 00 	mov    rdi,QWORD PTR [rsp+rcx*8+0x210]
  66e907:	00 
  66e908:	ba 10 00 00 00       	mov    edx,0x10
  66e90d:	48 29 fa             	sub    rdx,rdi
  66e910:	b9 00 00 00 00       	mov    ecx,0x0
  66e915:	48 0f 43 ca          	cmovae rcx,rdx
  66e919:	48 83 ff 10          	cmp    rdi,0x10
  66e91d:	0f 83 8c 18 00 00    	jae    6701af <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5c9f>
  66e923:	4c 8d 47 01          	lea    r8,[rdi+0x1]
  66e927:	48 83 f9 01          	cmp    rcx,0x1
  66e92b:	0f 84 b3 16 00 00    	je     66ffe4 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5ad4>
  66e931:	4c 8d 4f 02          	lea    r9,[rdi+0x2]
  66e935:	48 83 f9 02          	cmp    rcx,0x2
  66e939:	0f 84 6d 18 00 00    	je     6701ac <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5c9c>
  66e93f:	4c 8d 57 03          	lea    r10,[rdi+0x3]
  66e943:	48 83 f9 03          	cmp    rcx,0x3
  66e947:	0f 84 17 18 00 00    	je     670164 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5c54>
  66e94d:	48 8d 4f 04          	lea    rcx,[rdi+0x4]
  66e951:	48 89 fe             	mov    rsi,rdi
  66e954:	48 c1 e6 08          	shl    rsi,0x8
  66e958:	48 8d 94 34 c0 02 00 	lea    rdx,[rsp+rsi*1+0x2c0]
  66e95f:	00 
  66e960:	62 f1 fe 48 6f 44 34 	vmovdqu64 zmm0,ZMMWORD PTR [rsp+rsi*1+0x2c0]
  66e967:	0b 
  66e968:	49 c1 e0 08          	shl    r8,0x8
  66e96c:	4a 8d b4 04 c0 02 00 	lea    rsi,[rsp+r8*1+0x2c0]
  66e973:	00 
  66e974:	62 b1 fe 48 6f 4c 04 	vmovdqu64 zmm1,ZMMWORD PTR [rsp+r8*1+0x2c0]
  66e97b:	0b 
  66e97c:	49 c1 e1 08          	shl    r9,0x8
  66e980:	4e 8d 84 0c c0 02 00 	lea    r8,[rsp+r9*1+0x2c0]
  66e987:	00 
  66e988:	62 b1 fe 48 6f 54 0c 	vmovdqu64 zmm2,ZMMWORD PTR [rsp+r9*1+0x2c0]
  66e98f:	0b 
  66e990:	62 b1 fe 48 6f 5c 0c 	vmovdqu64 zmm3,ZMMWORD PTR [rsp+r9*1+0x300]
  66e997:	0c 
  66e998:	62 b1 fe 48 6f 64 0c 	vmovdqu64 zmm4,ZMMWORD PTR [rsp+r9*1+0x340]
  66e99f:	0d 
  66e9a0:	62 b1 fe 48 6f 6c 0c 	vmovdqu64 zmm5,ZMMWORD PTR [rsp+r9*1+0x380]
  66e9a7:	0e 
  66e9a8:	49 c1 e2 08          	shl    r10,0x8
  66e9ac:	4e 8d 8c 14 c0 02 00 	lea    r9,[rsp+r10*1+0x2c0]
  66e9b3:	00 
  66e9b4:	62 b1 fe 48 6f 74 14 	vmovdqu64 zmm6,ZMMWORD PTR [rsp+r10*1+0x2c0]
  66e9bb:	0b 
  66e9bc:	62 b1 fe 48 6f 7c 14 	vmovdqu64 zmm7,ZMMWORD PTR [rsp+r10*1+0x300]
  66e9c3:	0c 
  66e9c4:	62 31 fe 48 6f 44 14 	vmovdqu64 zmm8,ZMMWORD PTR [rsp+r10*1+0x340]
  66e9cb:	0d 
  66e9cc:	62 31 fe 48 6f 4c 14 	vmovdqu64 zmm9,ZMMWORD PTR [rsp+r10*1+0x380]
  66e9d3:	0e 
  66e9d4:	62 71 fe 48 6f 6a 01 	vmovdqu64 zmm13,ZMMWORD PTR [rdx+0x40]
  66e9db:	62 71 fe 48 6f 72 02 	vmovdqu64 zmm14,ZMMWORD PTR [rdx+0x80]
  66e9e2:	62 71 fe 48 6f 7a 03 	vmovdqu64 zmm15,ZMMWORD PTR [rdx+0xc0]
  66e9e9:	62 71 fe 48 6f 56 01 	vmovdqu64 zmm10,ZMMWORD PTR [rsi+0x40]
  66e9f0:	62 71 fe 48 6f 5e 02 	vmovdqu64 zmm11,ZMMWORD PTR [rsi+0x80]
  66e9f7:	62 71 fe 48 6f 66 03 	vmovdqu64 zmm12,ZMMWORD PTR [rsi+0xc0]
  66e9fe:	4d 8d 14 3f          	lea    r10,[r15+rdi*1]
  66ea02:	45 31 db             	xor    r11d,r11d
  66ea05:	66 66 2e 0f 1f 84 00 	data16 cs nop WORD PTR [rax+rax*1+0x0]
  66ea0c:	00 00 00 00 
  66ea10:	62 82 7d 48 78 44 1a 	vpbroadcastb zmm16,BYTE PTR [r10+r11*1-0x3]
  66ea17:	fd 
  66ea18:	c4 a1 f8 90 94 1c c0 	kmovq  k2,QWORD PTR [rsp+r11*1+0x12c0]
  66ea1f:	12 00 00 
  66ea22:	c4 a1 f8 90 8c 1c c8 	kmovq  k1,QWORD PTR [rsp+r11*1+0x12c8]
  66ea29:	12 00 00 
  66ea2c:	62 a1 7f ca 6f c8    	vmovdqu8 zmm17{k2}{z},zmm16
  66ea32:	62 b1 7d 48 ef c1    	vpxord zmm0,zmm0,zmm17
  66ea38:	62 a1 7f c9 6f c8    	vmovdqu8 zmm17{k1}{z},zmm16
  66ea3e:	62 31 15 48 ef e9    	vpxord zmm13,zmm13,zmm17
  66ea44:	62 a1 fd 48 6f 4c dc 	vmovdqa64 zmm17,ZMMWORD PTR [rsp+r11*8+0x1ac0]
  66ea4b:	6b 
  66ea4c:	62 a1 fd 48 6f 54 dc 	vmovdqa64 zmm18,ZMMWORD PTR [rsp+r11*8+0x1b00]
  66ea53:	6c 
  66ea54:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66ea5a:	62 31 0d 48 ef f3    	vpxord zmm14,zmm14,zmm19
  66ea60:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66ea66:	62 31 05 48 ef f8    	vpxord zmm15,zmm15,zmm16
  66ea6c:	62 82 7d 48 78 44 1a 	vpbroadcastb zmm16,BYTE PTR [r10+r11*1-0x2]
  66ea73:	fe 
  66ea74:	62 a1 7f ca 6f d8    	vmovdqu8 zmm19{k2}{z},zmm16
  66ea7a:	62 b1 75 48 ef cb    	vpxord zmm1,zmm1,zmm19
  66ea80:	62 a1 7f c9 6f d8    	vmovdqu8 zmm19{k1}{z},zmm16
  66ea86:	62 31 2d 48 ef d3    	vpxord zmm10,zmm10,zmm19
  66ea8c:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66ea92:	62 31 25 48 ef db    	vpxord zmm11,zmm11,zmm19
  66ea98:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66ea9e:	62 31 1d 48 ef e0    	vpxord zmm12,zmm12,zmm16
  66eaa4:	62 82 7d 48 78 44 1a 	vpbroadcastb zmm16,BYTE PTR [r10+r11*1-0x1]
  66eaab:	ff 
  66eaac:	62 a1 7f ca 6f d8    	vmovdqu8 zmm19{k2}{z},zmm16
  66eab2:	62 b1 6d 48 ef d3    	vpxord zmm2,zmm2,zmm19
  66eab8:	62 a1 7f c9 6f d8    	vmovdqu8 zmm19{k1}{z},zmm16
  66eabe:	62 b1 65 48 ef db    	vpxord zmm3,zmm3,zmm19
  66eac4:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66eaca:	62 b1 5d 48 ef e3    	vpxord zmm4,zmm4,zmm19
  66ead0:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66ead6:	62 b1 55 48 ef e8    	vpxord zmm5,zmm5,zmm16
  66eadc:	62 82 7d 48 78 04 1a 	vpbroadcastb zmm16,BYTE PTR [r10+r11*1]
  66eae3:	62 a1 7f ca 6f d8    	vmovdqu8 zmm19{k2}{z},zmm16
  66eae9:	62 b1 4d 48 ef f3    	vpxord zmm6,zmm6,zmm19
  66eaef:	62 a1 7f c9 6f d8    	vmovdqu8 zmm19{k1}{z},zmm16
  66eaf5:	62 b1 45 48 ef fb    	vpxord zmm7,zmm7,zmm19
  66eafb:	62 a2 75 40 cf c8    	vgf2p8mulb zmm17,zmm17,zmm16
  66eb01:	62 31 3d 48 ef c1    	vpxord zmm8,zmm8,zmm17
  66eb07:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66eb0d:	62 31 35 48 ef c8    	vpxord zmm9,zmm9,zmm16
  66eb13:	49 83 c3 10          	add    r11,0x10
  66eb17:	4d 39 dc             	cmp    r12,r11
  66eb1a:	0f 85 f0 fe ff ff    	jne    66ea10 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4500>
  66eb20:	62 f1 fe 48 7f 02    	vmovdqu64 ZMMWORD PTR [rdx],zmm0
  66eb26:	62 71 fe 48 7f 6a 01 	vmovdqu64 ZMMWORD PTR [rdx+0x40],zmm13
  66eb2d:	62 71 fe 48 7f 72 02 	vmovdqu64 ZMMWORD PTR [rdx+0x80],zmm14
  66eb34:	62 71 fe 48 7f 7a 03 	vmovdqu64 ZMMWORD PTR [rdx+0xc0],zmm15
  66eb3b:	62 f1 fe 48 7f 0e    	vmovdqu64 ZMMWORD PTR [rsi],zmm1
  66eb41:	62 71 fe 48 7f 56 01 	vmovdqu64 ZMMWORD PTR [rsi+0x40],zmm10
  66eb48:	62 71 fe 48 7f 5e 02 	vmovdqu64 ZMMWORD PTR [rsi+0x80],zmm11
  66eb4f:	62 71 fe 48 7f 66 03 	vmovdqu64 ZMMWORD PTR [rsi+0xc0],zmm12
  66eb56:	62 d1 fe 48 7f 10    	vmovdqu64 ZMMWORD PTR [r8],zmm2
  66eb5c:	62 d1 fe 48 7f 58 01 	vmovdqu64 ZMMWORD PTR [r8+0x40],zmm3
  66eb63:	62 d1 fe 48 7f 60 02 	vmovdqu64 ZMMWORD PTR [r8+0x80],zmm4
  66eb6a:	62 d1 fe 48 7f 68 03 	vmovdqu64 ZMMWORD PTR [r8+0xc0],zmm5
  66eb71:	62 d1 fe 48 7f 31    	vmovdqu64 ZMMWORD PTR [r9],zmm6
  66eb77:	62 d1 fe 48 7f 79 01 	vmovdqu64 ZMMWORD PTR [r9+0x40],zmm7
  66eb7e:	62 51 fe 48 7f 41 02 	vmovdqu64 ZMMWORD PTR [r9+0x80],zmm8
  66eb85:	62 51 fe 48 7f 49 03 	vmovdqu64 ZMMWORD PTR [r9+0xc0],zmm9
  66eb8c:	be 10 00 00 00       	mov    esi,0x10
  66eb91:	48 29 ce             	sub    rsi,rcx
  66eb94:	ba 00 00 00 00       	mov    edx,0x0
  66eb99:	48 0f 43 d6          	cmovae rdx,rsi
  66eb9d:	48 83 f9 0f          	cmp    rcx,0xf
  66eba1:	0f 87 a5 15 00 00    	ja     67014c <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5c3c>
  66eba7:	48 8d 77 05          	lea    rsi,[rdi+0x5]
  66ebab:	48 83 fa 01          	cmp    rdx,0x1
  66ebaf:	0f 84 df 15 00 00    	je     670194 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5c84>
  66ebb5:	4c 8d 47 06          	lea    r8,[rdi+0x6]
  66ebb9:	48 83 fa 02          	cmp    rdx,0x2
  66ebbd:	0f 84 21 14 00 00    	je     66ffe4 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5ad4>
  66ebc3:	4c 8d 4f 07          	lea    r9,[rdi+0x7]
  66ebc7:	48 83 fa 03          	cmp    rdx,0x3
  66ebcb:	0f 84 ab 15 00 00    	je     67017c <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5c6c>
  66ebd1:	48 c1 e1 08          	shl    rcx,0x8
  66ebd5:	48 8d 94 0c c0 02 00 	lea    rdx,[rsp+rcx*1+0x2c0]
  66ebdc:	00 
  66ebdd:	62 f1 fe 48 6f 44 0c 	vmovdqu64 zmm0,ZMMWORD PTR [rsp+rcx*1+0x2c0]
  66ebe4:	0b 
  66ebe5:	48 c1 e6 08          	shl    rsi,0x8
  66ebe9:	48 8d 8c 34 c0 02 00 	lea    rcx,[rsp+rsi*1+0x2c0]
  66ebf0:	00 
  66ebf1:	62 f1 fe 48 6f 4c 34 	vmovdqu64 zmm1,ZMMWORD PTR [rsp+rsi*1+0x2c0]
  66ebf8:	0b 
  66ebf9:	49 c1 e0 08          	shl    r8,0x8
  66ebfd:	4a 8d b4 04 c0 02 00 	lea    rsi,[rsp+r8*1+0x2c0]
  66ec04:	00 
  66ec05:	62 b1 fe 48 6f 54 04 	vmovdqu64 zmm2,ZMMWORD PTR [rsp+r8*1+0x2c0]
  66ec0c:	0b 
  66ec0d:	62 b1 fe 48 6f 5c 04 	vmovdqu64 zmm3,ZMMWORD PTR [rsp+r8*1+0x300]
  66ec14:	0c 
  66ec15:	62 b1 fe 48 6f 64 04 	vmovdqu64 zmm4,ZMMWORD PTR [rsp+r8*1+0x340]
  66ec1c:	0d 
  66ec1d:	62 b1 fe 48 6f 6c 04 	vmovdqu64 zmm5,ZMMWORD PTR [rsp+r8*1+0x380]
  66ec24:	0e 
  66ec25:	49 c1 e1 08          	shl    r9,0x8
  66ec29:	4e 8d 84 0c c0 02 00 	lea    r8,[rsp+r9*1+0x2c0]
  66ec30:	00 
  66ec31:	62 b1 fe 48 6f 74 0c 	vmovdqu64 zmm6,ZMMWORD PTR [rsp+r9*1+0x2c0]
  66ec38:	0b 
  66ec39:	62 b1 fe 48 6f 7c 0c 	vmovdqu64 zmm7,ZMMWORD PTR [rsp+r9*1+0x300]
  66ec40:	0c 
  66ec41:	62 31 fe 48 6f 44 0c 	vmovdqu64 zmm8,ZMMWORD PTR [rsp+r9*1+0x340]
  66ec48:	0d 
  66ec49:	62 31 fe 48 6f 4c 0c 	vmovdqu64 zmm9,ZMMWORD PTR [rsp+r9*1+0x380]
  66ec50:	0e 
  66ec51:	62 71 fe 48 6f 6a 01 	vmovdqu64 zmm13,ZMMWORD PTR [rdx+0x40]
  66ec58:	62 71 fe 48 6f 72 02 	vmovdqu64 zmm14,ZMMWORD PTR [rdx+0x80]
  66ec5f:	62 71 fe 48 6f 7a 03 	vmovdqu64 zmm15,ZMMWORD PTR [rdx+0xc0]
  66ec66:	62 71 fe 48 6f 51 01 	vmovdqu64 zmm10,ZMMWORD PTR [rcx+0x40]
  66ec6d:	62 71 fe 48 6f 59 02 	vmovdqu64 zmm11,ZMMWORD PTR [rcx+0x80]
  66ec74:	62 71 fe 48 6f 61 03 	vmovdqu64 zmm12,ZMMWORD PTR [rcx+0xc0]
  66ec7b:	4c 01 ef             	add    rdi,r13
  66ec7e:	45 31 c9             	xor    r9d,r9d
  66ec81:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  66ec88:	0f 1f 84 00 00 00 00 
  66ec8f:	00 
  66ec90:	62 a2 7d 48 78 44 0f 	vpbroadcastb zmm16,BYTE PTR [rdi+r9*1-0x3]
  66ec97:	fd 
  66ec98:	c4 a1 f8 90 94 0c c0 	kmovq  k2,QWORD PTR [rsp+r9*1+0x12c0]
  66ec9f:	12 00 00 
  66eca2:	c4 a1 f8 90 8c 0c c8 	kmovq  k1,QWORD PTR [rsp+r9*1+0x12c8]
  66eca9:	12 00 00 
  66ecac:	62 a1 7f ca 6f c8    	vmovdqu8 zmm17{k2}{z},zmm16
  66ecb2:	62 b1 7d 48 ef c1    	vpxord zmm0,zmm0,zmm17
  66ecb8:	62 a1 7f c9 6f c8    	vmovdqu8 zmm17{k1}{z},zmm16
  66ecbe:	62 31 15 48 ef e9    	vpxord zmm13,zmm13,zmm17
  66ecc4:	62 a1 fd 48 6f 4c cc 	vmovdqa64 zmm17,ZMMWORD PTR [rsp+r9*8+0x1ac0]
  66eccb:	6b 
  66eccc:	62 a1 fd 48 6f 54 cc 	vmovdqa64 zmm18,ZMMWORD PTR [rsp+r9*8+0x1b00]
  66ecd3:	6c 
  66ecd4:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66ecda:	62 31 0d 48 ef f3    	vpxord zmm14,zmm14,zmm19
  66ece0:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66ece6:	62 31 05 48 ef f8    	vpxord zmm15,zmm15,zmm16
  66ecec:	62 a2 7d 48 78 44 0f 	vpbroadcastb zmm16,BYTE PTR [rdi+r9*1-0x2]
  66ecf3:	fe 
  66ecf4:	62 a1 7f ca 6f d8    	vmovdqu8 zmm19{k2}{z},zmm16
  66ecfa:	62 b1 75 48 ef cb    	vpxord zmm1,zmm1,zmm19
  66ed00:	62 a1 7f c9 6f d8    	vmovdqu8 zmm19{k1}{z},zmm16
  66ed06:	62 31 2d 48 ef d3    	vpxord zmm10,zmm10,zmm19
  66ed0c:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66ed12:	62 31 25 48 ef db    	vpxord zmm11,zmm11,zmm19
  66ed18:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66ed1e:	62 31 1d 48 ef e0    	vpxord zmm12,zmm12,zmm16
  66ed24:	62 a2 7d 48 78 44 0f 	vpbroadcastb zmm16,BYTE PTR [rdi+r9*1-0x1]
  66ed2b:	ff 
  66ed2c:	62 a1 7f ca 6f d8    	vmovdqu8 zmm19{k2}{z},zmm16
  66ed32:	62 b1 6d 48 ef d3    	vpxord zmm2,zmm2,zmm19
  66ed38:	62 a1 7f c9 6f d8    	vmovdqu8 zmm19{k1}{z},zmm16
  66ed3e:	62 b1 65 48 ef db    	vpxord zmm3,zmm3,zmm19
  66ed44:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  66ed4a:	62 b1 5d 48 ef e3    	vpxord zmm4,zmm4,zmm19
  66ed50:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66ed56:	62 b1 55 48 ef e8    	vpxord zmm5,zmm5,zmm16
  66ed5c:	62 a2 7d 48 78 04 0f 	vpbroadcastb zmm16,BYTE PTR [rdi+r9*1]
  66ed63:	62 a1 7f ca 6f d8    	vmovdqu8 zmm19{k2}{z},zmm16
  66ed69:	62 b1 4d 48 ef f3    	vpxord zmm6,zmm6,zmm19
  66ed6f:	62 a1 7f c9 6f d8    	vmovdqu8 zmm19{k1}{z},zmm16
  66ed75:	62 b1 45 48 ef fb    	vpxord zmm7,zmm7,zmm19
  66ed7b:	62 a2 75 40 cf c8    	vgf2p8mulb zmm17,zmm17,zmm16
  66ed81:	62 31 3d 48 ef c1    	vpxord zmm8,zmm8,zmm17
  66ed87:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  66ed8d:	62 31 35 48 ef c8    	vpxord zmm9,zmm9,zmm16
  66ed93:	49 83 c1 10          	add    r9,0x10
  66ed97:	4d 39 cc             	cmp    r12,r9
  66ed9a:	0f 85 f0 fe ff ff    	jne    66ec90 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4780>
  66eda0:	62 f1 fe 48 7f 02    	vmovdqu64 ZMMWORD PTR [rdx],zmm0
  66eda6:	62 71 fe 48 7f 6a 01 	vmovdqu64 ZMMWORD PTR [rdx+0x40],zmm13
  66edad:	62 71 fe 48 7f 72 02 	vmovdqu64 ZMMWORD PTR [rdx+0x80],zmm14
  66edb4:	62 71 fe 48 7f 7a 03 	vmovdqu64 ZMMWORD PTR [rdx+0xc0],zmm15
  66edbb:	62 f1 fe 48 7f 09    	vmovdqu64 ZMMWORD PTR [rcx],zmm1
  66edc1:	62 71 fe 48 7f 51 01 	vmovdqu64 ZMMWORD PTR [rcx+0x40],zmm10
  66edc8:	62 71 fe 48 7f 59 02 	vmovdqu64 ZMMWORD PTR [rcx+0x80],zmm11
  66edcf:	62 71 fe 48 7f 61 03 	vmovdqu64 ZMMWORD PTR [rcx+0xc0],zmm12
  66edd6:	62 f1 fe 48 7f 16    	vmovdqu64 ZMMWORD PTR [rsi],zmm2
  66eddc:	62 f1 fe 48 7f 5e 01 	vmovdqu64 ZMMWORD PTR [rsi+0x40],zmm3
  66ede3:	62 f1 fe 48 7f 66 02 	vmovdqu64 ZMMWORD PTR [rsi+0x80],zmm4
  66edea:	62 f1 fe 48 7f 6e 03 	vmovdqu64 ZMMWORD PTR [rsi+0xc0],zmm5
  66edf1:	62 d1 fe 48 7f 30    	vmovdqu64 ZMMWORD PTR [r8],zmm6
  66edf7:	62 d1 fe 48 7f 78 01 	vmovdqu64 ZMMWORD PTR [r8+0x40],zmm7
  66edfe:	62 51 fe 48 7f 40 02 	vmovdqu64 ZMMWORD PTR [r8+0x80],zmm8
  66ee05:	62 51 fe 48 7f 48 03 	vmovdqu64 ZMMWORD PTR [r8+0xc0],zmm9
  66ee0c:	b9 01 00 00 00       	mov    ecx,0x1
  66ee11:	a8 01                	test   al,0x1
  66ee13:	b0 01                	mov    al,0x1
  66ee15:	0f 84 e5 fa ff ff    	je     66e900 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x43f0>
  66ee1b:	e9 95 f4 ff ff       	jmp    66e2b5 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x3da5>
  66ee20:	4c 89 f6             	mov    rsi,r14
  66ee23:	48 8b 54 24 28       	mov    rdx,QWORD PTR [rsp+0x28]
  66ee28:	4d 89 f8             	mov    r8,r15
  66ee2b:	4c 8b 4c 24 70       	mov    r9,QWORD PTR [rsp+0x70]
  66ee30:	6a 01                	push   0x1
  66ee32:	ff b4 24 c8 00 00 00 	push   QWORD PTR [rsp+0xc8]
  66ee39:	e8 82 e6 00 00       	call   67d4c0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled15aggregate_block>
  66ee3e:	48 83 c4 10          	add    rsp,0x10
  66ee42:	e9 c5 0f 00 00       	jmp    66fe0c <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x58fc>
  66ee47:	8b 05 03 bf 09 00    	mov    eax,DWORD PTR [rip+0x9bf03]        # 70ad50 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1120>
  66ee4d:	85 c0                	test   eax,eax
  66ee4f:	0f 85 69 11 00 00    	jne    66ffbe <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5aae>
  66ee55:	4c 89 74 24 60       	mov    QWORD PTR [rsp+0x60],r14
  66ee5a:	48 89 4c 24 30       	mov    QWORD PTR [rsp+0x30],rcx
  66ee5f:	48 89 7c 24 50       	mov    QWORD PTR [rsp+0x50],rdi
  66ee64:	8b 05 be ad 09 00    	mov    eax,DWORD PTR [rip+0x9adbe]        # 709c28 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate5field5field5FIELD.llvm.8267282299410697290+0x930>
  66ee6a:	85 c0                	test   eax,eax
  66ee6c:	0f 85 68 11 00 00    	jne    66ffda <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5aca>
  66ee72:	48 8d bc 24 c0 02 00 	lea    rdi,[rsp+0x2c0]
  66ee79:	00 
  66ee7a:	31 db                	xor    ebx,ebx
  66ee7c:	4c 8b 35 35 85 09 00 	mov    r14,QWORD PTR [rip+0x98535]        # 7073b8 <memset@GLIBC_2.2.5>
  66ee83:	ba 00 10 00 00       	mov    edx,0x1000
  66ee88:	31 f6                	xor    esi,esi
  66ee8a:	41 ff d6             	call   r14
  66ee8d:	48 8d bc 24 c0 16 00 	lea    rdi,[rsp+0x16c0]
  66ee94:	00 
  66ee95:	ba 00 04 00 00       	mov    edx,0x400
  66ee9a:	31 f6                	xor    esi,esi
  66ee9c:	41 ff d6             	call   r14
  66ee9f:	48 8d bc 24 c0 12 00 	lea    rdi,[rsp+0x12c0]
  66eea6:	00 
  66eea7:	ba 00 04 00 00       	mov    edx,0x400
  66eeac:	31 f6                	xor    esi,esi
  66eeae:	41 ff d6             	call   r14
  66eeb1:	4d 85 ff             	test   r15,r15
  66eeb4:	4c 89 7c 24 38       	mov    QWORD PTR [rsp+0x38],r15
  66eeb9:	74 21                	je     66eedc <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x49cc>
  66eebb:	48 8b 44 24 28       	mov    rax,QWORD PTR [rsp+0x28]
  66eec0:	48 05 ff 03 00 00    	add    rax,0x3ff
  66eec6:	48 c1 e8 0a          	shr    rax,0xa
  66eeca:	49 8d 9f ff 03 00 00 	lea    rbx,[r15+0x3ff]
  66eed1:	48 c1 eb 0a          	shr    rbx,0xa
  66eed5:	48 39 c3             	cmp    rbx,rax
  66eed8:	48 0f 43 d8          	cmovae rbx,rax
  66eedc:	48 83 bc 24 c0 00 00 	cmp    QWORD PTR [rsp+0xc0],0x0
  66eee3:	00 00 
  66eee5:	0f 84 be 0d 00 00    	je     66fca9 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5799>
  66eeeb:	48 8b 84 24 c0 00 00 	mov    rax,QWORD PTR [rsp+0xc0]
  66eef2:	00 
  66eef3:	48 83 c0 3f          	add    rax,0x3f
  66eef7:	48 c1 e8 06          	shr    rax,0x6
  66eefb:	48 39 d8             	cmp    rax,rbx
  66eefe:	48 0f 43 c3          	cmovae rax,rbx
  66ef02:	48 89 84 24 20 01 00 	mov    QWORD PTR [rsp+0x120],rax
  66ef09:	00 
  66ef0a:	48 85 c0             	test   rax,rax
  66ef0d:	0f 84 96 0d 00 00    	je     66fca9 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5799>
  66ef13:	48 bb c0 ff ff ff ff 	movabs rbx,0x7fffffffffffffc0
  66ef1a:	ff ff 7f 
  66ef1d:	31 c9                	xor    ecx,ecx
  66ef1f:	4c 8d 35 8a 38 9a ff 	lea    r14,[rip+0xffffffffff9a388a]        # 127b0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1662>
  66ef26:	48 8b 84 24 c0 00 00 	mov    rax,QWORD PTR [rsp+0xc0]
  66ef2d:	00 
  66ef2e:	48 8b 54 24 30       	mov    rdx,QWORD PTR [rsp+0x30]
  66ef33:	48 89 54 24 78       	mov    QWORD PTR [rsp+0x78],rdx
  66ef38:	48 8b 54 24 60       	mov    rdx,QWORD PTR [rsp+0x60]
  66ef3d:	48 89 94 24 90 00 00 	mov    QWORD PTR [rsp+0x90],rdx
  66ef44:	00 
  66ef45:	48 8b 54 24 38       	mov    rdx,QWORD PTR [rsp+0x38]
  66ef4a:	48 8b 74 24 28       	mov    rsi,QWORD PTR [rsp+0x28]
  66ef4f:	eb 61                	jmp    66efb2 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4aa2>
  66ef51:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  66ef58:	0f 1f 84 00 00 00 00 
  66ef5f:	00 
  66ef60:	48 8b b4 24 88 00 00 	mov    rsi,QWORD PTR [rsp+0x88]
  66ef67:	00 
  66ef68:	48 81 c6 00 fc ff ff 	add    rsi,0xfffffffffffffc00
  66ef6f:	48 8b 94 24 98 00 00 	mov    rdx,QWORD PTR [rsp+0x98]
  66ef76:	00 
  66ef77:	48 81 c2 00 fc ff ff 	add    rdx,0xfffffffffffffc00
  66ef7e:	48 81 84 24 90 00 00 	add    QWORD PTR [rsp+0x90],0x400
  66ef85:	00 00 04 00 00 
  66ef8a:	48 81 44 24 78 00 04 	add    QWORD PTR [rsp+0x78],0x400
  66ef91:	00 00 
  66ef93:	48 8b 84 24 80 00 00 	mov    rax,QWORD PTR [rsp+0x80]
  66ef9a:	00 
  66ef9b:	48 83 c0 c0          	add    rax,0xffffffffffffffc0
  66ef9f:	48 8b 4c 24 58       	mov    rcx,QWORD PTR [rsp+0x58]
  66efa4:	48 3b 8c 24 20 01 00 	cmp    rcx,QWORD PTR [rsp+0x120]
  66efab:	00 
  66efac:	0f 84 f7 0c 00 00    	je     66fca9 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5799>
  66efb2:	48 83 f8 40          	cmp    rax,0x40
  66efb6:	bf 40 00 00 00       	mov    edi,0x40
  66efbb:	48 89 84 24 80 00 00 	mov    QWORD PTR [rsp+0x80],rax
  66efc2:	00 
  66efc3:	48 0f 42 f8          	cmovb  rdi,rax
  66efc7:	48 89 bc 24 40 01 00 	mov    QWORD PTR [rsp+0x140],rdi
  66efce:	00 
  66efcf:	48 81 fe 00 04 00 00 	cmp    rsi,0x400
  66efd6:	b8 00 04 00 00       	mov    eax,0x400
  66efdb:	48 89 b4 24 88 00 00 	mov    QWORD PTR [rsp+0x88],rsi
  66efe2:	00 
  66efe3:	48 0f 42 c6          	cmovb  rax,rsi
  66efe7:	c1 e8 04             	shr    eax,0x4
  66efea:	48 81 fa 00 04 00 00 	cmp    rdx,0x400
  66eff1:	be 00 04 00 00       	mov    esi,0x400
  66eff6:	48 89 94 24 98 00 00 	mov    QWORD PTR [rsp+0x98],rdx
  66effd:	00 
  66effe:	48 0f 42 f2          	cmovb  rsi,rdx
  66f002:	c1 ee 04             	shr    esi,0x4
  66f005:	48 39 f0             	cmp    rax,rsi
  66f008:	48 0f 42 f0          	cmovb  rsi,rax
  66f00c:	48 89 74 24 40       	mov    QWORD PTR [rsp+0x40],rsi
  66f011:	48 89 ca             	mov    rdx,rcx
  66f014:	48 c1 e2 0a          	shl    rdx,0xa
  66f018:	4c 8b 7c 24 28       	mov    r15,QWORD PTR [rsp+0x28]
  66f01d:	49 29 d7             	sub    r15,rdx
  66f020:	49 81 ff 00 04 00 00 	cmp    r15,0x400
  66f027:	b8 00 04 00 00       	mov    eax,0x400
  66f02c:	4c 0f 43 f8          	cmovae r15,rax
  66f030:	4c 8b 64 24 38       	mov    r12,QWORD PTR [rsp+0x38]
  66f035:	49 29 d4             	sub    r12,rdx
  66f038:	49 81 fc 00 04 00 00 	cmp    r12,0x400
  66f03f:	4c 0f 43 e0          	cmovae r12,rax
  66f043:	48 89 4c 24 58       	mov    QWORD PTR [rsp+0x58],rcx
  66f048:	48 89 c8             	mov    rax,rcx
  66f04b:	48 c1 e0 06          	shl    rax,0x6
  66f04f:	48 8b 8c 24 c0 00 00 	mov    rcx,QWORD PTR [rsp+0xc0]
  66f056:	00 
  66f057:	48 89 84 24 a0 00 00 	mov    QWORD PTR [rsp+0xa0],rax
  66f05e:	00 
  66f05f:	48 29 c1             	sub    rcx,rax
  66f062:	48 83 f9 40          	cmp    rcx,0x40
  66f066:	b8 40 00 00 00       	mov    eax,0x40
  66f06b:	48 0f 43 c8          	cmovae rcx,rax
  66f06f:	48 8b 44 24 70       	mov    rax,QWORD PTR [rsp+0x70]
  66f074:	49 89 d5             	mov    r13,rdx
  66f077:	48 8d 3c 10          	lea    rdi,[rax+rdx*1]
  66f07b:	48 89 ce             	mov    rsi,rcx
  66f07e:	48 8d 94 24 c0 16 00 	lea    rdx,[rsp+0x16c0]
  66f085:	00 
  66f086:	c5 f8 77             	vzeroupper 
  66f089:	e8 b2 42 00 00       	call   673340 <_RNvMNtNtCs5Palesvl9wF_6mikros10univariate5fieldNtB2_10SmallField15decompose_block>
  66f08e:	62 f1 fd 48 6f 3d 68 	vmovdqa64 zmm7,ZMMWORD PTR [rip+0xffffffffff9a3468]        # 12500 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x13b2>
  66f095:	34 9a ff 
  66f098:	62 f1 fd 48 6f 35 1e 	vmovdqa64 zmm6,ZMMWORD PTR [rip+0xffffffffff9a341e]        # 124c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1372>
  66f09f:	34 9a ff 
  66f0a2:	41 c1 ef 04          	shr    r15d,0x4
  66f0a6:	41 c1 ec 04          	shr    r12d,0x4
  66f0aa:	4d 39 fc             	cmp    r12,r15
  66f0ad:	4d 0f 42 fc          	cmovb  r15,r12
  66f0b1:	4d 85 ff             	test   r15,r15
  66f0b4:	0f 84 96 03 00 00    	je     66f450 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4f40>
  66f0ba:	48 8b 44 24 60       	mov    rax,QWORD PTR [rsp+0x60]
  66f0bf:	4c 01 e8             	add    rax,r13
  66f0c2:	48 89 44 24 18       	mov    QWORD PTR [rsp+0x18],rax
  66f0c7:	4c 03 6c 24 30       	add    r13,QWORD PTR [rsp+0x30]
  66f0cc:	4c 89 6c 24 20       	mov    QWORD PTR [rsp+0x20],r13
  66f0d1:	48 8b 0d 60 bb 09 00 	mov    rcx,QWORD PTR [rip+0x9bb60]        # 70ac38 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1008>
  66f0d8:	48 03 0d 99 bb 09 00 	add    rcx,QWORD PTR [rip+0x9bb99]        # 70ac78 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1048>
  66f0df:	48 8b 54 24 78       	mov    rdx,QWORD PTR [rsp+0x78]
  66f0e4:	48 8b b4 24 90 00 00 	mov    rsi,QWORD PTR [rsp+0x90]
  66f0eb:	00 
  66f0ec:	31 ff                	xor    edi,edi
  66f0ee:	eb 36                	jmp    66f126 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4c16>
  66f0f0:	48 c1 e7 07          	shl    rdi,0x7
  66f0f4:	62 f2 75 48 cf cb    	vgf2p8mulb zmm1,zmm1,zmm3
  66f0fa:	62 f1 fd 48 7f 4c 3c 	vmovdqa64 ZMMWORD PTR [rsp+rdi*1+0x1ac0],zmm1
  66f101:	6b 
  66f102:	62 f2 7d 48 cf c2    	vgf2p8mulb zmm0,zmm0,zmm2
  66f108:	62 f1 fd 48 7f 44 3c 	vmovdqa64 ZMMWORD PTR [rsp+rdi*1+0x1b00],zmm0
  66f10f:	6c 
  66f110:	48 83 c6 10          	add    rsi,0x10
  66f114:	48 83 c2 10          	add    rdx,0x10
  66f118:	4c 89 c7             	mov    rdi,r8
  66f11b:	4c 3b 44 24 40       	cmp    r8,QWORD PTR [rsp+0x40]
  66f120:	0f 84 2a 03 00 00    	je     66f450 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4f40>
  66f126:	4c 8d 47 01          	lea    r8,[rdi+0x1]
  66f12a:	49 89 fa             	mov    r10,rdi
  66f12d:	49 c1 e2 04          	shl    r10,0x4
  66f131:	48 8b 44 24 20       	mov    rax,QWORD PTR [rsp+0x20]
  66f136:	4e 8b 0c 10          	mov    r9,QWORD PTR [rax+r10*1]
  66f13a:	4c 8b 5c 24 18       	mov    r11,QWORD PTR [rsp+0x18]
  66f13f:	c4 81 78 10 04 13    	vmovups xmm0,XMMWORD PTR [r11+r10*1]
  66f145:	47 8b 1c 13          	mov    r11d,DWORD PTR [r11+r10*1]
  66f149:	c4 a1 78 54 04 10    	vandps xmm0,xmm0,XMMWORD PTR [rax+r10*1]
  66f14f:	c4 a1 78 11 84 14 c0 	vmovups XMMWORD PTR [rsp+r10*1+0x12c0],xmm0
  66f156:	12 00 00 
  66f159:	45 0f b6 d3          	movzx  r10d,r11b
  66f15d:	41 c1 e2 07          	shl    r10d,0x7
  66f161:	62 b1 fe 48 6f 0c 11 	vmovdqu64 zmm1,ZMMWORD PTR [rcx+r10*1]
  66f168:	62 b1 fe 48 6f 44 11 	vmovdqu64 zmm0,ZMMWORD PTR [rcx+r10*1+0x40]
  66f16f:	01 
  66f170:	41 ba 01 00 00 00    	mov    r10d,0x1
  66f176:	41 bb 08 00 00 00    	mov    r11d,0x8
  66f17c:	eb 3c                	jmp    66f1ba <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4caa>
  66f17e:	62 f2 cd 48 36 d2    	vpermq zmm2,zmm6,zmm2
  66f184:	49 83 f4 40          	xor    r12,0x40
  66f188:	62 91 fe 48 6f 1c 27 	vmovdqu64 zmm3,ZMMWORD PTR [r15+r12*1]
  66f18f:	62 f3 e5 48 43 db 4e 	vshufi64x2 zmm3,zmm3,zmm3,0x4e
  66f196:	62 f1 6d 48 ef c9    	vpxord zmm1,zmm2,zmm1
  66f19c:	62 f1 7d 48 70 d3 4e 	vpshufd zmm2,zmm3,0x4e
  66f1a3:	62 f1 6d 48 ef c0    	vpxord zmm0,zmm2,zmm0
  66f1a9:	49 ff c2             	inc    r10
  66f1ac:	49 83 c3 08          	add    r11,0x8
  66f1b0:	49 83 fa 10          	cmp    r10,0x10
  66f1b4:	0f 84 16 01 00 00    	je     66f2d0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4dc0>
  66f1ba:	46 0f b6 3c 16       	movzx  r15d,BYTE PTR [rsi+r10*1]
  66f1bf:	41 c1 e7 07          	shl    r15d,0x7
  66f1c3:	49 01 cf             	add    r15,rcx
  66f1c6:	45 89 d5             	mov    r13d,r10d
  66f1c9:	41 d1 ed             	shr    r13d,1
  66f1cc:	41 83 e5 03          	and    r13d,0x3
  66f1d0:	4d 89 dc             	mov    r12,r11
  66f1d3:	49 21 dc             	and    r12,rbx
  66f1d6:	62 91 fe 48 6f 14 3c 	vmovdqu64 zmm2,ZMMWORD PTR [r12+r15*1]
  66f1dd:	41 f6 c2 01          	test   r10b,0x1
  66f1e1:	75 2d                	jne    66f210 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4d00>
  66f1e3:	48 8d 05 d6 35 9a ff 	lea    rax,[rip+0xffffffffff9a35d6]        # 127c0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1672>
  66f1ea:	4e 63 2c a8          	movsxd r13,DWORD PTR [rax+r13*4]
  66f1ee:	49 01 c5             	add    r13,rax
  66f1f1:	41 ff e5             	jmp    r13
  66f1f4:	49 83 f4 40          	xor    r12,0x40
  66f1f8:	62 91 fe 48 6f 1c 27 	vmovdqu64 zmm3,ZMMWORD PTR [r15+r12*1]
  66f1ff:	e9 93 00 00 00       	jmp    66f297 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4d87>
  66f204:	66 66 66 2e 0f 1f 84 	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  66f20b:	00 00 00 00 00 
  66f210:	4f 63 2c ae          	movsxd r13,DWORD PTR [r14+r13*4]
  66f214:	4d 01 f5             	add    r13,r14
  66f217:	41 ff e5             	jmp    r13
  66f21a:	62 f1 7d 48 70 d2 4e 	vpshufd zmm2,zmm2,0x4e
  66f221:	49 83 f4 40          	xor    r12,0x40
  66f225:	62 91 fe 48 6f 1c 27 	vmovdqu64 zmm3,ZMMWORD PTR [r15+r12*1]
  66f22c:	e9 65 ff ff ff       	jmp    66f196 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4c86>
  66f231:	62 f3 ed 48 43 d2 4e 	vshufi64x2 zmm2,zmm2,zmm2,0x4e
  66f238:	49 83 f4 40          	xor    r12,0x40
  66f23c:	62 91 fe 48 6f 1c 27 	vmovdqu64 zmm3,ZMMWORD PTR [r15+r12*1]
  66f243:	62 f3 e5 48 43 db 4e 	vshufi64x2 zmm3,zmm3,zmm3,0x4e
  66f24a:	eb 4b                	jmp    66f297 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4d87>
  66f24c:	62 f3 ed 48 43 d2 1b 	vshufi64x2 zmm2,zmm2,zmm2,0x1b
  66f253:	49 83 f4 40          	xor    r12,0x40
  66f257:	62 91 fe 48 6f 1c 27 	vmovdqu64 zmm3,ZMMWORD PTR [r15+r12*1]
  66f25e:	62 f3 e5 48 43 db 1b 	vshufi64x2 zmm3,zmm3,zmm3,0x1b
  66f265:	eb 30                	jmp    66f297 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4d87>
  66f267:	62 f2 c5 48 36 d2    	vpermq zmm2,zmm7,zmm2
  66f26d:	49 83 f4 40          	xor    r12,0x40
  66f271:	62 91 fe 48 6f 1c 27 	vmovdqu64 zmm3,ZMMWORD PTR [r15+r12*1]
  66f278:	62 f3 e5 48 43 db 1b 	vshufi64x2 zmm3,zmm3,zmm3,0x1b
  66f27f:	e9 12 ff ff ff       	jmp    66f196 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4c86>
  66f284:	62 f3 fd 48 00 d2 4e 	vpermq zmm2,zmm2,0x4e
  66f28b:	49 83 f4 40          	xor    r12,0x40
  66f28f:	62 93 fd 48 00 1c 27 	vpermq zmm3,ZMMWORD PTR [r15+r12*1],0x4e
  66f296:	4e 
  66f297:	62 f1 75 48 ef ca    	vpxord zmm1,zmm1,zmm2
  66f29d:	62 f1 7d 48 ef c3    	vpxord zmm0,zmm0,zmm3
  66f2a3:	49 ff c2             	inc    r10
  66f2a6:	49 83 c3 08          	add    r11,0x8
  66f2aa:	49 83 fa 10          	cmp    r10,0x10
  66f2ae:	0f 85 06 ff ff ff    	jne    66f1ba <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4caa>
  66f2b4:	eb 1a                	jmp    66f2d0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4dc0>
  66f2b6:	62 f3 fd 48 00 d2 1b 	vpermq zmm2,zmm2,0x1b
  66f2bd:	49 83 f4 40          	xor    r12,0x40
  66f2c1:	62 93 fd 48 00 1c 27 	vpermq zmm3,ZMMWORD PTR [r15+r12*1],0x4e
  66f2c8:	4e 
  66f2c9:	e9 c8 fe ff ff       	jmp    66f196 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4c86>
  66f2ce:	66 90                	xchg   ax,ax
  66f2d0:	45 0f b6 c9          	movzx  r9d,r9b
  66f2d4:	41 c1 e1 07          	shl    r9d,0x7
  66f2d8:	62 b1 fe 48 6f 1c 09 	vmovdqu64 zmm3,ZMMWORD PTR [rcx+r9*1]
  66f2df:	62 b1 fe 48 6f 54 09 	vmovdqu64 zmm2,ZMMWORD PTR [rcx+r9*1+0x40]
  66f2e6:	01 
  66f2e7:	41 b9 01 00 00 00    	mov    r9d,0x1
  66f2ed:	41 ba 08 00 00 00    	mov    r10d,0x8
  66f2f3:	eb 3c                	jmp    66f331 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4e21>
  66f2f5:	62 f2 cd 48 36 e4    	vpermq zmm4,zmm6,zmm4
  66f2fb:	49 83 f7 40          	xor    r15,0x40
  66f2ff:	62 91 fe 48 6f 2c 3b 	vmovdqu64 zmm5,ZMMWORD PTR [r11+r15*1]
  66f306:	62 f3 d5 48 43 ed 4e 	vshufi64x2 zmm5,zmm5,zmm5,0x4e
  66f30d:	62 f1 5d 48 ef db    	vpxord zmm3,zmm4,zmm3
  66f313:	62 f1 7d 48 70 e5 4e 	vpshufd zmm4,zmm5,0x4e
  66f31a:	62 f1 5d 48 ef d2    	vpxord zmm2,zmm4,zmm2
  66f320:	49 ff c1             	inc    r9
  66f323:	49 83 c2 08          	add    r10,0x8
  66f327:	49 83 f9 10          	cmp    r9,0x10
  66f32b:	0f 84 bf fd ff ff    	je     66f0f0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4be0>
  66f331:	46 0f b6 1c 0a       	movzx  r11d,BYTE PTR [rdx+r9*1]
  66f336:	41 c1 e3 07          	shl    r11d,0x7
  66f33a:	49 01 cb             	add    r11,rcx
  66f33d:	45 89 cc             	mov    r12d,r9d
  66f340:	41 d1 ec             	shr    r12d,1
  66f343:	41 83 e4 03          	and    r12d,0x3
  66f347:	4d 89 d7             	mov    r15,r10
  66f34a:	49 21 df             	and    r15,rbx
  66f34d:	62 91 fe 48 6f 24 1f 	vmovdqu64 zmm4,ZMMWORD PTR [r15+r11*1]
  66f354:	41 f6 c1 01          	test   r9b,0x1
  66f358:	75 26                	jne    66f380 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4e70>
  66f35a:	48 8d 05 7f 34 9a ff 	lea    rax,[rip+0xffffffffff9a347f]        # 127e0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1692>
  66f361:	4e 63 24 a0          	movsxd r12,DWORD PTR [rax+r12*4]
  66f365:	49 01 c4             	add    r12,rax
  66f368:	41 ff e4             	jmp    r12
  66f36b:	49 83 f7 40          	xor    r15,0x40
  66f36f:	62 91 fe 48 6f 2c 3b 	vmovdqu64 zmm5,ZMMWORD PTR [r11+r15*1]
  66f376:	e9 93 00 00 00       	jmp    66f40e <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4efe>
  66f37b:	0f 1f 44 00 00       	nop    DWORD PTR [rax+rax*1+0x0]
  66f380:	48 8d 05 49 34 9a ff 	lea    rax,[rip+0xffffffffff9a3449]        # 127d0 <_RNvCs8nGewTAJRlY_4itoa13DECIMAL_PAIRS+0x1682>
  66f387:	4e 63 24 a0          	movsxd r12,DWORD PTR [rax+r12*4]
  66f38b:	49 01 c4             	add    r12,rax
  66f38e:	41 ff e4             	jmp    r12
  66f391:	62 f1 7d 48 70 e4 4e 	vpshufd zmm4,zmm4,0x4e
  66f398:	49 83 f7 40          	xor    r15,0x40
  66f39c:	62 91 fe 48 6f 2c 3b 	vmovdqu64 zmm5,ZMMWORD PTR [r11+r15*1]
  66f3a3:	e9 65 ff ff ff       	jmp    66f30d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4dfd>
  66f3a8:	62 f3 dd 48 43 e4 4e 	vshufi64x2 zmm4,zmm4,zmm4,0x4e
  66f3af:	49 83 f7 40          	xor    r15,0x40
  66f3b3:	62 91 fe 48 6f 2c 3b 	vmovdqu64 zmm5,ZMMWORD PTR [r11+r15*1]
  66f3ba:	62 f3 d5 48 43 ed 4e 	vshufi64x2 zmm5,zmm5,zmm5,0x4e
  66f3c1:	eb 4b                	jmp    66f40e <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4efe>
  66f3c3:	62 f3 dd 48 43 e4 1b 	vshufi64x2 zmm4,zmm4,zmm4,0x1b
  66f3ca:	49 83 f7 40          	xor    r15,0x40
  66f3ce:	62 91 fe 48 6f 2c 3b 	vmovdqu64 zmm5,ZMMWORD PTR [r11+r15*1]
  66f3d5:	62 f3 d5 48 43 ed 1b 	vshufi64x2 zmm5,zmm5,zmm5,0x1b
  66f3dc:	eb 30                	jmp    66f40e <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4efe>
  66f3de:	62 f2 c5 48 36 e4    	vpermq zmm4,zmm7,zmm4
  66f3e4:	49 83 f7 40          	xor    r15,0x40
  66f3e8:	62 91 fe 48 6f 2c 3b 	vmovdqu64 zmm5,ZMMWORD PTR [r11+r15*1]
  66f3ef:	62 f3 d5 48 43 ed 1b 	vshufi64x2 zmm5,zmm5,zmm5,0x1b
  66f3f6:	e9 12 ff ff ff       	jmp    66f30d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4dfd>
  66f3fb:	62 f3 fd 48 00 e4 4e 	vpermq zmm4,zmm4,0x4e
  66f402:	49 83 f7 40          	xor    r15,0x40
  66f406:	62 93 fd 48 00 2c 3b 	vpermq zmm5,ZMMWORD PTR [r11+r15*1],0x4e
  66f40d:	4e 
  66f40e:	62 f1 65 48 ef dc    	vpxord zmm3,zmm3,zmm4
  66f414:	62 f1 6d 48 ef d5    	vpxord zmm2,zmm2,zmm5
  66f41a:	49 ff c1             	inc    r9
  66f41d:	49 83 c2 08          	add    r10,0x8
  66f421:	49 83 f9 10          	cmp    r9,0x10
  66f425:	0f 85 06 ff ff ff    	jne    66f331 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4e21>
  66f42b:	e9 c0 fc ff ff       	jmp    66f0f0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4be0>
  66f430:	62 f3 fd 48 00 e4 1b 	vpermq zmm4,zmm4,0x1b
  66f437:	49 83 f7 40          	xor    r15,0x40
  66f43b:	62 93 fd 48 00 2c 3b 	vpermq zmm5,ZMMWORD PTR [r11+r15*1],0x4e
  66f442:	4e 
  66f443:	e9 c5 fe ff ff       	jmp    66f30d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4dfd>
  66f448:	0f 1f 84 00 00 00 00 	nop    DWORD PTR [rax+rax*1+0x0]
  66f44f:	00 
  66f450:	48 ff 44 24 58       	inc    QWORD PTR [rsp+0x58]
  66f455:	48 c7 84 24 08 02 00 	mov    QWORD PTR [rsp+0x208],0x2
  66f45c:	00 02 00 00 00 
  66f461:	48 c7 84 24 10 02 00 	mov    QWORD PTR [rsp+0x210],0x0
  66f468:	00 00 00 00 00 
  66f46d:	48 c7 84 24 18 02 00 	mov    QWORD PTR [rsp+0x218],0x8
  66f474:	00 08 00 00 00 
  66f479:	31 d2                	xor    edx,edx
  66f47b:	31 c0                	xor    eax,eax
  66f47d:	e9 90 00 00 00       	jmp    66f512 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5002>
  66f482:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  66f489:	1f 84 00 00 00 00 00 
  66f490:	62 71 fe 48 7f 79 01 	vmovdqu64 ZMMWORD PTR [rcx+0x40],zmm15
  66f497:	62 71 fe 48 7f 71 03 	vmovdqu64 ZMMWORD PTR [rcx+0xc0],zmm14
  66f49e:	62 71 fe 48 7f 6a 01 	vmovdqu64 ZMMWORD PTR [rdx+0x40],zmm13
  66f4a5:	62 71 fe 48 7f 62 03 	vmovdqu64 ZMMWORD PTR [rdx+0xc0],zmm12
  66f4ac:	62 71 fe 48 7f 5e 01 	vmovdqu64 ZMMWORD PTR [rsi+0x40],zmm11
  66f4b3:	62 71 fe 48 7f 56 03 	vmovdqu64 ZMMWORD PTR [rsi+0xc0],zmm10
  66f4ba:	62 51 fe 48 7f 48 01 	vmovdqu64 ZMMWORD PTR [r8+0x40],zmm9
  66f4c1:	62 51 fe 48 7f 40 03 	vmovdqu64 ZMMWORD PTR [r8+0xc0],zmm8
  66f4c8:	62 d1 fe 48 7f 79 01 	vmovdqu64 ZMMWORD PTR [r9+0x40],zmm7
  66f4cf:	62 d1 fe 48 7f 71 03 	vmovdqu64 ZMMWORD PTR [r9+0xc0],zmm6
  66f4d6:	62 d1 fe 48 7f 6a 01 	vmovdqu64 ZMMWORD PTR [r10+0x40],zmm5
  66f4dd:	62 d1 fe 48 7f 62 03 	vmovdqu64 ZMMWORD PTR [r10+0xc0],zmm4
  66f4e4:	62 f1 fe 48 7f 58 01 	vmovdqu64 ZMMWORD PTR [rax+0x40],zmm3
  66f4eb:	62 f1 fe 48 7f 50 03 	vmovdqu64 ZMMWORD PTR [rax+0xc0],zmm2
  66f4f2:	62 d1 fe 48 7f 4b 01 	vmovdqu64 ZMMWORD PTR [r11+0x40],zmm1
  66f4f9:	62 d1 fe 48 7f 43 03 	vmovdqu64 ZMMWORD PTR [r11+0xc0],zmm0
  66f500:	b8 01 00 00 00       	mov    eax,0x1
  66f505:	f6 44 24 40 01       	test   BYTE PTR [rsp+0x40],0x1
  66f50a:	b2 01                	mov    dl,0x1
  66f50c:	0f 85 4e fa ff ff    	jne    66ef60 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4a50>
  66f512:	48 8b bc c4 10 02 00 	mov    rdi,QWORD PTR [rsp+rax*8+0x210]
  66f519:	00 
  66f51a:	b8 10 00 00 00       	mov    eax,0x10
  66f51f:	48 29 f8             	sub    rax,rdi
  66f522:	b9 00 00 00 00       	mov    ecx,0x0
  66f527:	48 0f 43 c8          	cmovae rcx,rax
  66f52b:	48 83 ff 10          	cmp    rdi,0x10
  66f52f:	0f 83 02 0c 00 00    	jae    670137 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5c27>
  66f535:	4c 8d 47 01          	lea    r8,[rdi+0x1]
  66f539:	48 83 f9 01          	cmp    rcx,0x1
  66f53d:	0f 84 61 0b 00 00    	je     6700a4 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5b94>
  66f543:	48 8d 77 02          	lea    rsi,[rdi+0x2]
  66f547:	48 83 f9 02          	cmp    rcx,0x2
  66f54b:	0f 84 23 0b 00 00    	je     670074 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5b64>
  66f551:	4c 8d 4f 03          	lea    r9,[rdi+0x3]
  66f555:	48 83 f9 03          	cmp    rcx,0x3
  66f559:	0f 84 cd 0a 00 00    	je     67002c <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5b1c>
  66f55f:	4c 8d 57 04          	lea    r10,[rdi+0x4]
  66f563:	48 83 f9 04          	cmp    rcx,0x4
  66f567:	0f 84 c7 0b 00 00    	je     670134 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5c24>
  66f56d:	48 8d 47 05          	lea    rax,[rdi+0x5]
  66f571:	48 83 f9 05          	cmp    rcx,0x5
  66f575:	0f 84 89 0b 00 00    	je     670104 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5bf4>
  66f57b:	4c 8d 5f 06          	lea    r11,[rdi+0x6]
  66f57f:	48 83 f9 06          	cmp    rcx,0x6
  66f583:	0f 84 4b 0b 00 00    	je     6700d4 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5bc4>
  66f589:	4c 8d 7f 07          	lea    r15,[rdi+0x7]
  66f58d:	48 83 f9 07          	cmp    rcx,0x7
  66f591:	0f 84 ad 0a 00 00    	je     670044 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5b34>
  66f597:	48 89 54 24 40       	mov    QWORD PTR [rsp+0x40],rdx
  66f59c:	48 89 fa             	mov    rdx,rdi
  66f59f:	48 c1 e2 08          	shl    rdx,0x8
  66f5a3:	48 8d 8c 14 c0 02 00 	lea    rcx,[rsp+rdx*1+0x2c0]
  66f5aa:	00 
  66f5ab:	62 f1 fe 48 6f 44 14 	vmovdqu64 zmm0,ZMMWORD PTR [rsp+rdx*1+0x2c0]
  66f5b2:	0b 
  66f5b3:	49 c1 e0 08          	shl    r8,0x8
  66f5b7:	4a 8d 94 04 c0 02 00 	lea    rdx,[rsp+r8*1+0x2c0]
  66f5be:	00 
  66f5bf:	62 b1 fe 48 6f 4c 04 	vmovdqu64 zmm1,ZMMWORD PTR [rsp+r8*1+0x2c0]
  66f5c6:	0b 
  66f5c7:	48 c1 e6 08          	shl    rsi,0x8
  66f5cb:	62 f1 fe 48 6f 54 34 	vmovdqu64 zmm2,ZMMWORD PTR [rsp+rsi*1+0x2c0]
  66f5d2:	0b 
  66f5d3:	48 8d b4 34 c0 02 00 	lea    rsi,[rsp+rsi*1+0x2c0]
  66f5da:	00 
  66f5db:	49 c1 e1 08          	shl    r9,0x8
  66f5df:	4e 8d 84 0c c0 02 00 	lea    r8,[rsp+r9*1+0x2c0]
  66f5e6:	00 
  66f5e7:	62 b1 fe 48 6f 5c 0c 	vmovdqu64 zmm3,ZMMWORD PTR [rsp+r9*1+0x2c0]
  66f5ee:	0b 
  66f5ef:	49 c1 e2 08          	shl    r10,0x8
  66f5f3:	4e 8d 8c 14 c0 02 00 	lea    r9,[rsp+r10*1+0x2c0]
  66f5fa:	00 
  66f5fb:	62 b1 fe 48 6f 64 14 	vmovdqu64 zmm4,ZMMWORD PTR [rsp+r10*1+0x2c0]
  66f602:	0b 
  66f603:	48 c1 e0 08          	shl    rax,0x8
  66f607:	62 f1 fe 48 6f 6c 04 	vmovdqu64 zmm5,ZMMWORD PTR [rsp+rax*1+0x2c0]
  66f60e:	0b 
  66f60f:	49 c1 e3 08          	shl    r11,0x8
  66f613:	49 c1 e7 08          	shl    r15,0x8
  66f617:	62 31 fe 48 6f 4c 1c 	vmovdqu64 zmm9,ZMMWORD PTR [rsp+r11*1+0x2c0]
  66f61e:	0b 
  66f61f:	62 31 fe 48 6f 44 1c 	vmovdqu64 zmm8,ZMMWORD PTR [rsp+r11*1+0x340]
  66f626:	0d 
  66f627:	62 b1 fe 48 6f 7c 3c 	vmovdqu64 zmm7,ZMMWORD PTR [rsp+r15*1+0x2c0]
  66f62e:	0b 
  66f62f:	62 b1 fe 48 6f 74 3c 	vmovdqu64 zmm6,ZMMWORD PTR [rsp+r15*1+0x340]
  66f636:	0d 
  66f637:	62 71 fe 48 6f 79 02 	vmovdqu64 zmm15,ZMMWORD PTR [rcx+0x80]
  66f63e:	62 71 fe 48 6f 72 02 	vmovdqu64 zmm14,ZMMWORD PTR [rdx+0x80]
  66f645:	62 71 fe 48 6f 6e 02 	vmovdqu64 zmm13,ZMMWORD PTR [rsi+0x80]
  66f64c:	62 51 fe 48 6f 60 02 	vmovdqu64 zmm12,ZMMWORD PTR [r8+0x80]
  66f653:	4c 8d 94 04 c0 02 00 	lea    r10,[rsp+rax*1+0x2c0]
  66f65a:	00 
  66f65b:	62 51 fe 48 6f 59 02 	vmovdqu64 zmm11,ZMMWORD PTR [r9+0x80]
  66f662:	62 51 fe 48 6f 52 02 	vmovdqu64 zmm10,ZMMWORD PTR [r10+0x80]
  66f669:	48 8b 84 24 a0 00 00 	mov    rax,QWORD PTR [rsp+0xa0]
  66f670:	00 
  66f671:	48 39 84 24 c0 00 00 	cmp    QWORD PTR [rsp+0xc0],rax
  66f678:	00 
  66f679:	0f 84 3f 01 00 00    	je     66f7be <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x52ae>
  66f67f:	48 8d 84 24 c7 16 00 	lea    rax,[rsp+0x16c7]
  66f686:	00 
  66f687:	48 01 f8             	add    rax,rdi
  66f68a:	4c 8b a4 24 40 01 00 	mov    r12,QWORD PTR [rsp+0x140]
  66f691:	00 
  66f692:	45 31 ed             	xor    r13d,r13d
  66f695:	66 66 2e 0f 1f 84 00 	data16 cs nop WORD PTR [rax+rax*1+0x0]
  66f69c:	00 00 00 00 
  66f6a0:	62 a2 7d 48 78 44 28 	vpbroadcastb zmm16,BYTE PTR [rax+r13*1-0x7]
  66f6a7:	f9 
  66f6a8:	c4 a1 f8 90 8c 2c c0 	kmovq  k1,QWORD PTR [rsp+r13*1+0x12c0]
  66f6af:	12 00 00 
  66f6b2:	62 a1 7f c9 6f c8    	vmovdqu8 zmm17{k1}{z},zmm16
  66f6b8:	62 b1 7d 48 ef c1    	vpxord zmm0,zmm0,zmm17
  66f6be:	62 a1 fd 48 6f 4c ec 	vmovdqa64 zmm17,ZMMWORD PTR [rsp+r13*8+0x1ac0]
  66f6c5:	6b 
  66f6c6:	62 a2 75 40 cf c0    	vgf2p8mulb zmm16,zmm17,zmm16
  66f6cc:	62 31 05 48 ef f8    	vpxord zmm15,zmm15,zmm16
  66f6d2:	62 a2 7d 48 78 44 28 	vpbroadcastb zmm16,BYTE PTR [rax+r13*1-0x6]
  66f6d9:	fa 
  66f6da:	62 a1 7f c9 6f d0    	vmovdqu8 zmm18{k1}{z},zmm16
  66f6e0:	62 b1 75 48 ef ca    	vpxord zmm1,zmm1,zmm18
  66f6e6:	62 a2 75 40 cf c0    	vgf2p8mulb zmm16,zmm17,zmm16
  66f6ec:	62 31 0d 48 ef f0    	vpxord zmm14,zmm14,zmm16
  66f6f2:	62 a2 7d 48 78 44 28 	vpbroadcastb zmm16,BYTE PTR [rax+r13*1-0x5]
  66f6f9:	fb 
  66f6fa:	62 a1 7f c9 6f d0    	vmovdqu8 zmm18{k1}{z},zmm16
  66f700:	62 b1 6d 48 ef d2    	vpxord zmm2,zmm2,zmm18
  66f706:	62 a2 75 40 cf c0    	vgf2p8mulb zmm16,zmm17,zmm16
  66f70c:	62 31 15 48 ef e8    	vpxord zmm13,zmm13,zmm16
  66f712:	62 a2 7d 48 78 44 28 	vpbroadcastb zmm16,BYTE PTR [rax+r13*1-0x4]
  66f719:	fc 
  66f71a:	62 a1 7f c9 6f d0    	vmovdqu8 zmm18{k1}{z},zmm16
  66f720:	62 b1 65 48 ef da    	vpxord zmm3,zmm3,zmm18
  66f726:	62 a2 75 40 cf c0    	vgf2p8mulb zmm16,zmm17,zmm16
  66f72c:	62 31 1d 48 ef e0    	vpxord zmm12,zmm12,zmm16
  66f732:	62 a2 7d 48 78 44 28 	vpbroadcastb zmm16,BYTE PTR [rax+r13*1-0x3]
  66f739:	fd 
  66f73a:	62 a1 7f c9 6f d0    	vmovdqu8 zmm18{k1}{z},zmm16
  66f740:	62 b1 5d 48 ef e2    	vpxord zmm4,zmm4,zmm18
  66f746:	62 a2 75 40 cf c0    	vgf2p8mulb zmm16,zmm17,zmm16
  66f74c:	62 31 25 48 ef d8    	vpxord zmm11,zmm11,zmm16
  66f752:	62 a2 7d 48 78 44 28 	vpbroadcastb zmm16,BYTE PTR [rax+r13*1-0x2]
  66f759:	fe 
  66f75a:	62 a1 7f c9 6f d0    	vmovdqu8 zmm18{k1}{z},zmm16
  66f760:	62 b1 55 48 ef ea    	vpxord zmm5,zmm5,zmm18
  66f766:	62 a2 75 40 cf c0    	vgf2p8mulb zmm16,zmm17,zmm16
  66f76c:	62 31 2d 48 ef d0    	vpxord zmm10,zmm10,zmm16
  66f772:	62 a2 7d 48 78 44 28 	vpbroadcastb zmm16,BYTE PTR [rax+r13*1-0x1]
  66f779:	ff 
  66f77a:	62 a1 7f c9 6f d0    	vmovdqu8 zmm18{k1}{z},zmm16
  66f780:	62 31 35 48 ef ca    	vpxord zmm9,zmm9,zmm18
  66f786:	62 a2 75 40 cf c0    	vgf2p8mulb zmm16,zmm17,zmm16
  66f78c:	62 31 3d 48 ef c0    	vpxord zmm8,zmm8,zmm16
  66f792:	62 a2 7d 48 78 04 28 	vpbroadcastb zmm16,BYTE PTR [rax+r13*1]
  66f799:	62 a1 7f c9 6f d0    	vmovdqu8 zmm18{k1}{z},zmm16
  66f79f:	62 b1 45 48 ef fa    	vpxord zmm7,zmm7,zmm18
  66f7a5:	62 a2 75 40 cf c0    	vgf2p8mulb zmm16,zmm17,zmm16
  66f7ab:	62 b1 4d 48 ef f0    	vpxord zmm6,zmm6,zmm16
  66f7b1:	49 83 c5 10          	add    r13,0x10
  66f7b5:	49 ff cc             	dec    r12
  66f7b8:	0f 85 e2 fe ff ff    	jne    66f6a0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5190>
  66f7be:	4a 8d 84 1c c0 02 00 	lea    rax,[rsp+r11*1+0x2c0]
  66f7c5:	00 
  66f7c6:	4e 8d 9c 3c c0 02 00 	lea    r11,[rsp+r15*1+0x2c0]
  66f7cd:	00 
  66f7ce:	62 f1 fe 48 7f 01    	vmovdqu64 ZMMWORD PTR [rcx],zmm0
  66f7d4:	62 71 fe 48 7f 79 02 	vmovdqu64 ZMMWORD PTR [rcx+0x80],zmm15
  66f7db:	62 f1 fe 48 7f 0a    	vmovdqu64 ZMMWORD PTR [rdx],zmm1
  66f7e1:	62 71 fe 48 7f 72 02 	vmovdqu64 ZMMWORD PTR [rdx+0x80],zmm14
  66f7e8:	62 f1 fe 48 7f 16    	vmovdqu64 ZMMWORD PTR [rsi],zmm2
  66f7ee:	62 71 fe 48 7f 6e 02 	vmovdqu64 ZMMWORD PTR [rsi+0x80],zmm13
  66f7f5:	62 d1 fe 48 7f 18    	vmovdqu64 ZMMWORD PTR [r8],zmm3
  66f7fb:	62 51 fe 48 7f 60 02 	vmovdqu64 ZMMWORD PTR [r8+0x80],zmm12
  66f802:	62 d1 fe 48 7f 21    	vmovdqu64 ZMMWORD PTR [r9],zmm4
  66f808:	62 51 fe 48 7f 59 02 	vmovdqu64 ZMMWORD PTR [r9+0x80],zmm11
  66f80f:	62 d1 fe 48 7f 2a    	vmovdqu64 ZMMWORD PTR [r10],zmm5
  66f815:	62 51 fe 48 7f 52 02 	vmovdqu64 ZMMWORD PTR [r10+0x80],zmm10
  66f81c:	62 71 fe 48 7f 08    	vmovdqu64 ZMMWORD PTR [rax],zmm9
  66f822:	62 71 fe 48 7f 40 02 	vmovdqu64 ZMMWORD PTR [rax+0x80],zmm8
  66f829:	62 d1 fe 48 7f 3b    	vmovdqu64 ZMMWORD PTR [r11],zmm7
  66f82f:	62 d1 fe 48 7f 73 02 	vmovdqu64 ZMMWORD PTR [r11+0x80],zmm6
  66f836:	62 71 fe 48 6f 79 01 	vmovdqu64 zmm15,ZMMWORD PTR [rcx+0x40]
  66f83d:	62 71 fe 48 6f 71 03 	vmovdqu64 zmm14,ZMMWORD PTR [rcx+0xc0]
  66f844:	62 71 fe 48 6f 6a 01 	vmovdqu64 zmm13,ZMMWORD PTR [rdx+0x40]
  66f84b:	62 71 fe 48 6f 62 03 	vmovdqu64 zmm12,ZMMWORD PTR [rdx+0xc0]
  66f852:	62 71 fe 48 6f 5e 01 	vmovdqu64 zmm11,ZMMWORD PTR [rsi+0x40]
  66f859:	62 71 fe 48 6f 56 03 	vmovdqu64 zmm10,ZMMWORD PTR [rsi+0xc0]
  66f860:	62 51 fe 48 6f 48 01 	vmovdqu64 zmm9,ZMMWORD PTR [r8+0x40]
  66f867:	62 51 fe 48 6f 40 03 	vmovdqu64 zmm8,ZMMWORD PTR [r8+0xc0]
  66f86e:	62 d1 fe 48 6f 79 01 	vmovdqu64 zmm7,ZMMWORD PTR [r9+0x40]
  66f875:	62 d1 fe 48 6f 71 03 	vmovdqu64 zmm6,ZMMWORD PTR [r9+0xc0]
  66f87c:	62 d1 fe 48 6f 6a 01 	vmovdqu64 zmm5,ZMMWORD PTR [r10+0x40]
  66f883:	62 d1 fe 48 6f 62 03 	vmovdqu64 zmm4,ZMMWORD PTR [r10+0xc0]
  66f88a:	62 f1 fe 48 6f 58 01 	vmovdqu64 zmm3,ZMMWORD PTR [rax+0x40]
  66f891:	62 f1 fe 48 6f 50 03 	vmovdqu64 zmm2,ZMMWORD PTR [rax+0xc0]
  66f898:	62 d1 fe 48 6f 4b 01 	vmovdqu64 zmm1,ZMMWORD PTR [r11+0x40]
  66f89f:	62 d1 fe 48 6f 43 03 	vmovdqu64 zmm0,ZMMWORD PTR [r11+0xc0]
  66f8a6:	4c 8b bc 24 a0 00 00 	mov    r15,QWORD PTR [rsp+0xa0]
  66f8ad:	00 
  66f8ae:	4c 39 bc 24 c0 00 00 	cmp    QWORD PTR [rsp+0xc0],r15
  66f8b5:	00 
  66f8b6:	0f 84 d4 fb ff ff    	je     66f490 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4f80>
  66f8bc:	4c 8d bc 24 c7 16 00 	lea    r15,[rsp+0x16c7]
  66f8c3:	00 
  66f8c4:	4c 01 ff             	add    rdi,r15
  66f8c7:	4c 8b bc 24 40 01 00 	mov    r15,QWORD PTR [rsp+0x140]
  66f8ce:	00 
  66f8cf:	45 31 e4             	xor    r12d,r12d
  66f8d2:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  66f8d9:	1f 84 00 00 00 00 00 
  66f8e0:	62 a2 7d 48 78 44 27 	vpbroadcastb zmm16,BYTE PTR [rdi+r12*1-0x7]
  66f8e7:	f9 
  66f8e8:	c4 a1 f8 90 8c 24 c8 	kmovq  k1,QWORD PTR [rsp+r12*1+0x12c8]
  66f8ef:	12 00 00 
  66f8f2:	62 a1 7f c9 6f c8    	vmovdqu8 zmm17{k1}{z},zmm16
  66f8f8:	62 31 05 48 ef f9    	vpxord zmm15,zmm15,zmm17
  66f8fe:	62 a1 fd 48 6f 4c e4 	vmovdqa64 zmm17,ZMMWORD PTR [rsp+r12*8+0x1b00]
  66f905:	6c 
  66f906:	62 a2 75 40 cf c0    	vgf2p8mulb zmm16,zmm17,zmm16
  66f90c:	62 31 0d 48 ef f0    	vpxord zmm14,zmm14,zmm16
  66f912:	62 a2 7d 48 78 44 27 	vpbroadcastb zmm16,BYTE PTR [rdi+r12*1-0x6]
  66f919:	fa 
  66f91a:	62 a1 7f c9 6f d0    	vmovdqu8 zmm18{k1}{z},zmm16
  66f920:	62 31 15 48 ef ea    	vpxord zmm13,zmm13,zmm18
  66f926:	62 a2 75 40 cf c0    	vgf2p8mulb zmm16,zmm17,zmm16
  66f92c:	62 31 1d 48 ef e0    	vpxord zmm12,zmm12,zmm16
  66f932:	62 a2 7d 48 78 44 27 	vpbroadcastb zmm16,BYTE PTR [rdi+r12*1-0x5]
  66f939:	fb 
  66f93a:	62 a1 7f c9 6f d0    	vmovdqu8 zmm18{k1}{z},zmm16
  66f940:	62 31 25 48 ef da    	vpxord zmm11,zmm11,zmm18
  66f946:	62 a2 75 40 cf c0    	vgf2p8mulb zmm16,zmm17,zmm16
  66f94c:	62 31 2d 48 ef d0    	vpxord zmm10,zmm10,zmm16
  66f952:	62 a2 7d 48 78 44 27 	vpbroadcastb zmm16,BYTE PTR [rdi+r12*1-0x4]
  66f959:	fc 
  66f95a:	62 a1 7f c9 6f d0    	vmovdqu8 zmm18{k1}{z},zmm16
  66f960:	62 31 35 48 ef ca    	vpxord zmm9,zmm9,zmm18
  66f966:	62 a2 75 40 cf c0    	vgf2p8mulb zmm16,zmm17,zmm16
  66f96c:	62 31 3d 48 ef c0    	vpxord zmm8,zmm8,zmm16
  66f972:	62 a2 7d 48 78 44 27 	vpbroadcastb zmm16,BYTE PTR [rdi+r12*1-0x3]
  66f979:	fd 
  66f97a:	62 a1 7f c9 6f d0    	vmovdqu8 zmm18{k1}{z},zmm16
  66f980:	62 b1 45 48 ef fa    	vpxord zmm7,zmm7,zmm18
  66f986:	62 a2 75 40 cf c0    	vgf2p8mulb zmm16,zmm17,zmm16
  66f98c:	62 b1 4d 48 ef f0    	vpxord zmm6,zmm6,zmm16
  66f992:	62 a2 7d 48 78 44 27 	vpbroadcastb zmm16,BYTE PTR [rdi+r12*1-0x2]
  66f999:	fe 
  66f99a:	62 a1 7f c9 6f d0    	vmovdqu8 zmm18{k1}{z},zmm16
  66f9a0:	62 b1 55 48 ef ea    	vpxord zmm5,zmm5,zmm18
  66f9a6:	62 a2 75 40 cf c0    	vgf2p8mulb zmm16,zmm17,zmm16
  66f9ac:	62 b1 5d 48 ef e0    	vpxord zmm4,zmm4,zmm16
  66f9b2:	62 a2 7d 48 78 44 27 	vpbroadcastb zmm16,BYTE PTR [rdi+r12*1-0x1]
  66f9b9:	ff 
  66f9ba:	62 a1 7f c9 6f d0    	vmovdqu8 zmm18{k1}{z},zmm16
  66f9c0:	62 b1 65 48 ef da    	vpxord zmm3,zmm3,zmm18
  66f9c6:	62 a2 75 40 cf c0    	vgf2p8mulb zmm16,zmm17,zmm16
  66f9cc:	62 b1 6d 48 ef d0    	vpxord zmm2,zmm2,zmm16
  66f9d2:	62 a2 7d 48 78 04 27 	vpbroadcastb zmm16,BYTE PTR [rdi+r12*1]
  66f9d9:	62 a1 7f c9 6f d0    	vmovdqu8 zmm18{k1}{z},zmm16
  66f9df:	62 b1 75 48 ef ca    	vpxord zmm1,zmm1,zmm18
  66f9e5:	62 a2 75 40 cf c0    	vgf2p8mulb zmm16,zmm17,zmm16
  66f9eb:	62 b1 7d 48 ef c0    	vpxord zmm0,zmm0,zmm16
  66f9f1:	49 83 c4 10          	add    r12,0x10
  66f9f5:	49 ff cf             	dec    r15
  66f9f8:	0f 85 e2 fe ff ff    	jne    66f8e0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x53d0>
  66f9fe:	e9 8d fa ff ff       	jmp    66f490 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4f80>
  66fa03:	bf f0 0f 00 00       	mov    edi,0xff0
  66fa08:	c5 f8 77             	vzeroupper 
  66fa0b:	ff 15 77 79 09 00    	call   QWORD PTR [rip+0x97977]        # 707388 <malloc@GLIBC_2.2.5>
  66fa11:	48 85 c0             	test   rax,rax
  66fa14:	0f 84 57 08 00 00    	je     670271 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5d61>
  66fa1a:	b9 0f 00 00 00       	mov    ecx,0xf
  66fa1f:	48 8d 94 24 c0 11 00 	lea    rdx,[rsp+0x11c0]
  66fa26:	00 
  66fa27:	66 0f 1f 84 00 00 00 	nop    WORD PTR [rax+rax*1+0x0]
  66fa2e:	00 00 
  66fa30:	0f b6 b2 00 f1 ff ff 	movzx  esi,BYTE PTR [rdx-0xf00]
  66fa37:	40 88 b4 24 a0 00 00 	mov    BYTE PTR [rsp+0xa0],sil
  66fa3e:	00 
  66fa3f:	0f b6 ba 00 f2 ff ff 	movzx  edi,BYTE PTR [rdx-0xe00]
  66fa46:	44 0f b6 82 00 f3 ff 	movzx  r8d,BYTE PTR [rdx-0xd00]
  66fa4d:	ff 
  66fa4e:	44 0f b6 8a 00 f4 ff 	movzx  r9d,BYTE PTR [rdx-0xc00]
  66fa55:	ff 
  66fa56:	44 0f b6 92 00 f5 ff 	movzx  r10d,BYTE PTR [rdx-0xb00]
  66fa5d:	ff 
  66fa5e:	44 0f b6 9a 00 f6 ff 	movzx  r11d,BYTE PTR [rdx-0xa00]
  66fa65:	ff 
  66fa66:	0f b6 9a 00 f7 ff ff 	movzx  ebx,BYTE PTR [rdx-0x900]
  66fa6d:	44 0f b6 b2 00 f8 ff 	movzx  r14d,BYTE PTR [rdx-0x800]
  66fa74:	ff 
  66fa75:	44 0f b6 ba 00 f9 ff 	movzx  r15d,BYTE PTR [rdx-0x700]
  66fa7c:	ff 
  66fa7d:	44 0f b6 a2 00 fa ff 	movzx  r12d,BYTE PTR [rdx-0x600]
  66fa84:	ff 
  66fa85:	44 0f b6 aa 00 fb ff 	movzx  r13d,BYTE PTR [rdx-0x500]
  66fa8c:	ff 
  66fa8d:	0f b6 b2 00 fc ff ff 	movzx  esi,BYTE PTR [rdx-0x400]
  66fa94:	40 88 b4 24 c0 00 00 	mov    BYTE PTR [rsp+0xc0],sil
  66fa9b:	00 
  66fa9c:	0f b6 b2 00 fd ff ff 	movzx  esi,BYTE PTR [rdx-0x300]
  66faa3:	40 88 74 24 40       	mov    BYTE PTR [rsp+0x40],sil
  66faa8:	0f b6 b2 00 fe ff ff 	movzx  esi,BYTE PTR [rdx-0x200]
  66faaf:	40 88 b4 24 40 01 00 	mov    BYTE PTR [rsp+0x140],sil
  66fab6:	00 
  66fab7:	0f b6 b2 00 ff ff ff 	movzx  esi,BYTE PTR [rdx-0x100]
  66fabe:	40 88 74 24 20       	mov    BYTE PTR [rsp+0x20],sil
  66fac3:	0f b6 32             	movzx  esi,BYTE PTR [rdx]
  66fac6:	40 88 74 24 18       	mov    BYTE PTR [rsp+0x18],sil
  66facb:	0f b6 b4 24 a0 00 00 	movzx  esi,BYTE PTR [rsp+0xa0]
  66fad2:	00 
  66fad3:	40 88 74 08 f1       	mov    BYTE PTR [rax+rcx*1-0xf],sil
  66fad8:	40 88 7c 08 f2       	mov    BYTE PTR [rax+rcx*1-0xe],dil
  66fadd:	44 88 44 08 f3       	mov    BYTE PTR [rax+rcx*1-0xd],r8b
  66fae2:	44 88 4c 08 f4       	mov    BYTE PTR [rax+rcx*1-0xc],r9b
  66fae7:	44 88 54 08 f5       	mov    BYTE PTR [rax+rcx*1-0xb],r10b
  66faec:	44 88 5c 08 f6       	mov    BYTE PTR [rax+rcx*1-0xa],r11b
  66faf1:	88 5c 08 f7          	mov    BYTE PTR [rax+rcx*1-0x9],bl
  66faf5:	44 88 74 08 f8       	mov    BYTE PTR [rax+rcx*1-0x8],r14b
  66fafa:	44 88 7c 08 f9       	mov    BYTE PTR [rax+rcx*1-0x7],r15b
  66faff:	44 88 64 08 fa       	mov    BYTE PTR [rax+rcx*1-0x6],r12b
  66fb04:	44 88 6c 08 fb       	mov    BYTE PTR [rax+rcx*1-0x5],r13b
  66fb09:	0f b6 b4 24 c0 00 00 	movzx  esi,BYTE PTR [rsp+0xc0]
  66fb10:	00 
  66fb11:	40 88 74 08 fc       	mov    BYTE PTR [rax+rcx*1-0x4],sil
  66fb16:	0f b6 74 24 40       	movzx  esi,BYTE PTR [rsp+0x40]
  66fb1b:	40 88 74 08 fd       	mov    BYTE PTR [rax+rcx*1-0x3],sil
  66fb20:	0f b6 b4 24 40 01 00 	movzx  esi,BYTE PTR [rsp+0x140]
  66fb27:	00 
  66fb28:	40 88 74 08 fe       	mov    BYTE PTR [rax+rcx*1-0x2],sil
  66fb2d:	0f b6 74 24 20       	movzx  esi,BYTE PTR [rsp+0x20]
  66fb32:	40 88 74 08 ff       	mov    BYTE PTR [rax+rcx*1-0x1],sil
  66fb37:	0f b6 74 24 18       	movzx  esi,BYTE PTR [rsp+0x18]
  66fb3c:	40 88 34 08          	mov    BYTE PTR [rax+rcx*1],sil
  66fb40:	48 83 c1 10          	add    rcx,0x10
  66fb44:	48 ff c2             	inc    rdx
  66fb47:	48 81 f9 ff 0f 00 00 	cmp    rcx,0xfff
  66fb4e:	0f 85 dc fe ff ff    	jne    66fa30 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5520>
  66fb54:	e9 9b 02 00 00       	jmp    66fdf4 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x58e4>
  66fb59:	bf f0 0f 00 00       	mov    edi,0xff0
  66fb5e:	c5 f8 77             	vzeroupper 
  66fb61:	ff 15 21 78 09 00    	call   QWORD PTR [rip+0x97821]        # 707388 <malloc@GLIBC_2.2.5>
  66fb67:	48 85 c0             	test   rax,rax
  66fb6a:	0f 84 11 07 00 00    	je     670281 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5d71>
  66fb70:	b9 0f 00 00 00       	mov    ecx,0xf
  66fb75:	48 8d 94 24 c0 11 00 	lea    rdx,[rsp+0x11c0]
  66fb7c:	00 
  66fb7d:	0f 1f 00             	nop    DWORD PTR [rax]
  66fb80:	0f b6 b2 00 f1 ff ff 	movzx  esi,BYTE PTR [rdx-0xf00]
  66fb87:	40 88 b4 24 a0 00 00 	mov    BYTE PTR [rsp+0xa0],sil
  66fb8e:	00 
  66fb8f:	0f b6 ba 00 f2 ff ff 	movzx  edi,BYTE PTR [rdx-0xe00]
  66fb96:	44 0f b6 82 00 f3 ff 	movzx  r8d,BYTE PTR [rdx-0xd00]
  66fb9d:	ff 
  66fb9e:	44 0f b6 8a 00 f4 ff 	movzx  r9d,BYTE PTR [rdx-0xc00]
  66fba5:	ff 
  66fba6:	44 0f b6 92 00 f5 ff 	movzx  r10d,BYTE PTR [rdx-0xb00]
  66fbad:	ff 
  66fbae:	44 0f b6 9a 00 f6 ff 	movzx  r11d,BYTE PTR [rdx-0xa00]
  66fbb5:	ff 
  66fbb6:	0f b6 9a 00 f7 ff ff 	movzx  ebx,BYTE PTR [rdx-0x900]
  66fbbd:	44 0f b6 b2 00 f8 ff 	movzx  r14d,BYTE PTR [rdx-0x800]
  66fbc4:	ff 
  66fbc5:	44 0f b6 ba 00 f9 ff 	movzx  r15d,BYTE PTR [rdx-0x700]
  66fbcc:	ff 
  66fbcd:	44 0f b6 a2 00 fa ff 	movzx  r12d,BYTE PTR [rdx-0x600]
  66fbd4:	ff 
  66fbd5:	44 0f b6 aa 00 fb ff 	movzx  r13d,BYTE PTR [rdx-0x500]
  66fbdc:	ff 
  66fbdd:	0f b6 b2 00 fc ff ff 	movzx  esi,BYTE PTR [rdx-0x400]
  66fbe4:	40 88 b4 24 c0 00 00 	mov    BYTE PTR [rsp+0xc0],sil
  66fbeb:	00 
  66fbec:	0f b6 b2 00 fd ff ff 	movzx  esi,BYTE PTR [rdx-0x300]
  66fbf3:	40 88 74 24 40       	mov    BYTE PTR [rsp+0x40],sil
  66fbf8:	0f b6 b2 00 fe ff ff 	movzx  esi,BYTE PTR [rdx-0x200]
  66fbff:	40 88 b4 24 40 01 00 	mov    BYTE PTR [rsp+0x140],sil
  66fc06:	00 
  66fc07:	0f b6 b2 00 ff ff ff 	movzx  esi,BYTE PTR [rdx-0x100]
  66fc0e:	40 88 74 24 20       	mov    BYTE PTR [rsp+0x20],sil
  66fc13:	0f b6 32             	movzx  esi,BYTE PTR [rdx]
  66fc16:	40 88 74 24 18       	mov    BYTE PTR [rsp+0x18],sil
  66fc1b:	0f b6 b4 24 a0 00 00 	movzx  esi,BYTE PTR [rsp+0xa0]
  66fc22:	00 
  66fc23:	40 88 74 08 f1       	mov    BYTE PTR [rax+rcx*1-0xf],sil
  66fc28:	40 88 7c 08 f2       	mov    BYTE PTR [rax+rcx*1-0xe],dil
  66fc2d:	44 88 44 08 f3       	mov    BYTE PTR [rax+rcx*1-0xd],r8b
  66fc32:	44 88 4c 08 f4       	mov    BYTE PTR [rax+rcx*1-0xc],r9b
  66fc37:	44 88 54 08 f5       	mov    BYTE PTR [rax+rcx*1-0xb],r10b
  66fc3c:	44 88 5c 08 f6       	mov    BYTE PTR [rax+rcx*1-0xa],r11b
  66fc41:	88 5c 08 f7          	mov    BYTE PTR [rax+rcx*1-0x9],bl
  66fc45:	44 88 74 08 f8       	mov    BYTE PTR [rax+rcx*1-0x8],r14b
  66fc4a:	44 88 7c 08 f9       	mov    BYTE PTR [rax+rcx*1-0x7],r15b
  66fc4f:	44 88 64 08 fa       	mov    BYTE PTR [rax+rcx*1-0x6],r12b
  66fc54:	44 88 6c 08 fb       	mov    BYTE PTR [rax+rcx*1-0x5],r13b
  66fc59:	0f b6 b4 24 c0 00 00 	movzx  esi,BYTE PTR [rsp+0xc0]
  66fc60:	00 
  66fc61:	40 88 74 08 fc       	mov    BYTE PTR [rax+rcx*1-0x4],sil
  66fc66:	0f b6 74 24 40       	movzx  esi,BYTE PTR [rsp+0x40]
  66fc6b:	40 88 74 08 fd       	mov    BYTE PTR [rax+rcx*1-0x3],sil
  66fc70:	0f b6 b4 24 40 01 00 	movzx  esi,BYTE PTR [rsp+0x140]
  66fc77:	00 
  66fc78:	40 88 74 08 fe       	mov    BYTE PTR [rax+rcx*1-0x2],sil
  66fc7d:	0f b6 74 24 20       	movzx  esi,BYTE PTR [rsp+0x20]
  66fc82:	40 88 74 08 ff       	mov    BYTE PTR [rax+rcx*1-0x1],sil
  66fc87:	0f b6 74 24 18       	movzx  esi,BYTE PTR [rsp+0x18]
  66fc8c:	40 88 34 08          	mov    BYTE PTR [rax+rcx*1],sil
  66fc90:	48 83 c1 10          	add    rcx,0x10
  66fc94:	48 ff c2             	inc    rdx
  66fc97:	48 81 f9 ff 0f 00 00 	cmp    rcx,0xfff
  66fc9e:	0f 85 dc fe ff ff    	jne    66fb80 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5670>
  66fca4:	e9 4b 01 00 00       	jmp    66fdf4 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x58e4>
  66fca9:	bf f0 0f 00 00       	mov    edi,0xff0
  66fcae:	c5 f8 77             	vzeroupper 
  66fcb1:	ff 15 d1 76 09 00    	call   QWORD PTR [rip+0x976d1]        # 707388 <malloc@GLIBC_2.2.5>
  66fcb7:	48 85 c0             	test   rax,rax
  66fcba:	0f 84 d1 05 00 00    	je     670291 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5d81>
  66fcc0:	b9 0f 00 00 00       	mov    ecx,0xf
  66fcc5:	48 8d 94 24 c0 11 00 	lea    rdx,[rsp+0x11c0]
  66fccc:	00 
  66fccd:	0f 1f 00             	nop    DWORD PTR [rax]
  66fcd0:	0f b6 b2 00 f1 ff ff 	movzx  esi,BYTE PTR [rdx-0xf00]
  66fcd7:	40 88 b4 24 a0 00 00 	mov    BYTE PTR [rsp+0xa0],sil
  66fcde:	00 
  66fcdf:	0f b6 ba 00 f2 ff ff 	movzx  edi,BYTE PTR [rdx-0xe00]
  66fce6:	44 0f b6 82 00 f3 ff 	movzx  r8d,BYTE PTR [rdx-0xd00]
  66fced:	ff 
  66fcee:	44 0f b6 8a 00 f4 ff 	movzx  r9d,BYTE PTR [rdx-0xc00]
  66fcf5:	ff 
  66fcf6:	44 0f b6 92 00 f5 ff 	movzx  r10d,BYTE PTR [rdx-0xb00]
  66fcfd:	ff 
  66fcfe:	44 0f b6 9a 00 f6 ff 	movzx  r11d,BYTE PTR [rdx-0xa00]
  66fd05:	ff 
  66fd06:	0f b6 9a 00 f7 ff ff 	movzx  ebx,BYTE PTR [rdx-0x900]
  66fd0d:	44 0f b6 b2 00 f8 ff 	movzx  r14d,BYTE PTR [rdx-0x800]
  66fd14:	ff 
  66fd15:	44 0f b6 ba 00 f9 ff 	movzx  r15d,BYTE PTR [rdx-0x700]
  66fd1c:	ff 
  66fd1d:	44 0f b6 a2 00 fa ff 	movzx  r12d,BYTE PTR [rdx-0x600]
  66fd24:	ff 
  66fd25:	44 0f b6 aa 00 fb ff 	movzx  r13d,BYTE PTR [rdx-0x500]
  66fd2c:	ff 
  66fd2d:	0f b6 b2 00 fc ff ff 	movzx  esi,BYTE PTR [rdx-0x400]
  66fd34:	40 88 b4 24 c0 00 00 	mov    BYTE PTR [rsp+0xc0],sil
  66fd3b:	00 
  66fd3c:	0f b6 b2 00 fd ff ff 	movzx  esi,BYTE PTR [rdx-0x300]
  66fd43:	40 88 74 24 40       	mov    BYTE PTR [rsp+0x40],sil
  66fd48:	0f b6 b2 00 fe ff ff 	movzx  esi,BYTE PTR [rdx-0x200]
  66fd4f:	40 88 b4 24 40 01 00 	mov    BYTE PTR [rsp+0x140],sil
  66fd56:	00 
  66fd57:	0f b6 b2 00 ff ff ff 	movzx  esi,BYTE PTR [rdx-0x100]
  66fd5e:	40 88 74 24 20       	mov    BYTE PTR [rsp+0x20],sil
  66fd63:	0f b6 32             	movzx  esi,BYTE PTR [rdx]
  66fd66:	40 88 74 24 18       	mov    BYTE PTR [rsp+0x18],sil
  66fd6b:	0f b6 b4 24 a0 00 00 	movzx  esi,BYTE PTR [rsp+0xa0]
  66fd72:	00 
  66fd73:	40 88 74 08 f1       	mov    BYTE PTR [rax+rcx*1-0xf],sil
  66fd78:	40 88 7c 08 f2       	mov    BYTE PTR [rax+rcx*1-0xe],dil
  66fd7d:	44 88 44 08 f3       	mov    BYTE PTR [rax+rcx*1-0xd],r8b
  66fd82:	44 88 4c 08 f4       	mov    BYTE PTR [rax+rcx*1-0xc],r9b
  66fd87:	44 88 54 08 f5       	mov    BYTE PTR [rax+rcx*1-0xb],r10b
  66fd8c:	44 88 5c 08 f6       	mov    BYTE PTR [rax+rcx*1-0xa],r11b
  66fd91:	88 5c 08 f7          	mov    BYTE PTR [rax+rcx*1-0x9],bl
  66fd95:	44 88 74 08 f8       	mov    BYTE PTR [rax+rcx*1-0x8],r14b
  66fd9a:	44 88 7c 08 f9       	mov    BYTE PTR [rax+rcx*1-0x7],r15b
  66fd9f:	44 88 64 08 fa       	mov    BYTE PTR [rax+rcx*1-0x6],r12b
  66fda4:	44 88 6c 08 fb       	mov    BYTE PTR [rax+rcx*1-0x5],r13b
  66fda9:	0f b6 b4 24 c0 00 00 	movzx  esi,BYTE PTR [rsp+0xc0]
  66fdb0:	00 
  66fdb1:	40 88 74 08 fc       	mov    BYTE PTR [rax+rcx*1-0x4],sil
  66fdb6:	0f b6 74 24 40       	movzx  esi,BYTE PTR [rsp+0x40]
  66fdbb:	40 88 74 08 fd       	mov    BYTE PTR [rax+rcx*1-0x3],sil
  66fdc0:	0f b6 b4 24 40 01 00 	movzx  esi,BYTE PTR [rsp+0x140]
  66fdc7:	00 
  66fdc8:	40 88 74 08 fe       	mov    BYTE PTR [rax+rcx*1-0x2],sil
  66fdcd:	0f b6 74 24 20       	movzx  esi,BYTE PTR [rsp+0x20]
  66fdd2:	40 88 74 08 ff       	mov    BYTE PTR [rax+rcx*1-0x1],sil
  66fdd7:	0f b6 74 24 18       	movzx  esi,BYTE PTR [rsp+0x18]
  66fddc:	40 88 34 08          	mov    BYTE PTR [rax+rcx*1],sil
  66fde0:	48 83 c1 10          	add    rcx,0x10
  66fde4:	48 ff c2             	inc    rdx
  66fde7:	48 81 f9 ff 0f 00 00 	cmp    rcx,0xfff
  66fdee:	0f 85 dc fe ff ff    	jne    66fcd0 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x57c0>
  66fdf4:	48 8b 4c 24 50       	mov    rcx,QWORD PTR [rsp+0x50]
  66fdf9:	48 c7 01 ff 00 00 00 	mov    QWORD PTR [rcx],0xff
  66fe00:	48 89 41 08          	mov    QWORD PTR [rcx+0x8],rax
  66fe04:	48 c7 41 10 ff 00 00 	mov    QWORD PTR [rcx+0x10],0xff
  66fe0b:	00 
  66fe0c:	48 8d 65 d8          	lea    rsp,[rbp-0x28]
  66fe10:	5b                   	pop    rbx
  66fe11:	41 5c                	pop    r12
  66fe13:	41 5d                	pop    r13
  66fe15:	41 5e                	pop    r14
  66fe17:	41 5f                	pop    r15
  66fe19:	5d                   	pop    rbp
  66fe1a:	c3                   	ret    
  66fe1b:	48 8d 15 4e 41 09 00 	lea    rdx,[rip+0x9414e]        # 703f70 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0xf0>
  66fe22:	c5 f8 77             	vzeroupper 
  66fe25:	ff 15 b5 75 09 00    	call   QWORD PTR [rip+0x975b5]        # 7073e0 <_DYNAMIC+0x250>
  66fe2b:	e9 0f 04 00 00       	jmp    67023f <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5d2f>
  66fe30:	4c 89 c7             	mov    rdi,r8
  66fe33:	eb 38                	jmp    66fe6d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x595d>
  66fe35:	4c 89 ff             	mov    rdi,r15
  66fe38:	eb 33                	jmp    66fe6d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x595d>
  66fe3a:	48 89 c7             	mov    rdi,rax
  66fe3d:	eb 2e                	jmp    66fe6d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x595d>
  66fe3f:	4c 89 d7             	mov    rdi,r10
  66fe42:	eb 29                	jmp    66fe6d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x595d>
  66fe44:	4c 89 e7             	mov    rdi,r12
  66fe47:	eb 24                	jmp    66fe6d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x595d>
  66fe49:	4c 89 cf             	mov    rdi,r9
  66fe4c:	48 8b b4 24 c0 01 00 	mov    rsi,QWORD PTR [rsp+0x1c0]
  66fe53:	00 
  66fe54:	eb 17                	jmp    66fe6d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x595d>
  66fe56:	4c 89 df             	mov    rdi,r11
  66fe59:	eb 12                	jmp    66fe6d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x595d>
  66fe5b:	48 89 df             	mov    rdi,rbx
  66fe5e:	eb 0d                	jmp    66fe6d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x595d>
  66fe60:	48 89 d7             	mov    rdi,rdx
  66fe63:	eb 08                	jmp    66fe6d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x595d>
  66fe65:	4c 89 f7             	mov    rdi,r14
  66fe68:	eb 03                	jmp    66fe6d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x595d>
  66fe6a:	4c 89 ef             	mov    rdi,r13
  66fe6d:	48 8b 94 24 b8 01 00 	mov    rdx,QWORD PTR [rsp+0x1b8]
  66fe74:	00 
  66fe75:	4c 8b 74 24 68       	mov    r14,QWORD PTR [rsp+0x68]
  66fe7a:	ff 15 60 75 09 00    	call   QWORD PTR [rip+0x97560]        # 7073e0 <_DYNAMIC+0x250>
  66fe80:	e9 ba 03 00 00       	jmp    67023f <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5d2f>
  66fe85:	4d 89 f4             	mov    r12,r14
  66fe88:	49 89 ce             	mov    r14,rcx
  66fe8b:	48 89 fb             	mov    rbx,rdi
  66fe8e:	e8 e1 e1 fe ff       	call   65e074 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.8267282299410697290>
  66fe93:	48 89 df             	mov    rdi,rbx
  66fe96:	4c 89 f1             	mov    rcx,r14
  66fe99:	4d 89 e6             	mov    r14,r12
  66fe9c:	8b 05 ae ae 09 00    	mov    eax,DWORD PTR [rip+0x9aeae]        # 70ad50 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1120>
  66fea2:	85 c0                	test   eax,eax
  66fea4:	0f 84 04 a7 ff ff    	je     66a5ae <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x9e>
  66feaa:	4d 89 f4             	mov    r12,r14
  66fead:	49 89 ce             	mov    r14,rcx
  66feb0:	48 89 fb             	mov    rbx,rdi
  66feb3:	e8 0e e2 fe ff       	call   65e0c6 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate6domain6DomainE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.8267282299410697290>
  66feb8:	48 89 df             	mov    rdi,rbx
  66febb:	4c 89 f1             	mov    rcx,r14
  66febe:	4d 89 e6             	mov    r14,r12
  66fec1:	e9 e8 a6 ff ff       	jmp    66a5ae <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x9e>
  66fec6:	48 89 fb             	mov    rbx,rdi
  66fec9:	e8 a6 e1 fe ff       	call   65e074 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.8267282299410697290>
  66fece:	48 89 df             	mov    rdi,rbx
  66fed1:	48 89 fb             	mov    rbx,rdi
  66fed4:	8b 05 76 ae 09 00    	mov    eax,DWORD PTR [rip+0x9ae76]        # 70ad50 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1120>
  66feda:	85 c0                	test   eax,eax
  66fedc:	0f 84 fa ac ff ff    	je     66abdc <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x6cc>
  66fee2:	e8 df e1 fe ff       	call   65e0c6 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate6domain6DomainE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.8267282299410697290>
  66fee7:	e9 f0 ac ff ff       	jmp    66abdc <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x6cc>
  66feec:	4d 89 f4             	mov    r12,r14
  66feef:	49 89 ce             	mov    r14,rcx
  66fef2:	48 89 fb             	mov    rbx,rdi
  66fef5:	e8 cc e1 fe ff       	call   65e0c6 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate6domain6DomainE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.8267282299410697290>
  66fefa:	48 89 df             	mov    rdi,rbx
  66fefd:	4c 89 f1             	mov    rcx,r14
  66ff00:	4d 89 e6             	mov    r14,r12
  66ff03:	e9 f6 b6 ff ff       	jmp    66b5fe <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x10ee>
  66ff08:	e8 67 e1 fe ff       	call   65e074 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.8267282299410697290>
  66ff0d:	e9 09 b7 ff ff       	jmp    66b61b <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x110b>
  66ff12:	4d 89 f4             	mov    r12,r14
  66ff15:	49 89 ce             	mov    r14,rcx
  66ff18:	48 89 fb             	mov    rbx,rdi
  66ff1b:	e8 a6 e1 fe ff       	call   65e0c6 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate6domain6DomainE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.8267282299410697290>
  66ff20:	48 89 df             	mov    rdi,rbx
  66ff23:	4c 89 f1             	mov    rcx,r14
  66ff26:	4d 89 e6             	mov    r14,r12
  66ff29:	e9 90 c3 ff ff       	jmp    66c2be <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1dae>
  66ff2e:	e8 41 e1 fe ff       	call   65e074 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.8267282299410697290>
  66ff33:	e9 a3 c3 ff ff       	jmp    66c2db <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x1dcb>
  66ff38:	4d 89 f4             	mov    r12,r14
  66ff3b:	49 89 ce             	mov    r14,rcx
  66ff3e:	48 89 fb             	mov    rbx,rdi
  66ff41:	e8 80 e1 fe ff       	call   65e0c6 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate6domain6DomainE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.8267282299410697290>
  66ff46:	48 89 df             	mov    rdi,rbx
  66ff49:	4c 89 f1             	mov    rcx,r14
  66ff4c:	4d 89 e6             	mov    r14,r12
  66ff4f:	e9 27 cf ff ff       	jmp    66ce7b <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x296b>
  66ff54:	e8 1b e1 fe ff       	call   65e074 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.8267282299410697290>
  66ff59:	e9 38 cf ff ff       	jmp    66ce96 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x2986>
  66ff5e:	4d 89 f4             	mov    r12,r14
  66ff61:	49 89 ce             	mov    r14,rcx
  66ff64:	48 89 fb             	mov    rbx,rdi
  66ff67:	e8 08 e1 fe ff       	call   65e074 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.8267282299410697290>
  66ff6c:	48 89 df             	mov    rdi,rbx
  66ff6f:	4c 89 f1             	mov    rcx,r14
  66ff72:	4d 89 e6             	mov    r14,r12
  66ff75:	48 89 4c 24 30       	mov    QWORD PTR [rsp+0x30],rcx
  66ff7a:	8b 05 d0 ad 09 00    	mov    eax,DWORD PTR [rip+0x9add0]        # 70ad50 <_RNvNvNtNtCs5Palesvl9wF_6mikros10univariate6domain6domain6DOMAIN.llvm.8267282299410697290+0x1120>
  66ff80:	85 c0                	test   eax,eax
  66ff82:	0f 84 f2 db ff ff    	je     66db7a <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x366a>
  66ff88:	48 89 fb             	mov    rbx,rdi
  66ff8b:	e8 36 e1 fe ff       	call   65e0c6 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate6domain6DomainE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.8267282299410697290>
  66ff90:	48 89 df             	mov    rdi,rbx
  66ff93:	e9 e2 db ff ff       	jmp    66db7a <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x366a>
  66ff98:	4d 89 f4             	mov    r12,r14
  66ff9b:	49 89 ce             	mov    r14,rcx
  66ff9e:	48 89 fb             	mov    rbx,rdi
  66ffa1:	e8 20 e1 fe ff       	call   65e0c6 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate6domain6DomainE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.8267282299410697290>
  66ffa6:	48 89 df             	mov    rdi,rbx
  66ffa9:	4c 89 f1             	mov    rcx,r14
  66ffac:	4d 89 e6             	mov    r14,r12
  66ffaf:	e9 23 e2 ff ff       	jmp    66e1d7 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x3cc7>
  66ffb4:	e8 bb e0 fe ff       	call   65e074 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.8267282299410697290>
  66ffb9:	e9 34 e2 ff ff       	jmp    66e1f2 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x3ce2>
  66ffbe:	4d 89 f4             	mov    r12,r14
  66ffc1:	49 89 ce             	mov    r14,rcx
  66ffc4:	48 89 fb             	mov    rbx,rdi
  66ffc7:	e8 fa e0 fe ff       	call   65e0c6 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate6domain6DomainE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.8267282299410697290>
  66ffcc:	48 89 df             	mov    rdi,rbx
  66ffcf:	4c 89 f1             	mov    rcx,r14
  66ffd2:	4d 89 e6             	mov    r14,r12
  66ffd5:	e9 7b ee ff ff       	jmp    66ee55 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4945>
  66ffda:	e8 95 e0 fe ff       	call   65e074 <_RINvMNtNtCs8srXsAgjQHr_3std4sync9once_lockINtB3_8OnceLockNtNtNtCs5Palesvl9wF_6mikros10univariate5field10SmallFieldE10initializeNCINvB2_11get_or_initNvMBV_BT_3newE0zEBZ_.llvm.8267282299410697290>
  66ffdf:	e9 8e ee ff ff       	jmp    66ee72 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x4962>
  66ffe4:	4c 89 c7             	mov    rdi,r8
  66ffe7:	48 8d 15 d2 40 09 00 	lea    rdx,[rip+0x940d2]        # 7040c0 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x240>
  66ffee:	be 10 00 00 00       	mov    esi,0x10
  66fff3:	c5 f8 77             	vzeroupper 
  66fff6:	ff 15 e4 73 09 00    	call   QWORD PTR [rip+0x973e4]        # 7073e0 <_DYNAMIC+0x250>
  66fffc:	4c 89 d7             	mov    rdi,r10
  66ffff:	48 8d 15 02 41 09 00 	lea    rdx,[rip+0x94102]        # 704108 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x288>
  670006:	be 10 00 00 00       	mov    esi,0x10
  67000b:	c5 f8 77             	vzeroupper 
  67000e:	ff 15 cc 73 09 00    	call   QWORD PTR [rip+0x973cc]        # 7073e0 <_DYNAMIC+0x250>
  670014:	4c 89 e7             	mov    rdi,r12
  670017:	48 8d 15 ea 40 09 00 	lea    rdx,[rip+0x940ea]        # 704108 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x288>
  67001e:	be 10 00 00 00       	mov    esi,0x10
  670023:	c5 f8 77             	vzeroupper 
  670026:	ff 15 b4 73 09 00    	call   QWORD PTR [rip+0x973b4]        # 7073e0 <_DYNAMIC+0x250>
  67002c:	4c 89 cf             	mov    rdi,r9
  67002f:	48 8d 15 f2 3e 09 00 	lea    rdx,[rip+0x93ef2]        # 703f28 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0xa8>
  670036:	be 10 00 00 00       	mov    esi,0x10
  67003b:	c5 f8 77             	vzeroupper 
  67003e:	ff 15 9c 73 09 00    	call   QWORD PTR [rip+0x9739c]        # 7073e0 <_DYNAMIC+0x250>
  670044:	4c 89 ff             	mov    rdi,r15
  670047:	48 8d 15 da 3e 09 00 	lea    rdx,[rip+0x93eda]        # 703f28 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0xa8>
  67004e:	be 10 00 00 00       	mov    esi,0x10
  670053:	c5 f8 77             	vzeroupper 
  670056:	ff 15 84 73 09 00    	call   QWORD PTR [rip+0x97384]        # 7073e0 <_DYNAMIC+0x250>
  67005c:	4c 89 c7             	mov    rdi,r8
  67005f:	48 8d 15 a2 40 09 00 	lea    rdx,[rip+0x940a2]        # 704108 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x288>
  670066:	be 10 00 00 00       	mov    esi,0x10
  67006b:	c5 f8 77             	vzeroupper 
  67006e:	ff 15 6c 73 09 00    	call   QWORD PTR [rip+0x9736c]        # 7073e0 <_DYNAMIC+0x250>
  670074:	48 89 f7             	mov    rdi,rsi
  670077:	48 8d 15 aa 3e 09 00 	lea    rdx,[rip+0x93eaa]        # 703f28 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0xa8>
  67007e:	be 10 00 00 00       	mov    esi,0x10
  670083:	c5 f8 77             	vzeroupper 
  670086:	ff 15 54 73 09 00    	call   QWORD PTR [rip+0x97354]        # 7073e0 <_DYNAMIC+0x250>
  67008c:	4c 89 cf             	mov    rdi,r9
  67008f:	48 8d 15 72 40 09 00 	lea    rdx,[rip+0x94072]        # 704108 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x288>
  670096:	be 10 00 00 00       	mov    esi,0x10
  67009b:	c5 f8 77             	vzeroupper 
  67009e:	ff 15 3c 73 09 00    	call   QWORD PTR [rip+0x9733c]        # 7073e0 <_DYNAMIC+0x250>
  6700a4:	4c 89 c7             	mov    rdi,r8
  6700a7:	48 8d 15 7a 3e 09 00 	lea    rdx,[rip+0x93e7a]        # 703f28 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0xa8>
  6700ae:	be 10 00 00 00       	mov    esi,0x10
  6700b3:	c5 f8 77             	vzeroupper 
  6700b6:	ff 15 24 73 09 00    	call   QWORD PTR [rip+0x97324]        # 7073e0 <_DYNAMIC+0x250>
  6700bc:	48 89 cf             	mov    rdi,rcx
  6700bf:	48 8d 15 42 40 09 00 	lea    rdx,[rip+0x94042]        # 704108 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x288>
  6700c6:	be 10 00 00 00       	mov    esi,0x10
  6700cb:	c5 f8 77             	vzeroupper 
  6700ce:	ff 15 0c 73 09 00    	call   QWORD PTR [rip+0x9730c]        # 7073e0 <_DYNAMIC+0x250>
  6700d4:	4c 89 df             	mov    rdi,r11
  6700d7:	48 8d 15 4a 3e 09 00 	lea    rdx,[rip+0x93e4a]        # 703f28 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0xa8>
  6700de:	be 10 00 00 00       	mov    esi,0x10
  6700e3:	c5 f8 77             	vzeroupper 
  6700e6:	ff 15 f4 72 09 00    	call   QWORD PTR [rip+0x972f4]        # 7073e0 <_DYNAMIC+0x250>
  6700ec:	4c 89 df             	mov    rdi,r11
  6700ef:	48 8d 15 12 40 09 00 	lea    rdx,[rip+0x94012]        # 704108 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x288>
  6700f6:	be 10 00 00 00       	mov    esi,0x10
  6700fb:	c5 f8 77             	vzeroupper 
  6700fe:	ff 15 dc 72 09 00    	call   QWORD PTR [rip+0x972dc]        # 7073e0 <_DYNAMIC+0x250>
  670104:	48 89 c7             	mov    rdi,rax
  670107:	48 8d 15 1a 3e 09 00 	lea    rdx,[rip+0x93e1a]        # 703f28 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0xa8>
  67010e:	be 10 00 00 00       	mov    esi,0x10
  670113:	c5 f8 77             	vzeroupper 
  670116:	ff 15 c4 72 09 00    	call   QWORD PTR [rip+0x972c4]        # 7073e0 <_DYNAMIC+0x250>
  67011c:	4c 89 ff             	mov    rdi,r15
  67011f:	48 8d 15 e2 3f 09 00 	lea    rdx,[rip+0x93fe2]        # 704108 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x288>
  670126:	be 10 00 00 00       	mov    esi,0x10
  67012b:	c5 f8 77             	vzeroupper 
  67012e:	ff 15 ac 72 09 00    	call   QWORD PTR [rip+0x972ac]        # 7073e0 <_DYNAMIC+0x250>
  670134:	4c 89 d7             	mov    rdi,r10
  670137:	48 8d 15 ea 3d 09 00 	lea    rdx,[rip+0x93dea]        # 703f28 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0xa8>
  67013e:	be 10 00 00 00       	mov    esi,0x10
  670143:	c5 f8 77             	vzeroupper 
  670146:	ff 15 94 72 09 00    	call   QWORD PTR [rip+0x97294]        # 7073e0 <_DYNAMIC+0x250>
  67014c:	48 89 cf             	mov    rdi,rcx
  67014f:	48 8d 15 6a 3f 09 00 	lea    rdx,[rip+0x93f6a]        # 7040c0 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x240>
  670156:	be 10 00 00 00       	mov    esi,0x10
  67015b:	c5 f8 77             	vzeroupper 
  67015e:	ff 15 7c 72 09 00    	call   QWORD PTR [rip+0x9727c]        # 7073e0 <_DYNAMIC+0x250>
  670164:	4c 89 d7             	mov    rdi,r10
  670167:	48 8d 15 52 3f 09 00 	lea    rdx,[rip+0x93f52]        # 7040c0 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x240>
  67016e:	be 10 00 00 00       	mov    esi,0x10
  670173:	c5 f8 77             	vzeroupper 
  670176:	ff 15 64 72 09 00    	call   QWORD PTR [rip+0x97264]        # 7073e0 <_DYNAMIC+0x250>
  67017c:	4c 89 cf             	mov    rdi,r9
  67017f:	48 8d 15 3a 3f 09 00 	lea    rdx,[rip+0x93f3a]        # 7040c0 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x240>
  670186:	be 10 00 00 00       	mov    esi,0x10
  67018b:	c5 f8 77             	vzeroupper 
  67018e:	ff 15 4c 72 09 00    	call   QWORD PTR [rip+0x9724c]        # 7073e0 <_DYNAMIC+0x250>
  670194:	48 89 f7             	mov    rdi,rsi
  670197:	48 8d 15 22 3f 09 00 	lea    rdx,[rip+0x93f22]        # 7040c0 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x240>
  67019e:	be 10 00 00 00       	mov    esi,0x10
  6701a3:	c5 f8 77             	vzeroupper 
  6701a6:	ff 15 34 72 09 00    	call   QWORD PTR [rip+0x97234]        # 7073e0 <_DYNAMIC+0x250>
  6701ac:	4c 89 cf             	mov    rdi,r9
  6701af:	48 8d 15 0a 3f 09 00 	lea    rdx,[rip+0x93f0a]        # 7040c0 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x240>
  6701b6:	be 10 00 00 00       	mov    esi,0x10
  6701bb:	c5 f8 77             	vzeroupper 
  6701be:	ff 15 1c 72 09 00    	call   QWORD PTR [rip+0x9721c]        # 7073e0 <_DYNAMIC+0x250>
  6701c4:	48 89 fe             	mov    rsi,rdi
  6701c7:	48 8d 05 72 43 09 00 	lea    rax,[rip+0x94372]        # 704540 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x6c0>
  6701ce:	48 89 84 24 b8 01 00 	mov    QWORD PTR [rsp+0x1b8],rax
  6701d5:	00 
  6701d6:	e9 92 fc ff ff       	jmp    66fe6d <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x595d>
  6701db:	48 8d 15 5e 43 09 00 	lea    rdx,[rip+0x9435e]        # 704540 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x6c0>
  6701e2:	48 89 fe             	mov    rsi,rdi
  6701e5:	ff 15 f5 71 09 00    	call   QWORD PTR [rip+0x971f5]        # 7073e0 <_DYNAMIC+0x250>
  6701eb:	bf 10 00 00 00       	mov    edi,0x10
  6701f0:	be f0 0f 00 00       	mov    esi,0xff0
  6701f5:	ff 15 d5 71 09 00    	call   QWORD PTR [rip+0x971d5]        # 7073d0 <_DYNAMIC+0x240>
  6701fb:	bf 10 00 00 00       	mov    edi,0x10
  670200:	be f0 0f 00 00       	mov    esi,0xff0
  670205:	ff 15 c5 71 09 00    	call   QWORD PTR [rip+0x971c5]        # 7073d0 <_DYNAMIC+0x240>
  67020b:	bf 01 00 00 00       	mov    edi,0x1
  670210:	be f0 0f 00 00       	mov    esi,0xff0
  670215:	ff 15 b5 71 09 00    	call   QWORD PTR [rip+0x971b5]        # 7073d0 <_DYNAMIC+0x240>
  67021b:	eb 22                	jmp    67023f <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5d2f>
  67021d:	48 8d 15 1c 43 09 00 	lea    rdx,[rip+0x9431c]        # 704540 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.8267282299410697290+0x6c0>
  670224:	48 89 fe             	mov    rsi,rdi
  670227:	ff 15 b3 71 09 00    	call   QWORD PTR [rip+0x971b3]        # 7073e0 <_DYNAMIC+0x250>
  67022d:	eb 10                	jmp    67023f <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5d2f>
  67022f:	bf 01 00 00 00       	mov    edi,0x1
  670234:	be f0 0f 00 00       	mov    esi,0xff0
  670239:	ff 15 91 71 09 00    	call   QWORD PTR [rip+0x97191]        # 7073d0 <_DYNAMIC+0x240>
  67023f:	0f 0b                	ud2    
  670241:	bf 01 00 00 00       	mov    edi,0x1
  670246:	be f0 0f 00 00       	mov    esi,0xff0
  67024b:	ff 15 7f 71 09 00    	call   QWORD PTR [rip+0x9717f]        # 7073d0 <_DYNAMIC+0x240>
  670251:	bf 01 00 00 00       	mov    edi,0x1
  670256:	be f0 0f 00 00       	mov    esi,0xff0
  67025b:	ff 15 6f 71 09 00    	call   QWORD PTR [rip+0x9716f]        # 7073d0 <_DYNAMIC+0x240>
  670261:	bf 01 00 00 00       	mov    edi,0x1
  670266:	be f0 0f 00 00       	mov    esi,0xff0
  67026b:	ff 15 5f 71 09 00    	call   QWORD PTR [rip+0x9715f]        # 7073d0 <_DYNAMIC+0x240>
  670271:	bf 01 00 00 00       	mov    edi,0x1
  670276:	be f0 0f 00 00       	mov    esi,0xff0
  67027b:	ff 15 4f 71 09 00    	call   QWORD PTR [rip+0x9714f]        # 7073d0 <_DYNAMIC+0x240>
  670281:	bf 01 00 00 00       	mov    edi,0x1
  670286:	be f0 0f 00 00       	mov    esi,0xff0
  67028b:	ff 15 3f 71 09 00    	call   QWORD PTR [rip+0x9713f]        # 7073d0 <_DYNAMIC+0x240>
  670291:	bf 01 00 00 00       	mov    edi,0x1
  670296:	be f0 0f 00 00       	mov    esi,0xff0
  67029b:	ff 15 2f 71 09 00    	call   QWORD PTR [rip+0x9712f]        # 7073d0 <_DYNAMIC+0x240>
  6702a1:	eb 23                	jmp    6702c6 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5db6>
  6702a3:	49 89 c7             	mov    r15,rax
  6702a6:	48 8b 7c 24 68       	mov    rdi,QWORD PTR [rsp+0x68]
  6702ab:	ff 15 e7 70 09 00    	call   QWORD PTR [rip+0x970e7]        # 707398 <free@GLIBC_2.2.5>
  6702b1:	4c 89 f7             	mov    rdi,r14
  6702b4:	ff 15 de 70 09 00    	call   QWORD PTR [rip+0x970de]        # 707398 <free@GLIBC_2.2.5>
  6702ba:	4c 89 ff             	mov    rdi,r15
  6702bd:	e8 6e 07 09 00       	call   700a30 <_Unwind_Resume@plt>
  6702c2:	eb 02                	jmp    6702c6 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5db6>
  6702c4:	eb 00                	jmp    6702c6 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5db6>
  6702c6:	49 89 c7             	mov    r15,rax
  6702c9:	4c 8b 74 24 68       	mov    r14,QWORD PTR [rsp+0x68]
  6702ce:	4c 89 f7             	mov    rdi,r14
  6702d1:	ff 15 c1 70 09 00    	call   QWORD PTR [rip+0x970c1]        # 707398 <free@GLIBC_2.2.5>
  6702d7:	4c 89 ff             	mov    rdi,r15
  6702da:	e8 51 07 09 00       	call   700a30 <_Unwind_Resume@plt>
  6702df:	eb 00                	jmp    6702e1 <_RNCNvNtCs5Palesvl9wF_6mikros10univariate9aggregate0B5_+0x5dd1>
  6702e1:	49 89 c7             	mov    r15,rax
  6702e4:	4c 89 f7             	mov    rdi,r14
  6702e7:	ff 15 ab 70 09 00    	call   QWORD PTR [rip+0x970ab]        # 707398 <free@GLIBC_2.2.5>
  6702ed:	4c 89 ff             	mov    rdi,r15
  6702f0:	e8 3b 07 09 00       	call   700a30 <_Unwind_Resume@plt>
  6702f5:	cc                   	int3   
  6702f6:	cc                   	int3   
  6702f7:	cc                   	int3   
  6702f8:	cc                   	int3   
  6702f9:	cc                   	int3   
  6702fa:	cc                   	int3   
  6702fb:	cc                   	int3   
  6702fc:	cc                   	int3   
  6702fd:	cc                   	int3   
  6702fe:	cc                   	int3   
  6702ff:	cc                   	int3   

