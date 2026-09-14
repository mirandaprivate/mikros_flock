0000000000664440 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_>:
  664440:	push   rbp
  664441:	push   r15
  664443:	push   r14
  664445:	push   r13
  664447:	push   r12
  664449:	push   rbx
  66444a:	sub    rsp,0x218
  664451:	mov    QWORD PTR [rsp+0x8],r9
  664456:	mov    QWORD PTR [rsp+0x10],rcx
  66445b:	mov    QWORD PTR [rsp+0x50],rdi
  664460:	lea    r9,[r8+0x7]
  664464:	shr    r9,0x3
  664468:	je     665021 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xbe1>
  66446e:	mov    rcx,QWORD PTR [rsi+0x10]
  664472:	mov    r15,QWORD PTR [rsi]
  664475:	mov    rax,QWORD PTR [rsi+0x8]
  664479:	mov    QWORD PTR [rsp+0x88],rax
  664481:	mov    rax,QWORD PTR [rsi+0x20]
  664485:	mov    QWORD PTR [rsp+0x18],rax
  66448a:	mov    rdi,QWORD PTR [rsi+0x18]
  66448e:	mov    rax,QWORD PTR [rsi+0x30]
  664492:	mov    QWORD PTR [rsp+0x28],rax
  664497:	mov    rax,QWORD PTR [rsi+0x28]
  66449b:	mov    QWORD PTR [rsp+0x58],rax
  6644a0:	mov    rax,rdx
  6644a3:	shl    rax,0x4
  6644a7:	mov    QWORD PTR [rsp+0x80],rdi
  6644af:	lea    rax,[rax+rdi*1+0x7]
  6644b4:	mov    QWORD PTR [rsp+0x30],rax
  6644b9:	xor    esi,esi
  6644bb:	mov    eax,0x87
  6644c0:	vmovq  xmm0,rax
  6644c5:	vmovdqa XMMWORD PTR [rsp+0x40],xmm0
  6644cb:	vxorps xmm10,xmm10,xmm10
  6644d0:	vxorps xmm8,xmm8,xmm8
  6644d5:	vxorps xmm9,xmm9,xmm9
  6644da:	mov    r10,r8
  6644dd:	mov    QWORD PTR [rsp+0x38],rcx
  6644e2:	mov    QWORD PTR [rsp+0x20],r8
  6644e7:	mov    QWORD PTR [rsp+0x60],rdx
  6644ec:	jmp    66450f <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xcf>
  6644ee:	xchg   ax,ax
  6644f0:	add    rsi,0x8
  6644f4:	dec    r9
  6644f7:	add    r10,0xfffffffffffffff8
  6644fb:	sub    QWORD PTR [rsp+0x30],0xffffffffffffff80
  664501:	test   r9,r9
  664504:	mov    rcx,QWORD PTR [rsp+0x38]
  664509:	je     66502b <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xbeb>
  66450f:	mov    QWORD PTR [rsp+0x70],r9
  664514:	cmp    r10,0x1
  664518:	mov    QWORD PTR [rsp+0x68],r10
  66451d:	adc    r10,0x0
  664521:	cmp    r10,0x8
  664525:	mov    eax,0x8
  66452a:	cmovae r10,rax
  66452e:	mov    QWORD PTR [rsp+0x110],r10
  664536:	mov    r9,r8
  664539:	sub    r9,rsi
  66453c:	cmp    r9,0x8
  664540:	cmovae r9,rax
  664544:	mov    QWORD PTR [rsp+0x78],r9
  664549:	vpxor  xmm0,xmm0,xmm0
  66454d:	vmovdqu YMMWORD PTR [rsp+0x190],ymm0
  664556:	vmovdqu YMMWORD PTR [rsp+0x170],ymm0
  66455f:	vmovdqu YMMWORD PTR [rsp+0x150],ymm0
  664568:	vmovdqu YMMWORD PTR [rsp+0x130],ymm0
  664571:	vmovdqu YMMWORD PTR [rsp+0xf0],ymm0
  66457a:	vmovdqu YMMWORD PTR [rsp+0xd0],ymm0
  664583:	lea    rdi,[rsi+rdx*1]
  664587:	vmovdqu YMMWORD PTR [rsp+0xb0],ymm0
  664590:	vmovdqu YMMWORD PTR [rsp+0x90],ymm0
  664599:	mov    QWORD PTR [rsp],rsi
  66459d:	cmp    r8,rsi
  6645a0:	jne    664c70 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0x830>
  6645a6:	mov    rdx,QWORD PTR [rsp+0x78]
  6645ab:	mov    rsi,rdx
  6645ae:	add    rsi,rdi
  6645b1:	jb     66507c <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xc3c>
  6645b7:	cmp    rsi,QWORD PTR [rsp+0x28]
  6645bc:	ja     66507c <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xc3c>
  6645c2:	shl    rdi,0x4
  6645c6:	add    rdi,QWORD PTR [rsp+0x58]
  6645cb:	mov    rax,rdx
  6645ce:	shr    rax,0x2
  6645d2:	je     6646c3 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0x283>
  6645d8:	vmovdqu64 zmm0,ZMMWORD PTR [rsp+0x90]
  6645e3:	vmovdqu64 zmm1,ZMMWORD PTR [rdi]
  6645e9:	vpclmulhqlqdq zmm2,zmm0,zmm1
  6645f0:	vpclmullqhqdq zmm3,zmm0,zmm1
  6645f7:	vpxorq zmm2,zmm3,zmm2
  6645fd:	vpclmulhqhqdq zmm3,zmm0,zmm1
  664604:	vpslldq zmm4,zmm3,0x8
  66460b:	vpbroadcastq zmm5,QWORD PTR [rip+0xffffffffff9a85bb]        # cbd0 <anon.36d21461a6996b7f2bd9a76e3225918f.29.llvm.7983859039285366149+0xa0>
  664615:	vpclmulhqlqdq zmm3,zmm3,zmm5
  66461c:	vpternlogq zmm3,zmm2,zmm4,0x96
  664623:	vpclmullqlqdq zmm0,zmm0,zmm1
  66462a:	vpslldq zmm1,zmm3,0x8
  664631:	vpclmulhqlqdq zmm2,zmm3,zmm5
  664638:	vpternlogq zmm2,zmm1,zmm0,0x96
  66463f:	vmovdqu64 ZMMWORD PTR [rsp+0x90],zmm2
  66464a:	cmp    rax,0x1
  66464e:	je     6646c3 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0x283>
  664650:	vmovdqu64 zmm0,ZMMWORD PTR [rsp+0xd0]
  66465b:	vmovdqu64 zmm1,ZMMWORD PTR [rdi+0x40]
  664662:	vpclmulhqlqdq zmm2,zmm0,zmm1
  664669:	vpclmullqhqdq zmm3,zmm0,zmm1
  664670:	vpxorq zmm2,zmm3,zmm2
  664676:	vpclmulhqhqdq zmm3,zmm0,zmm1
  66467d:	vpslldq zmm4,zmm3,0x8
  664684:	vpbroadcastq zmm5,QWORD PTR [rip+0xffffffffff9a8542]        # cbd0 <anon.36d21461a6996b7f2bd9a76e3225918f.29.llvm.7983859039285366149+0xa0>
  66468e:	vpclmulhqlqdq zmm3,zmm3,zmm5
  664695:	vpternlogq zmm3,zmm2,zmm4,0x96
  66469c:	vpclmullqlqdq zmm0,zmm0,zmm1
  6646a3:	vpslldq zmm1,zmm3,0x8
  6646aa:	vpclmulhqlqdq zmm2,zmm3,zmm5
  6646b1:	vpternlogq zmm2,zmm1,zmm0,0x96
  6646b8:	vmovdqu64 ZMMWORD PTR [rsp+0xd0],zmm2
  6646c3:	vmovups YMMWORD PTR [rsp+0x1b0],ymm9
  6646cc:	vmovups YMMWORD PTR [rsp+0x110],ymm8
  6646d5:	mov    rcx,rdx
  6646d8:	and    rcx,0x3
  6646dc:	je     664930 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0x4f0>
  6646e2:	mov    esi,edx
  6646e4:	and    esi,0xc
  6646e7:	shl    esi,0x4
  6646ea:	lea    rax,[rsp+rsi*1+0x90]
  6646f2:	add    rdi,rsi
  6646f5:	mov    rsi,QWORD PTR [rax]
  6646f8:	mov    r8,QWORD PTR [rax+0x8]
  6646fc:	vmovq  xmm0,rsi
  664701:	mov    r9,QWORD PTR [rdi]
  664704:	mov    r10,QWORD PTR [rdi+0x8]
  664708:	vmovq  xmm1,r9
  66470d:	vpclmullqlqdq xmm0,xmm0,xmm1
  664713:	vmovq  xmm1,r8
  664718:	vmovq  xmm2,r10
  66471d:	vpclmullqlqdq xmm1,xmm1,xmm2
  664723:	xor    r8,rsi
  664726:	xor    r10,r9
  664729:	vmovq  xmm2,r8
  66472e:	vmovq  xmm3,r10
  664733:	vpclmullqlqdq xmm2,xmm2,xmm3
  664739:	vpternlogq xmm2,xmm0,xmm1,0x96
  664740:	vpshufd xmm3,xmm1,0xee
  664745:	vmovdqa xmm5,XMMWORD PTR [rsp+0x40]
  66474b:	vpclmullqlqdq xmm3,xmm3,xmm5
  664751:	vpshufd xmm4,xmm2,0xee
  664756:	vpxor  xmm1,xmm4,xmm1
  66475a:	vpclmullqlqdq xmm1,xmm1,xmm5
  664760:	vpextrq rsi,xmm3,0x1
  664766:	lea    r8,[rsi+rsi*1]
  66476a:	shl    rsi,0x2
  66476e:	xor    rsi,r8
  664771:	vpxor  xmm4,xmm0,xmm1
  664775:	vmovq  r8,xmm4
  66477a:	xor    r8,rsi
  66477d:	vpshufd xmm0,xmm0,0xee
  664782:	vpsllq xmm4,xmm3,0x7
  664787:	vpxor  xmm0,xmm3,xmm0
  66478b:	vpbroadcastq xmm2,xmm2
  664790:	vpinsrq xmm2,xmm2,r8,0x0
  664796:	vpalignr xmm0,xmm0,xmm4,0x8
  66479c:	vpunpckhqdq xmm1,xmm3,xmm1
  6647a0:	vpternlogq xmm1,xmm0,xmm2,0x96
  6647a7:	vmovdqa XMMWORD PTR [rax],xmm1
  6647ab:	cmp    ecx,0x1
  6647ae:	je     664930 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0x4f0>
  6647b4:	mov    rsi,QWORD PTR [rax+0x10]
  6647b8:	mov    r8,QWORD PTR [rax+0x18]
  6647bc:	vmovq  xmm0,rsi
  6647c1:	mov    r9,QWORD PTR [rdi+0x10]
  6647c5:	mov    r10,QWORD PTR [rdi+0x18]
  6647c9:	vmovq  xmm1,r9
  6647ce:	vpclmullqlqdq xmm0,xmm0,xmm1
  6647d4:	vmovq  xmm1,r8
  6647d9:	vmovq  xmm2,r10
  6647de:	vpclmullqlqdq xmm1,xmm1,xmm2
  6647e4:	xor    r8,rsi
  6647e7:	xor    r10,r9
  6647ea:	vmovq  xmm2,r8
  6647ef:	vmovq  xmm3,r10
  6647f4:	vpclmullqlqdq xmm2,xmm2,xmm3
  6647fa:	vpternlogq xmm2,xmm0,xmm1,0x96
  664801:	vpshufd xmm3,xmm1,0xee
  664806:	vmovdqa xmm5,XMMWORD PTR [rsp+0x40]
  66480c:	vpclmullqlqdq xmm3,xmm3,xmm5
  664812:	vpshufd xmm4,xmm2,0xee
  664817:	vpxor  xmm1,xmm4,xmm1
  66481b:	vpclmullqlqdq xmm1,xmm1,xmm5
  664821:	vpextrq rsi,xmm3,0x1
  664827:	lea    r8,[rsi+rsi*1]
  66482b:	shl    rsi,0x2
  66482f:	xor    rsi,r8
  664832:	vpxor  xmm4,xmm0,xmm1
  664836:	vmovq  r8,xmm4
  66483b:	xor    r8,rsi
  66483e:	vpshufd xmm0,xmm0,0xee
  664843:	vpsllq xmm4,xmm3,0x7
  664848:	vpxor  xmm0,xmm3,xmm0
  66484c:	vpbroadcastq xmm2,xmm2
  664851:	vpinsrq xmm2,xmm2,r8,0x0
  664857:	vpalignr xmm0,xmm0,xmm4,0x8
  66485d:	vpunpckhqdq xmm1,xmm3,xmm1
  664861:	vpternlogq xmm1,xmm0,xmm2,0x96
  664868:	vmovdqa XMMWORD PTR [rax+0x10],xmm1
  66486d:	cmp    ecx,0x2
  664870:	je     664930 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0x4f0>
  664876:	mov    rcx,QWORD PTR [rax+0x20]
  66487a:	mov    rsi,QWORD PTR [rax+0x28]
  66487e:	vmovq  xmm0,rcx
  664883:	mov    r8,QWORD PTR [rdi+0x20]
  664887:	mov    rdi,QWORD PTR [rdi+0x28]
  66488b:	vmovq  xmm1,r8
  664890:	vpclmullqlqdq xmm0,xmm0,xmm1
  664896:	vmovq  xmm1,rsi
  66489b:	vmovq  xmm2,rdi
  6648a0:	vpclmullqlqdq xmm1,xmm1,xmm2
  6648a6:	xor    rsi,rcx
  6648a9:	xor    rdi,r8
  6648ac:	vmovq  xmm2,rsi
  6648b1:	vmovq  xmm3,rdi
  6648b6:	vpclmullqlqdq xmm2,xmm2,xmm3
  6648bc:	vpternlogq xmm2,xmm0,xmm1,0x96
  6648c3:	vpshufd xmm3,xmm1,0xee
  6648c8:	vmovdqa xmm5,XMMWORD PTR [rsp+0x40]
  6648ce:	vpclmullqlqdq xmm3,xmm3,xmm5
  6648d4:	vpshufd xmm4,xmm2,0xee
  6648d9:	vpxor  xmm1,xmm4,xmm1
  6648dd:	vpextrq rcx,xmm3,0x1
  6648e3:	vpclmullqlqdq xmm1,xmm1,xmm5
  6648e9:	lea    rsi,[rcx+rcx*1]
  6648ed:	shl    rcx,0x2
  6648f1:	xor    rcx,rsi
  6648f4:	vpxor  xmm4,xmm0,xmm1
  6648f8:	vmovq  rsi,xmm4
  6648fd:	xor    rsi,rcx
  664900:	vpshufd xmm0,xmm0,0xee
  664905:	vpsllq xmm4,xmm3,0x7
  66490a:	vpxor  xmm0,xmm3,xmm0
  66490e:	vpalignr xmm0,xmm0,xmm4,0x8
  664914:	vpbroadcastq xmm2,xmm2
  664919:	vpinsrq xmm2,xmm2,rsi,0x0
  66491f:	vpunpckhqdq xmm1,xmm3,xmm1
  664923:	vpternlogq xmm1,xmm0,xmm2,0x96
  66492a:	vmovdqa XMMWORD PTR [rax+0x20],xmm1
  66492f:	nop
  664930:	lea    rdi,[rsp+0x1d8]
  664938:	lea    rsi,[rsp+0x130]
  664940:	lea    rcx,[rsp+0x90]
  664948:	mov    r8,rdx
  66494b:	vzeroupper 
  66494e:	call   66cbd0 <_RNvNtCs5Palesvl9wF_6mikros13inner_product20native_message_chunk>
  664953:	vmovups ymm8,YMMWORD PTR [rsp+0x110]
  66495c:	vxorps ymm8,ymm8,YMMWORD PTR [rsp+0x1d8]
  664965:	vmovups ymm9,YMMWORD PTR [rsp+0x1b0]
  66496e:	vxorps ymm9,ymm9,YMMWORD PTR [rsp+0x1f8]
  664977:	mov    r8,QWORD PTR [rsp+0x20]
  66497c:	mov    rsi,QWORD PTR [rsp]
  664980:	cmp    r8,rsi
  664983:	mov    rdx,QWORD PTR [rsp+0x60]
  664988:	mov    r9,QWORD PTR [rsp+0x70]
  66498d:	vxorps xmm10,xmm10,xmm10
  664992:	mov    r10,QWORD PTR [rsp+0x68]
  664997:	je     6644f0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xb0>
  66499d:	jbe    6650b0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xc70>
  6649a3:	mov    rax,rsi
  6649a6:	shl    rax,0x4
  6649aa:	vmovaps xmm0,XMMWORD PTR [rsp+0x130]
  6649b3:	mov    rcx,QWORD PTR [rsp+0x10]
  6649b8:	vmovaps XMMWORD PTR [rcx+rax*1],xmm0
  6649bd:	cmp    rsi,QWORD PTR [rsp+0x250]
  6649c5:	jae    665091 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xc51>
  6649cb:	vmovdqa xmm0,XMMWORD PTR [rsp+0x90]
  6649d4:	mov    rcx,QWORD PTR [rsp+0x8]
  6649d9:	vmovdqa XMMWORD PTR [rcx+rax*1],xmm0
  6649de:	cmp    r10,0x2
  6649e2:	jb     6644f0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xb0>
  6649e8:	mov    rdi,rsi
  6649eb:	or     rdi,0x1
  6649ef:	cmp    rdi,r8
  6649f2:	jae    6650ac <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xc6c>
  6649f8:	mov    rax,rdi
  6649fb:	shl    rax,0x4
  6649ff:	lea    rcx,[rsp+0x140]
  664a07:	vmovaps xmm0,XMMWORD PTR [rcx]
  664a0b:	mov    rcx,QWORD PTR [rsp+0x10]
  664a10:	vmovaps XMMWORD PTR [rcx+rax*1],xmm0
  664a15:	cmp    rdi,QWORD PTR [rsp+0x250]
  664a1d:	jae    665094 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xc54>
  664a23:	lea    rcx,[rsp+0xa0]
  664a2b:	vmovdqa xmm0,XMMWORD PTR [rcx]
  664a2f:	mov    rcx,QWORD PTR [rsp+0x8]
  664a34:	vmovdqa XMMWORD PTR [rcx+rax*1],xmm0
  664a39:	cmp    r10,0x2
  664a3d:	je     6644f0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xb0>
  664a43:	mov    rdi,rsi
  664a46:	or     rdi,0x2
  664a4a:	cmp    rdi,r8
  664a4d:	jae    6650ac <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xc6c>
  664a53:	mov    rax,rdi
  664a56:	shl    rax,0x4
  664a5a:	lea    rcx,[rsp+0x140]
  664a62:	vmovaps xmm0,XMMWORD PTR [rcx+0x10]
  664a67:	mov    rcx,QWORD PTR [rsp+0x10]
  664a6c:	vmovaps XMMWORD PTR [rcx+rax*1],xmm0
  664a71:	cmp    rdi,QWORD PTR [rsp+0x250]
  664a79:	jae    665094 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xc54>
  664a7f:	lea    rcx,[rsp+0xa0]
  664a87:	vmovdqa xmm0,XMMWORD PTR [rcx+0x10]
  664a8c:	mov    rcx,QWORD PTR [rsp+0x8]
  664a91:	vmovdqa XMMWORD PTR [rcx+rax*1],xmm0
  664a96:	cmp    r10,0x3
  664a9a:	je     6644f0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xb0>
  664aa0:	mov    rdi,rsi
  664aa3:	or     rdi,0x3
  664aa7:	cmp    rdi,r8
  664aaa:	jae    6650ac <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xc6c>
  664ab0:	mov    rax,rdi
  664ab3:	shl    rax,0x4
  664ab7:	lea    rcx,[rsp+0x140]
  664abf:	vmovaps xmm0,XMMWORD PTR [rcx+0x20]
  664ac4:	mov    rcx,QWORD PTR [rsp+0x10]
  664ac9:	vmovaps XMMWORD PTR [rcx+rax*1],xmm0
  664ace:	cmp    rdi,QWORD PTR [rsp+0x250]
  664ad6:	jae    665094 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xc54>
  664adc:	lea    rcx,[rsp+0xa0]
  664ae4:	vmovdqa xmm0,XMMWORD PTR [rcx+0x20]
  664ae9:	mov    rcx,QWORD PTR [rsp+0x8]
  664aee:	vmovdqa XMMWORD PTR [rcx+rax*1],xmm0
  664af3:	cmp    r10,0x4
  664af7:	je     6644f0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xb0>
  664afd:	mov    rdi,rsi
  664b00:	or     rdi,0x4
  664b04:	cmp    rdi,r8
  664b07:	jae    6650ac <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xc6c>
  664b0d:	mov    rax,rdi
  664b10:	shl    rax,0x4
  664b14:	lea    rcx,[rsp+0x140]
  664b1c:	vmovaps xmm0,XMMWORD PTR [rcx+0x30]
  664b21:	mov    rcx,QWORD PTR [rsp+0x10]
  664b26:	vmovaps XMMWORD PTR [rcx+rax*1],xmm0
  664b2b:	cmp    rdi,QWORD PTR [rsp+0x250]
  664b33:	jae    665094 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xc54>
  664b39:	lea    rcx,[rsp+0xa0]
  664b41:	vmovdqa xmm0,XMMWORD PTR [rcx+0x30]
  664b46:	mov    rcx,QWORD PTR [rsp+0x8]
  664b4b:	vmovdqa XMMWORD PTR [rcx+rax*1],xmm0
  664b50:	cmp    r10,0x5
  664b54:	je     6644f0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xb0>
  664b5a:	mov    rdi,rsi
  664b5d:	or     rdi,0x5
  664b61:	cmp    rdi,r8
  664b64:	jae    6650ac <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xc6c>
  664b6a:	mov    rax,rdi
  664b6d:	shl    rax,0x4
  664b71:	lea    rcx,[rsp+0x140]
  664b79:	vmovaps xmm0,XMMWORD PTR [rcx+0x40]
  664b7e:	mov    rcx,QWORD PTR [rsp+0x10]
  664b83:	vmovaps XMMWORD PTR [rcx+rax*1],xmm0
  664b88:	cmp    rdi,QWORD PTR [rsp+0x250]
  664b90:	jae    665094 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xc54>
  664b96:	lea    rcx,[rsp+0xa0]
  664b9e:	vmovdqa xmm0,XMMWORD PTR [rcx+0x40]
  664ba3:	mov    rcx,QWORD PTR [rsp+0x8]
  664ba8:	vmovdqa XMMWORD PTR [rcx+rax*1],xmm0
  664bad:	cmp    r10,0x6
  664bb1:	je     6644f0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xb0>
  664bb7:	mov    rdi,rsi
  664bba:	or     rdi,0x6
  664bbe:	cmp    rdi,r8
  664bc1:	jae    6650ac <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xc6c>
  664bc7:	mov    rax,rdi
  664bca:	shl    rax,0x4
  664bce:	lea    rcx,[rsp+0x140]
  664bd6:	vmovaps xmm0,XMMWORD PTR [rcx+0x50]
  664bdb:	mov    rcx,QWORD PTR [rsp+0x10]
  664be0:	vmovaps XMMWORD PTR [rcx+rax*1],xmm0
  664be5:	cmp    rdi,QWORD PTR [rsp+0x250]
  664bed:	jae    665094 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xc54>
  664bf3:	lea    rcx,[rsp+0xa0]
  664bfb:	vmovdqa xmm0,XMMWORD PTR [rcx+0x50]
  664c00:	mov    rcx,QWORD PTR [rsp+0x8]
  664c05:	vmovdqa XMMWORD PTR [rcx+rax*1],xmm0
  664c0a:	cmp    r10,0x7
  664c0e:	je     6644f0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xb0>
  664c14:	mov    rdi,rsi
  664c17:	or     rdi,0x7
  664c1b:	cmp    rdi,r8
  664c1e:	jae    6650ac <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xc6c>
  664c24:	mov    rax,rdi
  664c27:	shl    rax,0x4
  664c2b:	lea    rcx,[rsp+0x140]
  664c33:	vmovaps xmm0,XMMWORD PTR [rcx+0x60]
  664c38:	mov    rcx,QWORD PTR [rsp+0x10]
  664c3d:	vmovaps XMMWORD PTR [rcx+rax*1],xmm0
  664c42:	cmp    rdi,QWORD PTR [rsp+0x250]
  664c4a:	jae    665094 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xc54>
  664c50:	lea    rcx,[rsp+0xa0]
  664c58:	vmovdqa xmm0,XMMWORD PTR [rcx+0x60]
  664c5d:	mov    rcx,QWORD PTR [rsp+0x8]
  664c62:	vmovdqa XMMWORD PTR [rcx+rax*1],xmm0
  664c67:	jmp    6644f0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xb0>
  664c6c:	nop    DWORD PTR [rax+0x0]
  664c70:	mov    r8,QWORD PTR [rsp+0x30]
  664c75:	xor    r9d,r9d
  664c78:	jmp    664d26 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0x8e6>
  664c7d:	nop    DWORD PTR [rax]
  664c80:	mov    r15,rdx
  664c83:	mov    rdi,rbx
  664c86:	vpxor  xmm0,xmm0,xmm0
  664c8a:	xor    r12d,r12d
  664c8d:	vpxor  xmm1,xmm1,xmm1
  664c91:	vpxor  xmm2,xmm2,xmm2
  664c95:	vpxor  xmm3,xmm3,xmm3
  664c99:	add    rax,QWORD PTR [rsp+0x80]
  664ca1:	mov    rsi,r12
  664ca4:	shl    rsi,0xe
  664ca8:	add    r10,rsi
  664cab:	movzx  esi,BYTE PTR [rax+r12*4]
  664cb0:	shl    esi,0x4
  664cb3:	movzx  r11d,BYTE PTR [rax+r12*4+0x1]
  664cb9:	shl    r11d,0x4
  664cbd:	movzx  r14d,BYTE PTR [rax+r12*4+0x2]
  664cc3:	shl    r14d,0x4
  664cc7:	movzx  eax,BYTE PTR [rax+r12*4+0x3]
  664ccd:	shl    eax,0x4
  664cd0:	vpxor  xmm1,xmm1,XMMWORD PTR [rsi+r10*1]
  664cd6:	vpxor  xmm0,xmm0,XMMWORD PTR [r11+r10*1+0x1000]
  664ce0:	vpxor  xmm2,xmm2,XMMWORD PTR [r14+r10*1+0x2000]
  664cea:	vpxor  xmm3,xmm3,XMMWORD PTR [rax+r10*1+0x3000]
  664cf4:	vpxor  xmm0,xmm0,xmm1
  664cf8:	vpternlogq xmm0,xmm3,xmm2,0x96
  664cff:	lea    rax,[r9+0x1]
  664d03:	shl    r9,0x4
  664d07:	vmovdqa XMMWORD PTR [rsp+r9*1+0x90],xmm0
  664d11:	add    r8,0x10
  664d15:	mov    r9,rax
  664d18:	cmp    rax,QWORD PTR [rsp+0x110]
  664d20:	je     6645a6 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0x166>
  664d26:	lea    rax,[r9+rdi*1]
  664d2a:	shl    rax,0x4
  664d2e:	lea    rsi,[rax+0x10]
  664d32:	lea    r10,[rax+0xf]
  664d36:	cmp    r10,rcx
  664d39:	jae    66504e <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xc0e>
  664d3f:	mov    edx,0x33
  664d44:	bzhi   r12,QWORD PTR [r15+0x10],rdx
  664d4a:	shr    r12,0x2
  664d4e:	cmp    r12,0x4
  664d52:	mov    r11d,0x4
  664d58:	cmovb  r11,r12
  664d5c:	test   r12,r12
  664d5f:	je     665000 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xbc0>
  664d65:	mov    rbx,rdi
  664d68:	mov    rdx,r15
  664d6b:	mov    r10,QWORD PTR [r15+0x8]
  664d6f:	mov    rdi,QWORD PTR [rsp+0x88]
  664d77:	movzx  r14d,BYTE PTR [rdi+rax*1]
  664d7c:	shl    r14d,0x4
  664d80:	movzx  r13d,BYTE PTR [rdi+rax*1+0x1]
  664d86:	shl    r13d,0x4
  664d8a:	movzx  r15d,BYTE PTR [rdi+rax*1+0x2]
  664d90:	shl    r15d,0x4
  664d94:	movzx  ebp,BYTE PTR [rdi+rax*1+0x3]
  664d99:	vmovdqa xmm0,XMMWORD PTR [r10+r14*1]
  664d9f:	vmovdqa xmm1,XMMWORD PTR [r10+r13*1+0x1000]
  664da9:	shl    ebp,0x4
  664dac:	vmovdqa xmm2,XMMWORD PTR [r10+r15*1+0x2000]
  664db6:	vmovdqa xmm3,XMMWORD PTR [r10+rbp*1+0x3000]
  664dc0:	cmp    r12,0x1
  664dc4:	je     664ed0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xa90>
  664dca:	movzx  r14d,BYTE PTR [rdi+rax*1+0x4]
  664dd0:	shl    r14d,0x4
  664dd4:	movzx  r15d,BYTE PTR [rdi+rax*1+0x5]
  664dda:	shl    r15d,0x4
  664dde:	movzx  r13d,BYTE PTR [rdi+rax*1+0x6]
  664de4:	shl    r13d,0x4
  664de8:	movzx  ebp,BYTE PTR [rdi+rax*1+0x7]
  664ded:	vpxor  xmm0,xmm0,XMMWORD PTR [r10+r14*1+0x4000]
  664df7:	vpxor  xmm1,xmm1,XMMWORD PTR [r10+r15*1+0x5000]
  664e01:	shl    ebp,0x4
  664e04:	vpxor  xmm2,xmm2,XMMWORD PTR [r10+r13*1+0x6000]
  664e0e:	vpxor  xmm3,xmm3,XMMWORD PTR [r10+rbp*1+0x7000]
  664e18:	cmp    r12,0x2
  664e1c:	je     664ed0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xa90>
  664e22:	movzx  r14d,BYTE PTR [rdi+rax*1+0x8]
  664e28:	shl    r14d,0x4
  664e2c:	movzx  r15d,BYTE PTR [rdi+rax*1+0x9]
  664e32:	shl    r15d,0x4
  664e36:	movzx  r13d,BYTE PTR [rdi+rax*1+0xa]
  664e3c:	shl    r13d,0x4
  664e40:	movzx  ebp,BYTE PTR [rdi+rax*1+0xb]
  664e45:	vpxor  xmm0,xmm0,XMMWORD PTR [r10+r14*1+0x8000]
  664e4f:	vpxor  xmm1,xmm1,XMMWORD PTR [r10+r15*1+0x9000]
  664e59:	shl    ebp,0x4
  664e5c:	vpxor  xmm2,xmm2,XMMWORD PTR [r10+r13*1+0xa000]
  664e66:	vpxor  xmm3,xmm3,XMMWORD PTR [r10+rbp*1+0xb000]
  664e70:	cmp    r12,0x3
  664e74:	je     664ed0 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xa90>
  664e76:	movzx  r14d,BYTE PTR [rdi+rax*1+0xc]
  664e7c:	shl    r14d,0x4
  664e80:	movzx  r15d,BYTE PTR [rdi+rax*1+0xd]
  664e86:	shl    r15d,0x4
  664e8a:	movzx  r13d,BYTE PTR [rdi+rax*1+0xe]
  664e90:	shl    r13d,0x4
  664e94:	movzx  ebp,BYTE PTR [rdi+rax*1+0xf]
  664e99:	shl    ebp,0x4
  664e9c:	vpxor  xmm0,xmm0,XMMWORD PTR [r10+r14*1+0xc000]
  664ea6:	vpxor  xmm1,xmm1,XMMWORD PTR [r10+r15*1+0xd000]
  664eb0:	vpxor  xmm2,xmm2,XMMWORD PTR [r10+r13*1+0xe000]
  664eba:	vpxor  xmm3,xmm3,XMMWORD PTR [r10+rbp*1+0xf000]
  664ec4:	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  664ed0:	vpxor  xmm0,xmm1,xmm0
  664ed4:	vpternlogq xmm0,xmm3,xmm2,0x96
  664edb:	mov    r14,r9
  664ede:	shl    r14,0x4
  664ee2:	vmovdqa XMMWORD PTR [rsp+r14*1+0x130],xmm0
  664eec:	cmp    rsi,QWORD PTR [rsp+0x18]
  664ef1:	ja     665064 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xc24>
  664ef7:	cmp    r12,0x1
  664efb:	je     664c80 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0x840>
  664f01:	mov    esi,r11d
  664f04:	and    esi,0x6
  664f07:	lea    r14,[r10+0x7000]
  664f0e:	vpxor  xmm0,xmm0,xmm0
  664f12:	xor    r12d,r12d
  664f15:	vpxor  xmm1,xmm1,xmm1
  664f19:	vpxor  xmm2,xmm2,xmm2
  664f1d:	vpxor  xmm3,xmm3,xmm3
  664f21:	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  664f30:	movzx  r15d,BYTE PTR [r8+r12*4-0x7]
  664f36:	shl    r15d,0x4
  664f3a:	movzx  r13d,BYTE PTR [r8+r12*4-0x6]
  664f40:	shl    r13d,0x4
  664f44:	movzx  ebp,BYTE PTR [r8+r12*4-0x5]
  664f4a:	shl    ebp,0x4
  664f4d:	movzx  ecx,BYTE PTR [r8+r12*4-0x4]
  664f53:	vmovdqa xmm4,XMMWORD PTR [r14+r15*1-0x7000]
  664f5d:	vmovdqa xmm5,XMMWORD PTR [r14+r13*1-0x6000]
  664f67:	shl    ecx,0x4
  664f6a:	vmovdqa xmm6,XMMWORD PTR [r14+rbp*1-0x5000]
  664f74:	vmovdqa xmm7,XMMWORD PTR [r14+rcx*1-0x4000]
  664f7e:	movzx  ecx,BYTE PTR [r8+r12*4-0x3]
  664f84:	movzx  r15d,BYTE PTR [r8+r12*4-0x2]
  664f8a:	movzx  r13d,BYTE PTR [r8+r12*4-0x1]
  664f90:	movzx  ebp,BYTE PTR [r8+r12*4]
  664f95:	add    r12,0x2
  664f99:	shl    ecx,0x4
  664f9c:	shl    r15d,0x4
  664fa0:	shl    r13d,0x4
  664fa4:	shl    ebp,0x4
  664fa7:	vpternlogq xmm1,xmm4,XMMWORD PTR [r14+rcx*1-0x3000],0x96
  664fb3:	vpternlogq xmm0,xmm5,XMMWORD PTR [r14+r15*1-0x2000],0x96
  664fbf:	vpternlogq xmm2,xmm6,XMMWORD PTR [r14+r13*1-0x1000],0x96
  664fcb:	vpternlogq xmm3,xmm7,XMMWORD PTR [r14+rbp*1],0x96
  664fd3:	add    r14,0x8000
  664fda:	cmp    rsi,r12
  664fdd:	jne    664f30 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xaf0>
  664fe3:	test   r11b,0x1
  664fe7:	mov    rcx,QWORD PTR [rsp+0x38]
  664fec:	mov    r15,rdx
  664fef:	mov    rdi,rbx
  664ff2:	jne    664c99 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0x859>
  664ff8:	jmp    664cf4 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0x8b4>
  664ffd:	nop    DWORD PTR [rax]
  665000:	mov    r10,r9
  665003:	shl    r10,0x4
  665007:	vmovaps XMMWORD PTR [rsp+r10*1+0x130],xmm10
  665011:	cmp    rsi,QWORD PTR [rsp+0x18]
  665016:	ja     665064 <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0xc24>
  665018:	vpxor  xmm0,xmm0,xmm0
  66501c:	jmp    664cff <_RNCNvNtCs5Palesvl9wF_6mikros6lookup36pack_pair_weighted_with_message_into0B5_+0x8bf>
  665021:	vxorps xmm8,xmm8,xmm8
  665026:	vxorps xmm9,xmm9,xmm9
  66502b:	mov    rax,QWORD PTR [rsp+0x50]
  665030:	vmovups YMMWORD PTR [rax],ymm8
  665034:	vmovups YMMWORD PTR [rax+0x20],ymm9
  665039:	add    rsp,0x218
  665040:	pop    rbx
  665041:	pop    r12
  665043:	pop    r13
  665045:	pop    r14
  665047:	pop    r15
  665049:	pop    rbp
  66504a:	vzeroupper 
  66504d:	ret    
  66504e:	mov    rdx,rcx
  665051:	lea    rcx,[rip+0x8fef0]        # 6f4f48 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x1c8>
  665058:	mov    rdi,rax
  66505b:	vzeroupper 
  66505e:	call   QWORD PTR [rip+0x932f4]        # 6f8358 <_DYNAMIC+0x340>
  665064:	lea    rcx,[rip+0x8fec5]        # 6f4f30 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x1b0>
  66506b:	mov    rdi,rax
  66506e:	mov    rdx,QWORD PTR [rsp+0x18]
  665073:	vzeroupper 
  665076:	call   QWORD PTR [rip+0x932dc]        # 6f8358 <_DYNAMIC+0x340>
  66507c:	lea    rcx,[rip+0x8fe95]        # 6f4f18 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x198>
  665083:	mov    rdx,QWORD PTR [rsp+0x28]
  665088:	vzeroupper 
  66508b:	call   QWORD PTR [rip+0x932c7]        # 6f8358 <_DYNAMIC+0x340>
  665091:	mov    rdi,rsi
  665094:	lea    rdx,[rip+0x8fe65]        # 6f4f00 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x180>
  66509b:	mov    rsi,QWORD PTR [rsp+0x250]
  6650a3:	vzeroupper 
  6650a6:	call   QWORD PTR [rip+0x931bc]        # 6f8268 <_DYNAMIC+0x250>
  6650ac:	mov    QWORD PTR [rsp],rdi
  6650b0:	lea    rdx,[rip+0x8fe31]        # 6f4ee8 <anon.0e5291c933a25680de2bf032d375ada2.12.llvm.16073103944965275176+0x168>
  6650b7:	mov    rdi,QWORD PTR [rsp]
  6650bb:	mov    rsi,QWORD PTR [rsp+0x20]
  6650c0:	vzeroupper 
  6650c3:	call   QWORD PTR [rip+0x9319f]        # 6f8268 <_DYNAMIC+0x250>
  6650c9:	int3   
  6650ca:	int3   
  6650cb:	int3   
  6650cc:	int3   
  6650cd:	int3   
  6650ce:	int3   
  6650cf:	int3   

