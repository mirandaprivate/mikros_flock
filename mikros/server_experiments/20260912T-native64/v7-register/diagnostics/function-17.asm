00000000006708e0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled12consume_four>:
  6708e0:	push   rbx
  6708e1:	mov    rbx,QWORD PTR [rsp+0x10]
  6708e6:	mov    r11,QWORD PTR [rsp+0x18]
  6708eb:	lea    r10,[r11+0x3]
  6708ef:	mov    rax,r10
  6708f2:	shl    rax,0x8
  6708f6:	vmovdqu64 zmm0,ZMMWORD PTR [rdi+rax*1]
  6708fd:	vmovdqu64 zmm1,ZMMWORD PTR [rdi+rax*1+0x40]
  670905:	vmovdqu64 zmm2,ZMMWORD PTR [rdi+rax*1+0x80]
  67090d:	vmovdqu64 zmm3,ZMMWORD PTR [rdi+rax*1+0xc0]
  670915:	add    rax,rdi
  670918:	cmp    r8,rdx
  67091b:	cmovb  rdx,r8
  67091f:	cmp    rbx,rdx
  670922:	cmovb  rdx,rbx
  670926:	test   rdx,rdx
  670929:	je     670afa <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled12consume_four+0x21a>
  67092f:	shl    r11,0x8
  670933:	lea    r8,[rdi+r11*1]
  670937:	lea    rdi,[rdi+r11*1+0x200]
  67093f:	vmovdqu64 zmm4,ZMMWORD PTR [rdi]
  670945:	vmovdqu64 zmm5,ZMMWORD PTR [rdi+0x40]
  67094c:	vmovdqu64 zmm6,ZMMWORD PTR [rdi+0x80]
  670953:	vmovdqu64 zmm7,ZMMWORD PTR [rdi+0xc0]
  67095a:	vmovdqu64 zmm8,ZMMWORD PTR [r8+0x1c0]
  670961:	vmovdqu64 zmm10,ZMMWORD PTR [r8]
  670967:	vmovdqu64 zmm11,ZMMWORD PTR [r8+0x40]
  67096e:	vmovdqu64 zmm13,ZMMWORD PTR [r8+0x80]
  670975:	vmovdqu64 zmm14,ZMMWORD PTR [r8+0xc0]
  67097c:	vmovdqu64 zmm9,ZMMWORD PTR [r8+0x180]
  670983:	vmovdqu64 zmm12,ZMMWORD PTR [r8+0x140]
  67098a:	vmovdqu64 zmm15,ZMMWORD PTR [r8+0x100]
  670991:	add    rcx,r10
  670994:	shl    rdx,0x4
  670998:	xor    r10d,r10d
  67099b:	nop    DWORD PTR [rax+rax*1+0x0]
  6709a0:	vpbroadcastb zmm16,BYTE PTR [rcx+r10*1-0x3]
  6709a8:	kmovq  k2,QWORD PTR [r9+r10*1]
  6709ae:	kmovq  k1,QWORD PTR [r9+r10*1+0x8]
  6709b5:	vmovdqu8 zmm17{k2}{z},zmm16
  6709bb:	vpxord zmm10,zmm10,zmm17
  6709c1:	vmovdqu8 zmm17{k1}{z},zmm16
  6709c7:	vpxord zmm11,zmm11,zmm17
  6709cd:	vmovdqa64 zmm17,ZMMWORD PTR [rsi+r10*8]
  6709d4:	vmovdqa64 zmm18,ZMMWORD PTR [rsi+r10*8+0x40]
  6709dc:	vgf2p8mulb zmm19,zmm17,zmm16
  6709e2:	vpxord zmm13,zmm13,zmm19
  6709e8:	vgf2p8mulb zmm16,zmm18,zmm16
  6709ee:	vpxord zmm14,zmm14,zmm16
  6709f4:	vpbroadcastb zmm16,BYTE PTR [rcx+r10*1-0x2]
  6709fc:	vmovdqu8 zmm19{k2}{z},zmm16
  670a02:	vpxord zmm15,zmm15,zmm19
  670a08:	vmovdqu8 zmm19{k1}{z},zmm16
  670a0e:	vpxord zmm12,zmm12,zmm19
  670a14:	vgf2p8mulb zmm19,zmm17,zmm16
  670a1a:	vpxord zmm9,zmm9,zmm19
  670a20:	vgf2p8mulb zmm16,zmm18,zmm16
  670a26:	vpxord zmm8,zmm8,zmm16
  670a2c:	vpbroadcastb zmm16,BYTE PTR [rcx+r10*1-0x1]
  670a34:	vmovdqu8 zmm19{k2}{z},zmm16
  670a3a:	vpxord zmm4,zmm4,zmm19
  670a40:	vmovdqu8 zmm19{k1}{z},zmm16
  670a46:	vpxord zmm5,zmm5,zmm19
  670a4c:	vgf2p8mulb zmm19,zmm17,zmm16
  670a52:	vpxord zmm6,zmm6,zmm19
  670a58:	vgf2p8mulb zmm16,zmm18,zmm16
  670a5e:	vpxord zmm7,zmm7,zmm16
  670a64:	vpbroadcastb zmm16,BYTE PTR [rcx+r10*1]
  670a6b:	vmovdqu8 zmm19{k2}{z},zmm16
  670a71:	vpxord zmm0,zmm0,zmm19
  670a77:	vmovdqu8 zmm19{k1}{z},zmm16
  670a7d:	vpxord zmm1,zmm1,zmm19
  670a83:	vgf2p8mulb zmm17,zmm17,zmm16
  670a89:	vpxord zmm2,zmm2,zmm17
  670a8f:	vgf2p8mulb zmm16,zmm18,zmm16
  670a95:	vpxord zmm3,zmm3,zmm16
  670a9b:	add    r10,0x10
  670a9f:	cmp    rdx,r10
  670aa2:	jne    6709a0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled12consume_four+0xc0>
  670aa8:	vmovdqu64 ZMMWORD PTR [r8],zmm10
  670aae:	vmovdqu64 ZMMWORD PTR [r8+0x40],zmm11
  670ab5:	vmovdqu64 ZMMWORD PTR [r8+0x80],zmm13
  670abc:	vmovdqu64 ZMMWORD PTR [r8+0xc0],zmm14
  670ac3:	vmovdqu64 ZMMWORD PTR [r8+0x100],zmm15
  670aca:	vmovdqu64 ZMMWORD PTR [r8+0x140],zmm12
  670ad1:	vmovdqu64 ZMMWORD PTR [r8+0x180],zmm9
  670ad8:	vmovdqu64 ZMMWORD PTR [r8+0x1c0],zmm8
  670adf:	vmovdqu64 ZMMWORD PTR [rdi],zmm4
  670ae5:	vmovdqu64 ZMMWORD PTR [rdi+0x40],zmm5
  670aec:	vmovdqu64 ZMMWORD PTR [rdi+0x80],zmm6
  670af3:	vmovdqu64 ZMMWORD PTR [rdi+0xc0],zmm7
  670afa:	vmovdqu64 ZMMWORD PTR [rax],zmm0
  670b00:	vmovdqu64 ZMMWORD PTR [rax+0x40],zmm1
  670b07:	vmovdqu64 ZMMWORD PTR [rax+0x80],zmm2
  670b0e:	vmovdqu64 ZMMWORD PTR [rax+0xc0],zmm3
  670b15:	pop    rbx
  670b16:	vzeroupper 
  670b19:	ret    
  670b1a:	int3   
  670b1b:	int3   
  670b1c:	int3   
  670b1d:	int3   
  670b1e:	int3   
  670b1f:	int3   

