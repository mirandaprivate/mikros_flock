00000000006677d0 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled12consume_four>:
  6677d0:	push   rbx
  6677d1:	mov    rbx,QWORD PTR [rsp+0x10]
  6677d6:	mov    r11,QWORD PTR [rsp+0x18]
  6677db:	lea    r10,[r11+0x3]
  6677df:	mov    rax,r10
  6677e2:	shl    rax,0x8
  6677e6:	vmovdqu64 zmm0,ZMMWORD PTR [rdi+rax*1]
  6677ed:	vmovdqu64 zmm1,ZMMWORD PTR [rdi+rax*1+0x40]
  6677f5:	vmovdqu64 zmm2,ZMMWORD PTR [rdi+rax*1+0x80]
  6677fd:	vmovdqu64 zmm3,ZMMWORD PTR [rdi+rax*1+0xc0]
  667805:	add    rax,rdi
  667808:	cmp    r8,rdx
  66780b:	cmovb  rdx,r8
  66780f:	cmp    rbx,rdx
  667812:	cmovb  rdx,rbx
  667816:	test   rdx,rdx
  667819:	je     6679ea <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled12consume_four+0x21a>
  66781f:	shl    r11,0x8
  667823:	lea    r8,[rdi+r11*1]
  667827:	lea    rdi,[rdi+r11*1+0x200]
  66782f:	vmovdqu64 zmm4,ZMMWORD PTR [rdi]
  667835:	vmovdqu64 zmm5,ZMMWORD PTR [rdi+0x40]
  66783c:	vmovdqu64 zmm6,ZMMWORD PTR [rdi+0x80]
  667843:	vmovdqu64 zmm7,ZMMWORD PTR [rdi+0xc0]
  66784a:	vmovdqu64 zmm8,ZMMWORD PTR [r8+0x1c0]
  667851:	vmovdqu64 zmm10,ZMMWORD PTR [r8]
  667857:	vmovdqu64 zmm11,ZMMWORD PTR [r8+0x40]
  66785e:	vmovdqu64 zmm13,ZMMWORD PTR [r8+0x80]
  667865:	vmovdqu64 zmm14,ZMMWORD PTR [r8+0xc0]
  66786c:	vmovdqu64 zmm9,ZMMWORD PTR [r8+0x180]
  667873:	vmovdqu64 zmm12,ZMMWORD PTR [r8+0x140]
  66787a:	vmovdqu64 zmm15,ZMMWORD PTR [r8+0x100]
  667881:	add    rcx,r10
  667884:	shl    rdx,0x4
  667888:	xor    r10d,r10d
  66788b:	nop    DWORD PTR [rax+rax*1+0x0]
  667890:	vpbroadcastb zmm16,BYTE PTR [rcx+r10*1-0x3]
  667898:	kmovq  k2,QWORD PTR [r9+r10*1]
  66789e:	kmovq  k1,QWORD PTR [r9+r10*1+0x8]
  6678a5:	vmovdqu8 zmm17{k2}{z},zmm16
  6678ab:	vpxord zmm10,zmm10,zmm17
  6678b1:	vmovdqu8 zmm17{k1}{z},zmm16
  6678b7:	vpxord zmm11,zmm11,zmm17
  6678bd:	vmovdqa64 zmm17,ZMMWORD PTR [rsi+r10*8]
  6678c4:	vmovdqa64 zmm18,ZMMWORD PTR [rsi+r10*8+0x40]
  6678cc:	vgf2p8mulb zmm19,zmm17,zmm16
  6678d2:	vpxord zmm13,zmm13,zmm19
  6678d8:	vgf2p8mulb zmm16,zmm18,zmm16
  6678de:	vpxord zmm14,zmm14,zmm16
  6678e4:	vpbroadcastb zmm16,BYTE PTR [rcx+r10*1-0x2]
  6678ec:	vmovdqu8 zmm19{k2}{z},zmm16
  6678f2:	vpxord zmm15,zmm15,zmm19
  6678f8:	vmovdqu8 zmm19{k1}{z},zmm16
  6678fe:	vpxord zmm12,zmm12,zmm19
  667904:	vgf2p8mulb zmm19,zmm17,zmm16
  66790a:	vpxord zmm9,zmm9,zmm19
  667910:	vgf2p8mulb zmm16,zmm18,zmm16
  667916:	vpxord zmm8,zmm8,zmm16
  66791c:	vpbroadcastb zmm16,BYTE PTR [rcx+r10*1-0x1]
  667924:	vmovdqu8 zmm19{k2}{z},zmm16
  66792a:	vpxord zmm4,zmm4,zmm19
  667930:	vmovdqu8 zmm19{k1}{z},zmm16
  667936:	vpxord zmm5,zmm5,zmm19
  66793c:	vgf2p8mulb zmm19,zmm17,zmm16
  667942:	vpxord zmm6,zmm6,zmm19
  667948:	vgf2p8mulb zmm16,zmm18,zmm16
  66794e:	vpxord zmm7,zmm7,zmm16
  667954:	vpbroadcastb zmm16,BYTE PTR [rcx+r10*1]
  66795b:	vmovdqu8 zmm19{k2}{z},zmm16
  667961:	vpxord zmm0,zmm0,zmm19
  667967:	vmovdqu8 zmm19{k1}{z},zmm16
  66796d:	vpxord zmm1,zmm1,zmm19
  667973:	vgf2p8mulb zmm17,zmm17,zmm16
  667979:	vpxord zmm2,zmm2,zmm17
  66797f:	vgf2p8mulb zmm16,zmm18,zmm16
  667985:	vpxord zmm3,zmm3,zmm16
  66798b:	add    r10,0x10
  66798f:	cmp    rdx,r10
  667992:	jne    667890 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled12consume_four+0xc0>
  667998:	vmovdqu64 ZMMWORD PTR [r8],zmm10
  66799e:	vmovdqu64 ZMMWORD PTR [r8+0x40],zmm11
  6679a5:	vmovdqu64 ZMMWORD PTR [r8+0x80],zmm13
  6679ac:	vmovdqu64 ZMMWORD PTR [r8+0xc0],zmm14
  6679b3:	vmovdqu64 ZMMWORD PTR [r8+0x100],zmm15
  6679ba:	vmovdqu64 ZMMWORD PTR [r8+0x140],zmm12
  6679c1:	vmovdqu64 ZMMWORD PTR [r8+0x180],zmm9
  6679c8:	vmovdqu64 ZMMWORD PTR [r8+0x1c0],zmm8
  6679cf:	vmovdqu64 ZMMWORD PTR [rdi],zmm4
  6679d5:	vmovdqu64 ZMMWORD PTR [rdi+0x40],zmm5
  6679dc:	vmovdqu64 ZMMWORD PTR [rdi+0x80],zmm6
  6679e3:	vmovdqu64 ZMMWORD PTR [rdi+0xc0],zmm7
  6679ea:	vmovdqu64 ZMMWORD PTR [rax],zmm0
  6679f0:	vmovdqu64 ZMMWORD PTR [rax+0x40],zmm1
  6679f7:	vmovdqu64 ZMMWORD PTR [rax+0x80],zmm2
  6679fe:	vmovdqu64 ZMMWORD PTR [rax+0xc0],zmm3
  667a05:	pop    rbx
  667a06:	vzeroupper 
  667a09:	ret    
  667a0a:	int3   
  667a0b:	int3   
  667a0c:	int3   
  667a0d:	int3   
  667a0e:	int3   
  667a0f:	int3   

