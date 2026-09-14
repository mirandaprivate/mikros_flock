000000000067d280 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled12consume_four>:
  67d280:	53                   	push   rbx
  67d281:	48 8b 5c 24 10       	mov    rbx,QWORD PTR [rsp+0x10]
  67d286:	4c 8b 5c 24 18       	mov    r11,QWORD PTR [rsp+0x18]
  67d28b:	4d 8d 53 03          	lea    r10,[r11+0x3]
  67d28f:	4c 89 d0             	mov    rax,r10
  67d292:	48 c1 e0 08          	shl    rax,0x8
  67d296:	62 f1 fe 48 6f 04 07 	vmovdqu64 zmm0,ZMMWORD PTR [rdi+rax*1]
  67d29d:	62 f1 fe 48 6f 4c 07 	vmovdqu64 zmm1,ZMMWORD PTR [rdi+rax*1+0x40]
  67d2a4:	01 
  67d2a5:	62 f1 fe 48 6f 54 07 	vmovdqu64 zmm2,ZMMWORD PTR [rdi+rax*1+0x80]
  67d2ac:	02 
  67d2ad:	62 f1 fe 48 6f 5c 07 	vmovdqu64 zmm3,ZMMWORD PTR [rdi+rax*1+0xc0]
  67d2b4:	03 
  67d2b5:	48 01 f8             	add    rax,rdi
  67d2b8:	49 39 d0             	cmp    r8,rdx
  67d2bb:	49 0f 42 d0          	cmovb  rdx,r8
  67d2bf:	48 39 d3             	cmp    rbx,rdx
  67d2c2:	48 0f 42 d3          	cmovb  rdx,rbx
  67d2c6:	48 85 d2             	test   rdx,rdx
  67d2c9:	0f 84 cb 01 00 00    	je     67d49a <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled12consume_four+0x21a>
  67d2cf:	49 c1 e3 08          	shl    r11,0x8
  67d2d3:	4e 8d 04 1f          	lea    r8,[rdi+r11*1]
  67d2d7:	4a 8d bc 1f 00 02 00 	lea    rdi,[rdi+r11*1+0x200]
  67d2de:	00 
  67d2df:	62 f1 fe 48 6f 27    	vmovdqu64 zmm4,ZMMWORD PTR [rdi]
  67d2e5:	62 f1 fe 48 6f 6f 01 	vmovdqu64 zmm5,ZMMWORD PTR [rdi+0x40]
  67d2ec:	62 f1 fe 48 6f 77 02 	vmovdqu64 zmm6,ZMMWORD PTR [rdi+0x80]
  67d2f3:	62 f1 fe 48 6f 7f 03 	vmovdqu64 zmm7,ZMMWORD PTR [rdi+0xc0]
  67d2fa:	62 51 fe 48 6f 40 07 	vmovdqu64 zmm8,ZMMWORD PTR [r8+0x1c0]
  67d301:	62 51 fe 48 6f 10    	vmovdqu64 zmm10,ZMMWORD PTR [r8]
  67d307:	62 51 fe 48 6f 58 01 	vmovdqu64 zmm11,ZMMWORD PTR [r8+0x40]
  67d30e:	62 51 fe 48 6f 68 02 	vmovdqu64 zmm13,ZMMWORD PTR [r8+0x80]
  67d315:	62 51 fe 48 6f 70 03 	vmovdqu64 zmm14,ZMMWORD PTR [r8+0xc0]
  67d31c:	62 51 fe 48 6f 48 06 	vmovdqu64 zmm9,ZMMWORD PTR [r8+0x180]
  67d323:	62 51 fe 48 6f 60 05 	vmovdqu64 zmm12,ZMMWORD PTR [r8+0x140]
  67d32a:	62 51 fe 48 6f 78 04 	vmovdqu64 zmm15,ZMMWORD PTR [r8+0x100]
  67d331:	4c 01 d1             	add    rcx,r10
  67d334:	48 c1 e2 04          	shl    rdx,0x4
  67d338:	45 31 d2             	xor    r10d,r10d
  67d33b:	0f 1f 44 00 00       	nop    DWORD PTR [rax+rax*1+0x0]
  67d340:	62 a2 7d 48 78 44 11 	vpbroadcastb zmm16,BYTE PTR [rcx+r10*1-0x3]
  67d347:	fd 
  67d348:	c4 81 f8 90 14 11    	kmovq  k2,QWORD PTR [r9+r10*1]
  67d34e:	c4 81 f8 90 4c 11 08 	kmovq  k1,QWORD PTR [r9+r10*1+0x8]
  67d355:	62 a1 7f ca 6f c8    	vmovdqu8 zmm17{k2}{z},zmm16
  67d35b:	62 31 2d 48 ef d1    	vpxord zmm10,zmm10,zmm17
  67d361:	62 a1 7f c9 6f c8    	vmovdqu8 zmm17{k1}{z},zmm16
  67d367:	62 31 25 48 ef d9    	vpxord zmm11,zmm11,zmm17
  67d36d:	62 a1 fd 48 6f 0c d6 	vmovdqa64 zmm17,ZMMWORD PTR [rsi+r10*8]
  67d374:	62 a1 fd 48 6f 54 d6 	vmovdqa64 zmm18,ZMMWORD PTR [rsi+r10*8+0x40]
  67d37b:	01 
  67d37c:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  67d382:	62 31 15 48 ef eb    	vpxord zmm13,zmm13,zmm19
  67d388:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  67d38e:	62 31 0d 48 ef f0    	vpxord zmm14,zmm14,zmm16
  67d394:	62 a2 7d 48 78 44 11 	vpbroadcastb zmm16,BYTE PTR [rcx+r10*1-0x2]
  67d39b:	fe 
  67d39c:	62 a1 7f ca 6f d8    	vmovdqu8 zmm19{k2}{z},zmm16
  67d3a2:	62 31 05 48 ef fb    	vpxord zmm15,zmm15,zmm19
  67d3a8:	62 a1 7f c9 6f d8    	vmovdqu8 zmm19{k1}{z},zmm16
  67d3ae:	62 31 1d 48 ef e3    	vpxord zmm12,zmm12,zmm19
  67d3b4:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  67d3ba:	62 31 35 48 ef cb    	vpxord zmm9,zmm9,zmm19
  67d3c0:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  67d3c6:	62 31 3d 48 ef c0    	vpxord zmm8,zmm8,zmm16
  67d3cc:	62 a2 7d 48 78 44 11 	vpbroadcastb zmm16,BYTE PTR [rcx+r10*1-0x1]
  67d3d3:	ff 
  67d3d4:	62 a1 7f ca 6f d8    	vmovdqu8 zmm19{k2}{z},zmm16
  67d3da:	62 b1 5d 48 ef e3    	vpxord zmm4,zmm4,zmm19
  67d3e0:	62 a1 7f c9 6f d8    	vmovdqu8 zmm19{k1}{z},zmm16
  67d3e6:	62 b1 55 48 ef eb    	vpxord zmm5,zmm5,zmm19
  67d3ec:	62 a2 75 40 cf d8    	vgf2p8mulb zmm19,zmm17,zmm16
  67d3f2:	62 b1 4d 48 ef f3    	vpxord zmm6,zmm6,zmm19
  67d3f8:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  67d3fe:	62 b1 45 48 ef f8    	vpxord zmm7,zmm7,zmm16
  67d404:	62 a2 7d 48 78 04 11 	vpbroadcastb zmm16,BYTE PTR [rcx+r10*1]
  67d40b:	62 a1 7f ca 6f d8    	vmovdqu8 zmm19{k2}{z},zmm16
  67d411:	62 b1 7d 48 ef c3    	vpxord zmm0,zmm0,zmm19
  67d417:	62 a1 7f c9 6f d8    	vmovdqu8 zmm19{k1}{z},zmm16
  67d41d:	62 b1 75 48 ef cb    	vpxord zmm1,zmm1,zmm19
  67d423:	62 a2 75 40 cf c8    	vgf2p8mulb zmm17,zmm17,zmm16
  67d429:	62 b1 6d 48 ef d1    	vpxord zmm2,zmm2,zmm17
  67d42f:	62 a2 6d 40 cf c0    	vgf2p8mulb zmm16,zmm18,zmm16
  67d435:	62 b1 65 48 ef d8    	vpxord zmm3,zmm3,zmm16
  67d43b:	49 83 c2 10          	add    r10,0x10
  67d43f:	4c 39 d2             	cmp    rdx,r10
  67d442:	0f 85 f8 fe ff ff    	jne    67d340 <_RNvNtNtCs5Palesvl9wF_6mikros10univariate5tiled12consume_four+0xc0>
  67d448:	62 51 fe 48 7f 10    	vmovdqu64 ZMMWORD PTR [r8],zmm10
  67d44e:	62 51 fe 48 7f 58 01 	vmovdqu64 ZMMWORD PTR [r8+0x40],zmm11
  67d455:	62 51 fe 48 7f 68 02 	vmovdqu64 ZMMWORD PTR [r8+0x80],zmm13
  67d45c:	62 51 fe 48 7f 70 03 	vmovdqu64 ZMMWORD PTR [r8+0xc0],zmm14
  67d463:	62 51 fe 48 7f 78 04 	vmovdqu64 ZMMWORD PTR [r8+0x100],zmm15
  67d46a:	62 51 fe 48 7f 60 05 	vmovdqu64 ZMMWORD PTR [r8+0x140],zmm12
  67d471:	62 51 fe 48 7f 48 06 	vmovdqu64 ZMMWORD PTR [r8+0x180],zmm9
  67d478:	62 51 fe 48 7f 40 07 	vmovdqu64 ZMMWORD PTR [r8+0x1c0],zmm8
  67d47f:	62 f1 fe 48 7f 27    	vmovdqu64 ZMMWORD PTR [rdi],zmm4
  67d485:	62 f1 fe 48 7f 6f 01 	vmovdqu64 ZMMWORD PTR [rdi+0x40],zmm5
  67d48c:	62 f1 fe 48 7f 77 02 	vmovdqu64 ZMMWORD PTR [rdi+0x80],zmm6
  67d493:	62 f1 fe 48 7f 7f 03 	vmovdqu64 ZMMWORD PTR [rdi+0xc0],zmm7
  67d49a:	62 f1 fe 48 7f 00    	vmovdqu64 ZMMWORD PTR [rax],zmm0
  67d4a0:	62 f1 fe 48 7f 48 01 	vmovdqu64 ZMMWORD PTR [rax+0x40],zmm1
  67d4a7:	62 f1 fe 48 7f 50 02 	vmovdqu64 ZMMWORD PTR [rax+0x80],zmm2
  67d4ae:	62 f1 fe 48 7f 58 03 	vmovdqu64 ZMMWORD PTR [rax+0xc0],zmm3
  67d4b5:	5b                   	pop    rbx
  67d4b6:	c5 f8 77             	vzeroupper 
  67d4b9:	c3                   	ret    
  67d4ba:	cc                   	int3   
  67d4bb:	cc                   	int3   
  67d4bc:	cc                   	int3   
  67d4bd:	cc                   	int3   
  67d4be:	cc                   	int3   
  67d4bf:	cc                   	int3   

