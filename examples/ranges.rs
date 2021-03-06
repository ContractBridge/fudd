use fudd::types::ranges::two_cards_set::TwoCardsSet;
use itertools::Itertools;

fn main() {
    let every = TwoCardsSet::every().two_cards_vec();
    let combos = every.iter().combinations(2);

    for (i, hand) in every.iter().enumerate() {
        println!("{} {}", i + 1, hand);
    }

    for (i, hands) in combos.into_iter().enumerate() {
        let hero = *hands.get(0).unwrap();
        let villain = *hands.get(1).unwrap();
        println!("{} {} - {}", i + 1, hero, villain);
    }
}

// 0 A♠ A♥
// 1 A♠ A♦
// 2 A♠ A♣
// 3 A♠ K♠
// 4 A♠ K♥
// 5 A♠ K♦
// 6 A♠ K♣
// 7 A♠ Q♠
// 8 A♠ Q♥
// 9 A♠ Q♦
// 10 A♠ Q♣
// 11 A♠ J♠
// 12 A♠ J♥
// 13 A♠ J♦
// 14 A♠ J♣
// 15 A♠ T♠
// 16 A♠ T♥
// 17 A♠ T♦
// 18 A♠ T♣
// 19 A♠ 9♠
// 20 A♠ 9♥
// 21 A♠ 9♦
// 22 A♠ 9♣
// 23 A♠ 8♠
// 24 A♠ 8♥
// 25 A♠ 8♦
// 26 A♠ 8♣
// 27 A♠ 7♠
// 28 A♠ 7♥
// 29 A♠ 7♦
// 30 A♠ 7♣
// 31 A♠ 6♠
// 32 A♠ 6♥
// 33 A♠ 6♦
// 34 A♠ 6♣
// 35 A♠ 5♠
// 36 A♠ 5♥
// 37 A♠ 5♦
// 38 A♠ 5♣
// 39 A♠ 4♠
// 40 A♠ 4♥
// 41 A♠ 4♦
// 42 A♠ 4♣
// 43 A♠ 3♠
// 44 A♠ 3♥
// 45 A♠ 3♦
// 46 A♠ 3♣
// 47 A♠ 2♠
// 48 A♠ 2♥
// 49 A♠ 2♦
// 50 A♠ 2♣
// 51 A♥ A♦
// 52 A♥ A♣
// 53 A♥ K♠
// 54 A♥ K♥
// 55 A♥ K♦
// 56 A♥ K♣
// 57 A♥ Q♠
// 58 A♥ Q♥
// 59 A♥ Q♦
// 60 A♥ Q♣
// 61 A♥ J♠
// 62 A♥ J♥
// 63 A♥ J♦
// 64 A♥ J♣
// 65 A♥ T♠
// 66 A♥ T♥
// 67 A♥ T♦
// 68 A♥ T♣
// 69 A♥ 9♠
// 70 A♥ 9♥
// 71 A♥ 9♦
// 72 A♥ 9♣
// 73 A♥ 8♠
// 74 A♥ 8♥
// 75 A♥ 8♦
// 76 A♥ 8♣
// 77 A♥ 7♠
// 78 A♥ 7♥
// 79 A♥ 7♦
// 80 A♥ 7♣
// 81 A♥ 6♠
// 82 A♥ 6♥
// 83 A♥ 6♦
// 84 A♥ 6♣
// 85 A♥ 5♠
// 86 A♥ 5♥
// 87 A♥ 5♦
// 88 A♥ 5♣
// 89 A♥ 4♠
// 90 A♥ 4♥
// 91 A♥ 4♦
// 92 A♥ 4♣
// 93 A♥ 3♠
// 94 A♥ 3♥
// 95 A♥ 3♦
// 96 A♥ 3♣
// 97 A♥ 2♠
// 98 A♥ 2♥
// 99 A♥ 2♦
// 100 A♥ 2♣
// 101 A♦ A♣
// 102 A♦ K♠
// 103 A♦ K♥
// 104 A♦ K♦
// 105 A♦ K♣
// 106 A♦ Q♠
// 107 A♦ Q♥
// 108 A♦ Q♦
// 109 A♦ Q♣
// 110 A♦ J♠
// 111 A♦ J♥
// 112 A♦ J♦
// 113 A♦ J♣
// 114 A♦ T♠
// 115 A♦ T♥
// 116 A♦ T♦
// 117 A♦ T♣
// 118 A♦ 9♠
// 119 A♦ 9♥
// 120 A♦ 9♦
// 121 A♦ 9♣
// 122 A♦ 8♠
// 123 A♦ 8♥
// 124 A♦ 8♦
// 125 A♦ 8♣
// 126 A♦ 7♠
// 127 A♦ 7♥
// 128 A♦ 7♦
// 129 A♦ 7♣
// 130 A♦ 6♠
// 131 A♦ 6♥
// 132 A♦ 6♦
// 133 A♦ 6♣
// 134 A♦ 5♠
// 135 A♦ 5♥
// 136 A♦ 5♦
// 137 A♦ 5♣
// 138 A♦ 4♠
// 139 A♦ 4♥
// 140 A♦ 4♦
// 141 A♦ 4♣
// 142 A♦ 3♠
// 143 A♦ 3♥
// 144 A♦ 3♦
// 145 A♦ 3♣
// 146 A♦ 2♠
// 147 A♦ 2♥
// 148 A♦ 2♦
// 149 A♦ 2♣
// 150 A♣ K♠
// 151 A♣ K♥
// 152 A♣ K♦
// 153 A♣ K♣
// 154 A♣ Q♠
// 155 A♣ Q♥
// 156 A♣ Q♦
// 157 A♣ Q♣
// 158 A♣ J♠
// 159 A♣ J♥
// 160 A♣ J♦
// 161 A♣ J♣
// 162 A♣ T♠
// 163 A♣ T♥
// 164 A♣ T♦
// 165 A♣ T♣
// 166 A♣ 9♠
// 167 A♣ 9♥
// 168 A♣ 9♦
// 169 A♣ 9♣
// 170 A♣ 8♠
// 171 A♣ 8♥
// 172 A♣ 8♦
// 173 A♣ 8♣
// 174 A♣ 7♠
// 175 A♣ 7♥
// 176 A♣ 7♦
// 177 A♣ 7♣
// 178 A♣ 6♠
// 179 A♣ 6♥
// 180 A♣ 6♦
// 181 A♣ 6♣
// 182 A♣ 5♠
// 183 A♣ 5♥
// 184 A♣ 5♦
// 185 A♣ 5♣
// 186 A♣ 4♠
// 187 A♣ 4♥
// 188 A♣ 4♦
// 189 A♣ 4♣
// 190 A♣ 3♠
// 191 A♣ 3♥
// 192 A♣ 3♦
// 193 A♣ 3♣
// 194 A♣ 2♠
// 195 A♣ 2♥
// 196 A♣ 2♦
// 197 A♣ 2♣
// 198 K♠ K♥
// 199 K♠ K♦
// 200 K♠ K♣
// 201 K♠ Q♠
// 202 K♠ Q♥
// 203 K♠ Q♦
// 204 K♠ Q♣
// 205 K♠ J♠
// 206 K♠ J♥
// 207 K♠ J♦
// 208 K♠ J♣
// 209 K♠ T♠
// 210 K♠ T♥
// 211 K♠ T♦
// 212 K♠ T♣
// 213 K♠ 9♠
// 214 K♠ 9♥
// 215 K♠ 9♦
// 216 K♠ 9♣
// 217 K♠ 8♠
// 218 K♠ 8♥
// 219 K♠ 8♦
// 220 K♠ 8♣
// 221 K♠ 7♠
// 222 K♠ 7♥
// 223 K♠ 7♦
// 224 K♠ 7♣
// 225 K♠ 6♠
// 226 K♠ 6♥
// 227 K♠ 6♦
// 228 K♠ 6♣
// 229 K♠ 5♠
// 230 K♠ 5♥
// 231 K♠ 5♦
// 232 K♠ 5♣
// 233 K♠ 4♠
// 234 K♠ 4♥
// 235 K♠ 4♦
// 236 K♠ 4♣
// 237 K♠ 3♠
// 238 K♠ 3♥
// 239 K♠ 3♦
// 240 K♠ 3♣
// 241 K♠ 2♠
// 242 K♠ 2♥
// 243 K♠ 2♦
// 244 K♠ 2♣
// 245 K♥ K♦
// 246 K♥ K♣
// 247 K♥ Q♠
// 248 K♥ Q♥
// 249 K♥ Q♦
// 250 K♥ Q♣
// 251 K♥ J♠
// 252 K♥ J♥
// 253 K♥ J♦
// 254 K♥ J♣
// 255 K♥ T♠
// 256 K♥ T♥
// 257 K♥ T♦
// 258 K♥ T♣
// 259 K♥ 9♠
// 260 K♥ 9♥
// 261 K♥ 9♦
// 262 K♥ 9♣
// 263 K♥ 8♠
// 264 K♥ 8♥
// 265 K♥ 8♦
// 266 K♥ 8♣
// 267 K♥ 7♠
// 268 K♥ 7♥
// 269 K♥ 7♦
// 270 K♥ 7♣
// 271 K♥ 6♠
// 272 K♥ 6♥
// 273 K♥ 6♦
// 274 K♥ 6♣
// 275 K♥ 5♠
// 276 K♥ 5♥
// 277 K♥ 5♦
// 278 K♥ 5♣
// 279 K♥ 4♠
// 280 K♥ 4♥
// 281 K♥ 4♦
// 282 K♥ 4♣
// 283 K♥ 3♠
// 284 K♥ 3♥
// 285 K♥ 3♦
// 286 K♥ 3♣
// 287 K♥ 2♠
// 288 K♥ 2♥
// 289 K♥ 2♦
// 290 K♥ 2♣
// 291 K♦ K♣
// 292 K♦ Q♠
// 293 K♦ Q♥
// 294 K♦ Q♦
// 295 K♦ Q♣
// 296 K♦ J♠
// 297 K♦ J♥
// 298 K♦ J♦
// 299 K♦ J♣
// 300 K♦ T♠
// 301 K♦ T♥
// 302 K♦ T♦
// 303 K♦ T♣
// 304 K♦ 9♠
// 305 K♦ 9♥
// 306 K♦ 9♦
// 307 K♦ 9♣
// 308 K♦ 8♠
// 309 K♦ 8♥
// 310 K♦ 8♦
// 311 K♦ 8♣
// 312 K♦ 7♠
// 313 K♦ 7♥
// 314 K♦ 7♦
// 315 K♦ 7♣
// 316 K♦ 6♠
// 317 K♦ 6♥
// 318 K♦ 6♦
// 319 K♦ 6♣
// 320 K♦ 5♠
// 321 K♦ 5♥
// 322 K♦ 5♦
// 323 K♦ 5♣
// 324 K♦ 4♠
// 325 K♦ 4♥
// 326 K♦ 4♦
// 327 K♦ 4♣
// 328 K♦ 3♠
// 329 K♦ 3♥
// 330 K♦ 3♦
// 331 K♦ 3♣
// 332 K♦ 2♠
// 333 K♦ 2♥
// 334 K♦ 2♦
// 335 K♦ 2♣
// 336 K♣ Q♠
// 337 K♣ Q♥
// 338 K♣ Q♦
// 339 K♣ Q♣
// 340 K♣ J♠
// 341 K♣ J♥
// 342 K♣ J♦
// 343 K♣ J♣
// 344 K♣ T♠
// 345 K♣ T♥
// 346 K♣ T♦
// 347 K♣ T♣
// 348 K♣ 9♠
// 349 K♣ 9♥
// 350 K♣ 9♦
// 351 K♣ 9♣
// 352 K♣ 8♠
// 353 K♣ 8♥
// 354 K♣ 8♦
// 355 K♣ 8♣
// 356 K♣ 7♠
// 357 K♣ 7♥
// 358 K♣ 7♦
// 359 K♣ 7♣
// 360 K♣ 6♠
// 361 K♣ 6♥
// 362 K♣ 6♦
// 363 K♣ 6♣
// 364 K♣ 5♠
// 365 K♣ 5♥
// 366 K♣ 5♦
// 367 K♣ 5♣
// 368 K♣ 4♠
// 369 K♣ 4♥
// 370 K♣ 4♦
// 371 K♣ 4♣
// 372 K♣ 3♠
// 373 K♣ 3♥
// 374 K♣ 3♦
// 375 K♣ 3♣
// 376 K♣ 2♠
// 377 K♣ 2♥
// 378 K♣ 2♦
// 379 K♣ 2♣
// 380 Q♠ Q♥
// 381 Q♠ Q♦
// 382 Q♠ Q♣
// 383 Q♠ J♠
// 384 Q♠ J♥
// 385 Q♠ J♦
// 386 Q♠ J♣
// 387 Q♠ T♠
// 388 Q♠ T♥
// 389 Q♠ T♦
// 390 Q♠ T♣
// 391 Q♠ 9♠
// 392 Q♠ 9♥
// 393 Q♠ 9♦
// 394 Q♠ 9♣
// 395 Q♠ 8♠
// 396 Q♠ 8♥
// 397 Q♠ 8♦
// 398 Q♠ 8♣
// 399 Q♠ 7♠
// 400 Q♠ 7♥
// 401 Q♠ 7♦
// 402 Q♠ 7♣
// 403 Q♠ 6♠
// 404 Q♠ 6♥
// 405 Q♠ 6♦
// 406 Q♠ 6♣
// 407 Q♠ 5♠
// 408 Q♠ 5♥
// 409 Q♠ 5♦
// 410 Q♠ 5♣
// 411 Q♠ 4♠
// 412 Q♠ 4♥
// 413 Q♠ 4♦
// 414 Q♠ 4♣
// 415 Q♠ 3♠
// 416 Q♠ 3♥
// 417 Q♠ 3♦
// 418 Q♠ 3♣
// 419 Q♠ 2♠
// 420 Q♠ 2♥
// 421 Q♠ 2♦
// 422 Q♠ 2♣
// 423 Q♥ Q♦
// 424 Q♥ Q♣
// 425 Q♥ J♠
// 426 Q♥ J♥
// 427 Q♥ J♦
// 428 Q♥ J♣
// 429 Q♥ T♠
// 430 Q♥ T♥
// 431 Q♥ T♦
// 432 Q♥ T♣
// 433 Q♥ 9♠
// 434 Q♥ 9♥
// 435 Q♥ 9♦
// 436 Q♥ 9♣
// 437 Q♥ 8♠
// 438 Q♥ 8♥
// 439 Q♥ 8♦
// 440 Q♥ 8♣
// 441 Q♥ 7♠
// 442 Q♥ 7♥
// 443 Q♥ 7♦
// 444 Q♥ 7♣
// 445 Q♥ 6♠
// 446 Q♥ 6♥
// 447 Q♥ 6♦
// 448 Q♥ 6♣
// 449 Q♥ 5♠
// 450 Q♥ 5♥
// 451 Q♥ 5♦
// 452 Q♥ 5♣
// 453 Q♥ 4♠
// 454 Q♥ 4♥
// 455 Q♥ 4♦
// 456 Q♥ 4♣
// 457 Q♥ 3♠
// 458 Q♥ 3♥
// 459 Q♥ 3♦
// 460 Q♥ 3♣
// 461 Q♥ 2♠
// 462 Q♥ 2♥
// 463 Q♥ 2♦
// 464 Q♥ 2♣
// 465 Q♦ Q♣
// 466 Q♦ J♠
// 467 Q♦ J♥
// 468 Q♦ J♦
// 469 Q♦ J♣
// 470 Q♦ T♠
// 471 Q♦ T♥
// 472 Q♦ T♦
// 473 Q♦ T♣
// 474 Q♦ 9♠
// 475 Q♦ 9♥
// 476 Q♦ 9♦
// 477 Q♦ 9♣
// 478 Q♦ 8♠
// 479 Q♦ 8♥
// 480 Q♦ 8♦
// 481 Q♦ 8♣
// 482 Q♦ 7♠
// 483 Q♦ 7♥
// 484 Q♦ 7♦
// 485 Q♦ 7♣
// 486 Q♦ 6♠
// 487 Q♦ 6♥
// 488 Q♦ 6♦
// 489 Q♦ 6♣
// 490 Q♦ 5♠
// 491 Q♦ 5♥
// 492 Q♦ 5♦
// 493 Q♦ 5♣
// 494 Q♦ 4♠
// 495 Q♦ 4♥
// 496 Q♦ 4♦
// 497 Q♦ 4♣
// 498 Q♦ 3♠
// 499 Q♦ 3♥
// 500 Q♦ 3♦
// 501 Q♦ 3♣
// 502 Q♦ 2♠
// 503 Q♦ 2♥
// 504 Q♦ 2♦
// 505 Q♦ 2♣
// 506 Q♣ J♠
// 507 Q♣ J♥
// 508 Q♣ J♦
// 509 Q♣ J♣
// 510 Q♣ T♠
// 511 Q♣ T♥
// 512 Q♣ T♦
// 513 Q♣ T♣
// 514 Q♣ 9♠
// 515 Q♣ 9♥
// 516 Q♣ 9♦
// 517 Q♣ 9♣
// 518 Q♣ 8♠
// 519 Q♣ 8♥
// 520 Q♣ 8♦
// 521 Q♣ 8♣
// 522 Q♣ 7♠
// 523 Q♣ 7♥
// 524 Q♣ 7♦
// 525 Q♣ 7♣
// 526 Q♣ 6♠
// 527 Q♣ 6♥
// 528 Q♣ 6♦
// 529 Q♣ 6♣
// 530 Q♣ 5♠
// 531 Q♣ 5♥
// 532 Q♣ 5♦
// 533 Q♣ 5♣
// 534 Q♣ 4♠
// 535 Q♣ 4♥
// 536 Q♣ 4♦
// 537 Q♣ 4♣
// 538 Q♣ 3♠
// 539 Q♣ 3♥
// 540 Q♣ 3♦
// 541 Q♣ 3♣
// 542 Q♣ 2♠
// 543 Q♣ 2♥
// 544 Q♣ 2♦
// 545 Q♣ 2♣
// 546 J♠ J♥
// 547 J♠ J♦
// 548 J♠ J♣
// 549 J♠ T♠
// 550 J♠ T♥
// 551 J♠ T♦
// 552 J♠ T♣
// 553 J♠ 9♠
// 554 J♠ 9♥
// 555 J♠ 9♦
// 556 J♠ 9♣
// 557 J♠ 8♠
// 558 J♠ 8♥
// 559 J♠ 8♦
// 560 J♠ 8♣
// 561 J♠ 7♠
// 562 J♠ 7♥
// 563 J♠ 7♦
// 564 J♠ 7♣
// 565 J♠ 6♠
// 566 J♠ 6♥
// 567 J♠ 6♦
// 568 J♠ 6♣
// 569 J♠ 5♠
// 570 J♠ 5♥
// 571 J♠ 5♦
// 572 J♠ 5♣
// 573 J♠ 4♠
// 574 J♠ 4♥
// 575 J♠ 4♦
// 576 J♠ 4♣
// 577 J♠ 3♠
// 578 J♠ 3♥
// 579 J♠ 3♦
// 580 J♠ 3♣
// 581 J♠ 2♠
// 582 J♠ 2♥
// 583 J♠ 2♦
// 584 J♠ 2♣
// 585 J♥ J♦
// 586 J♥ J♣
// 587 J♥ T♠
// 588 J♥ T♥
// 589 J♥ T♦
// 590 J♥ T♣
// 591 J♥ 9♠
// 592 J♥ 9♥
// 593 J♥ 9♦
// 594 J♥ 9♣
// 595 J♥ 8♠
// 596 J♥ 8♥
// 597 J♥ 8♦
// 598 J♥ 8♣
// 599 J♥ 7♠
// 600 J♥ 7♥
// 601 J♥ 7♦
// 602 J♥ 7♣
// 603 J♥ 6♠
// 604 J♥ 6♥
// 605 J♥ 6♦
// 606 J♥ 6♣
// 607 J♥ 5♠
// 608 J♥ 5♥
// 609 J♥ 5♦
// 610 J♥ 5♣
// 611 J♥ 4♠
// 612 J♥ 4♥
// 613 J♥ 4♦
// 614 J♥ 4♣
// 615 J♥ 3♠
// 616 J♥ 3♥
// 617 J♥ 3♦
// 618 J♥ 3♣
// 619 J♥ 2♠
// 620 J♥ 2♥
// 621 J♥ 2♦
// 622 J♥ 2♣
// 623 J♦ J♣
// 624 J♦ T♠
// 625 J♦ T♥
// 626 J♦ T♦
// 627 J♦ T♣
// 628 J♦ 9♠
// 629 J♦ 9♥
// 630 J♦ 9♦
// 631 J♦ 9♣
// 632 J♦ 8♠
// 633 J♦ 8♥
// 634 J♦ 8♦
// 635 J♦ 8♣
// 636 J♦ 7♠
// 637 J♦ 7♥
// 638 J♦ 7♦
// 639 J♦ 7♣
// 640 J♦ 6♠
// 641 J♦ 6♥
// 642 J♦ 6♦
// 643 J♦ 6♣
// 644 J♦ 5♠
// 645 J♦ 5♥
// 646 J♦ 5♦
// 647 J♦ 5♣
// 648 J♦ 4♠
// 649 J♦ 4♥
// 650 J♦ 4♦
// 651 J♦ 4♣
// 652 J♦ 3♠
// 653 J♦ 3♥
// 654 J♦ 3♦
// 655 J♦ 3♣
// 656 J♦ 2♠
// 657 J♦ 2♥
// 658 J♦ 2♦
// 659 J♦ 2♣
// 660 J♣ T♠
// 661 J♣ T♥
// 662 J♣ T♦
// 663 J♣ T♣
// 664 J♣ 9♠
// 665 J♣ 9♥
// 666 J♣ 9♦
// 667 J♣ 9♣
// 668 J♣ 8♠
// 669 J♣ 8♥
// 670 J♣ 8♦
// 671 J♣ 8♣
// 672 J♣ 7♠
// 673 J♣ 7♥
// 674 J♣ 7♦
// 675 J♣ 7♣
// 676 J♣ 6♠
// 677 J♣ 6♥
// 678 J♣ 6♦
// 679 J♣ 6♣
// 680 J♣ 5♠
// 681 J♣ 5♥
// 682 J♣ 5♦
// 683 J♣ 5♣
// 684 J♣ 4♠
// 685 J♣ 4♥
// 686 J♣ 4♦
// 687 J♣ 4♣
// 688 J♣ 3♠
// 689 J♣ 3♥
// 690 J♣ 3♦
// 691 J♣ 3♣
// 692 J♣ 2♠
// 693 J♣ 2♥
// 694 J♣ 2♦
// 695 J♣ 2♣
// 696 T♠ T♥
// 697 T♠ T♦
// 698 T♠ T♣
// 699 T♠ 9♠
// 700 T♠ 9♥
// 701 T♠ 9♦
// 702 T♠ 9♣
// 703 T♠ 8♠
// 704 T♠ 8♥
// 705 T♠ 8♦
// 706 T♠ 8♣
// 707 T♠ 7♠
// 708 T♠ 7♥
// 709 T♠ 7♦
// 710 T♠ 7♣
// 711 T♠ 6♠
// 712 T♠ 6♥
// 713 T♠ 6♦
// 714 T♠ 6♣
// 715 T♠ 5♠
// 716 T♠ 5♥
// 717 T♠ 5♦
// 718 T♠ 5♣
// 719 T♠ 4♠
// 720 T♠ 4♥
// 721 T♠ 4♦
// 722 T♠ 4♣
// 723 T♠ 3♠
// 724 T♠ 3♥
// 725 T♠ 3♦
// 726 T♠ 3♣
// 727 T♠ 2♠
// 728 T♠ 2♥
// 729 T♠ 2♦
// 730 T♠ 2♣
// 731 T♥ T♦
// 732 T♥ T♣
// 733 T♥ 9♠
// 734 T♥ 9♥
// 735 T♥ 9♦
// 736 T♥ 9♣
// 737 T♥ 8♠
// 738 T♥ 8♥
// 739 T♥ 8♦
// 740 T♥ 8♣
// 741 T♥ 7♠
// 742 T♥ 7♥
// 743 T♥ 7♦
// 744 T♥ 7♣
// 745 T♥ 6♠
// 746 T♥ 6♥
// 747 T♥ 6♦
// 748 T♥ 6♣
// 749 T♥ 5♠
// 750 T♥ 5♥
// 751 T♥ 5♦
// 752 T♥ 5♣
// 753 T♥ 4♠
// 754 T♥ 4♥
// 755 T♥ 4♦
// 756 T♥ 4♣
// 757 T♥ 3♠
// 758 T♥ 3♥
// 759 T♥ 3♦
// 760 T♥ 3♣
// 761 T♥ 2♠
// 762 T♥ 2♥
// 763 T♥ 2♦
// 764 T♥ 2♣
// 765 T♦ T♣
// 766 T♦ 9♠
// 767 T♦ 9♥
// 768 T♦ 9♦
// 769 T♦ 9♣
// 770 T♦ 8♠
// 771 T♦ 8♥
// 772 T♦ 8♦
// 773 T♦ 8♣
// 774 T♦ 7♠
// 775 T♦ 7♥
// 776 T♦ 7♦
// 777 T♦ 7♣
// 778 T♦ 6♠
// 779 T♦ 6♥
// 780 T♦ 6♦
// 781 T♦ 6♣
// 782 T♦ 5♠
// 783 T♦ 5♥
// 784 T♦ 5♦
// 785 T♦ 5♣
// 786 T♦ 4♠
// 787 T♦ 4♥
// 788 T♦ 4♦
// 789 T♦ 4♣
// 790 T♦ 3♠
// 791 T♦ 3♥
// 792 T♦ 3♦
// 793 T♦ 3♣
// 794 T♦ 2♠
// 795 T♦ 2♥
// 796 T♦ 2♦
// 797 T♦ 2♣
// 798 T♣ 9♠
// 799 T♣ 9♥
// 800 T♣ 9♦
// 801 T♣ 9♣
// 802 T♣ 8♠
// 803 T♣ 8♥
// 804 T♣ 8♦
// 805 T♣ 8♣
// 806 T♣ 7♠
// 807 T♣ 7♥
// 808 T♣ 7♦
// 809 T♣ 7♣
// 810 T♣ 6♠
// 811 T♣ 6♥
// 812 T♣ 6♦
// 813 T♣ 6♣
// 814 T♣ 5♠
// 815 T♣ 5♥
// 816 T♣ 5♦
// 817 T♣ 5♣
// 818 T♣ 4♠
// 819 T♣ 4♥
// 820 T♣ 4♦
// 821 T♣ 4♣
// 822 T♣ 3♠
// 823 T♣ 3♥
// 824 T♣ 3♦
// 825 T♣ 3♣
// 826 T♣ 2♠
// 827 T♣ 2♥
// 828 T♣ 2♦
// 829 T♣ 2♣
// 830 9♠ 9♥
// 831 9♠ 9♦
// 832 9♠ 9♣
// 833 9♠ 8♠
// 834 9♠ 8♥
// 835 9♠ 8♦
// 836 9♠ 8♣
// 837 9♠ 7♠
// 838 9♠ 7♥
// 839 9♠ 7♦
// 840 9♠ 7♣
// 841 9♠ 6♠
// 842 9♠ 6♥
// 843 9♠ 6♦
// 844 9♠ 6♣
// 845 9♠ 5♠
// 846 9♠ 5♥
// 847 9♠ 5♦
// 848 9♠ 5♣
// 849 9♠ 4♠
// 850 9♠ 4♥
// 851 9♠ 4♦
// 852 9♠ 4♣
// 853 9♠ 3♠
// 854 9♠ 3♥
// 855 9♠ 3♦
// 856 9♠ 3♣
// 857 9♠ 2♠
// 858 9♠ 2♥
// 859 9♠ 2♦
// 860 9♠ 2♣
// 861 9♥ 9♦
// 862 9♥ 9♣
// 863 9♥ 8♠
// 864 9♥ 8♥
// 865 9♥ 8♦
// 866 9♥ 8♣
// 867 9♥ 7♠
// 868 9♥ 7♥
// 869 9♥ 7♦
// 870 9♥ 7♣
// 871 9♥ 6♠
// 872 9♥ 6♥
// 873 9♥ 6♦
// 874 9♥ 6♣
// 875 9♥ 5♠
// 876 9♥ 5♥
// 877 9♥ 5♦
// 878 9♥ 5♣
// 879 9♥ 4♠
// 880 9♥ 4♥
// 881 9♥ 4♦
// 882 9♥ 4♣
// 883 9♥ 3♠
// 884 9♥ 3♥
// 885 9♥ 3♦
// 886 9♥ 3♣
// 887 9♥ 2♠
// 888 9♥ 2♥
// 889 9♥ 2♦
// 890 9♥ 2♣
// 891 9♦ 9♣
// 892 9♦ 8♠
// 893 9♦ 8♥
// 894 9♦ 8♦
// 895 9♦ 8♣
// 896 9♦ 7♠
// 897 9♦ 7♥
// 898 9♦ 7♦
// 899 9♦ 7♣
// 900 9♦ 6♠
// 901 9♦ 6♥
// 902 9♦ 6♦
// 903 9♦ 6♣
// 904 9♦ 5♠
// 905 9♦ 5♥
// 906 9♦ 5♦
// 907 9♦ 5♣
// 908 9♦ 4♠
// 909 9♦ 4♥
// 910 9♦ 4♦
// 911 9♦ 4♣
// 912 9♦ 3♠
// 913 9♦ 3♥
// 914 9♦ 3♦
// 915 9♦ 3♣
// 916 9♦ 2♠
// 917 9♦ 2♥
// 918 9♦ 2♦
// 919 9♦ 2♣
// 920 9♣ 8♠
// 921 9♣ 8♥
// 922 9♣ 8♦
// 923 9♣ 8♣
// 924 9♣ 7♠
// 925 9♣ 7♥
// 926 9♣ 7♦
// 927 9♣ 7♣
// 928 9♣ 6♠
// 929 9♣ 6♥
// 930 9♣ 6♦
// 931 9♣ 6♣
// 932 9♣ 5♠
// 933 9♣ 5♥
// 934 9♣ 5♦
// 935 9♣ 5♣
// 936 9♣ 4♠
// 937 9♣ 4♥
// 938 9♣ 4♦
// 939 9♣ 4♣
// 940 9♣ 3♠
// 941 9♣ 3♥
// 942 9♣ 3♦
// 943 9♣ 3♣
// 944 9♣ 2♠
// 945 9♣ 2♥
// 946 9♣ 2♦
// 947 9♣ 2♣
// 948 8♠ 8♥
// 949 8♠ 8♦
// 950 8♠ 8♣
// 951 8♠ 7♠
// 952 8♠ 7♥
// 953 8♠ 7♦
// 954 8♠ 7♣
// 955 8♠ 6♠
// 956 8♠ 6♥
// 957 8♠ 6♦
// 958 8♠ 6♣
// 959 8♠ 5♠
// 960 8♠ 5♥
// 961 8♠ 5♦
// 962 8♠ 5♣
// 963 8♠ 4♠
// 964 8♠ 4♥
// 965 8♠ 4♦
// 966 8♠ 4♣
// 967 8♠ 3♠
// 968 8♠ 3♥
// 969 8♠ 3♦
// 970 8♠ 3♣
// 971 8♠ 2♠
// 972 8♠ 2♥
// 973 8♠ 2♦
// 974 8♠ 2♣
// 975 8♥ 8♦
// 976 8♥ 8♣
// 977 8♥ 7♠
// 978 8♥ 7♥
// 979 8♥ 7♦
// 980 8♥ 7♣
// 981 8♥ 6♠
// 982 8♥ 6♥
// 983 8♥ 6♦
// 984 8♥ 6♣
// 985 8♥ 5♠
// 986 8♥ 5♥
// 987 8♥ 5♦
// 988 8♥ 5♣
// 989 8♥ 4♠
// 990 8♥ 4♥
// 991 8♥ 4♦
// 992 8♥ 4♣
// 993 8♥ 3♠
// 994 8♥ 3♥
// 995 8♥ 3♦
// 996 8♥ 3♣
// 997 8♥ 2♠
// 998 8♥ 2♥
// 999 8♥ 2♦
// 1000 8♥ 2♣
// 1001 8♦ 8♣
// 1002 8♦ 7♠
// 1003 8♦ 7♥
// 1004 8♦ 7♦
// 1005 8♦ 7♣
// 1006 8♦ 6♠
// 1007 8♦ 6♥
// 1008 8♦ 6♦
// 1009 8♦ 6♣
// 1010 8♦ 5♠
// 1011 8♦ 5♥
// 1012 8♦ 5♦
// 1013 8♦ 5♣
// 1014 8♦ 4♠
// 1015 8♦ 4♥
// 1016 8♦ 4♦
// 1017 8♦ 4♣
// 1018 8♦ 3♠
// 1019 8♦ 3♥
// 1020 8♦ 3♦
// 1021 8♦ 3♣
// 1022 8♦ 2♠
// 1023 8♦ 2♥
// 1024 8♦ 2♦
// 1025 8♦ 2♣
// 1026 8♣ 7♠
// 1027 8♣ 7♥
// 1028 8♣ 7♦
// 1029 8♣ 7♣
// 1030 8♣ 6♠
// 1031 8♣ 6♥
// 1032 8♣ 6♦
// 1033 8♣ 6♣
// 1034 8♣ 5♠
// 1035 8♣ 5♥
// 1036 8♣ 5♦
// 1037 8♣ 5♣
// 1038 8♣ 4♠
// 1039 8♣ 4♥
// 1040 8♣ 4♦
// 1041 8♣ 4♣
// 1042 8♣ 3♠
// 1043 8♣ 3♥
// 1044 8♣ 3♦
// 1045 8♣ 3♣
// 1046 8♣ 2♠
// 1047 8♣ 2♥
// 1048 8♣ 2♦
// 1049 8♣ 2♣
// 1050 7♠ 7♥
// 1051 7♠ 7♦
// 1052 7♠ 7♣
// 1053 7♠ 6♠
// 1054 7♠ 6♥
// 1055 7♠ 6♦
// 1056 7♠ 6♣
// 1057 7♠ 5♠
// 1058 7♠ 5♥
// 1059 7♠ 5♦
// 1060 7♠ 5♣
// 1061 7♠ 4♠
// 1062 7♠ 4♥
// 1063 7♠ 4♦
// 1064 7♠ 4♣
// 1065 7♠ 3♠
// 1066 7♠ 3♥
// 1067 7♠ 3♦
// 1068 7♠ 3♣
// 1069 7♠ 2♠
// 1070 7♠ 2♥
// 1071 7♠ 2♦
// 1072 7♠ 2♣
// 1073 7♥ 7♦
// 1074 7♥ 7♣
// 1075 7♥ 6♠
// 1076 7♥ 6♥
// 1077 7♥ 6♦
// 1078 7♥ 6♣
// 1079 7♥ 5♠
// 1080 7♥ 5♥
// 1081 7♥ 5♦
// 1082 7♥ 5♣
// 1083 7♥ 4♠
// 1084 7♥ 4♥
// 1085 7♥ 4♦
// 1086 7♥ 4♣
// 1087 7♥ 3♠
// 1088 7♥ 3♥
// 1089 7♥ 3♦
// 1090 7♥ 3♣
// 1091 7♥ 2♠
// 1092 7♥ 2♥
// 1093 7♥ 2♦
// 1094 7♥ 2♣
// 1095 7♦ 7♣
// 1096 7♦ 6♠
// 1097 7♦ 6♥
// 1098 7♦ 6♦
// 1099 7♦ 6♣
// 1100 7♦ 5♠
// 1101 7♦ 5♥
// 1102 7♦ 5♦
// 1103 7♦ 5♣
// 1104 7♦ 4♠
// 1105 7♦ 4♥
// 1106 7♦ 4♦
// 1107 7♦ 4♣
// 1108 7♦ 3♠
// 1109 7♦ 3♥
// 1110 7♦ 3♦
// 1111 7♦ 3♣
// 1112 7♦ 2♠
// 1113 7♦ 2♥
// 1114 7♦ 2♦
// 1115 7♦ 2♣
// 1116 7♣ 6♠
// 1117 7♣ 6♥
// 1118 7♣ 6♦
// 1119 7♣ 6♣
// 1120 7♣ 5♠
// 1121 7♣ 5♥
// 1122 7♣ 5♦
// 1123 7♣ 5♣
// 1124 7♣ 4♠
// 1125 7♣ 4♥
// 1126 7♣ 4♦
// 1127 7♣ 4♣
// 1128 7♣ 3♠
// 1129 7♣ 3♥
// 1130 7♣ 3♦
// 1131 7♣ 3♣
// 1132 7♣ 2♠
// 1133 7♣ 2♥
// 1134 7♣ 2♦
// 1135 7♣ 2♣
// 1136 6♠ 6♥
// 1137 6♠ 6♦
// 1138 6♠ 6♣
// 1139 6♠ 5♠
// 1140 6♠ 5♥
// 1141 6♠ 5♦
// 1142 6♠ 5♣
// 1143 6♠ 4♠
// 1144 6♠ 4♥
// 1145 6♠ 4♦
// 1146 6♠ 4♣
// 1147 6♠ 3♠
// 1148 6♠ 3♥
// 1149 6♠ 3♦
// 1150 6♠ 3♣
// 1151 6♠ 2♠
// 1152 6♠ 2♥
// 1153 6♠ 2♦
// 1154 6♠ 2♣
// 1155 6♥ 6♦
// 1156 6♥ 6♣
// 1157 6♥ 5♠
// 1158 6♥ 5♥
// 1159 6♥ 5♦
// 1160 6♥ 5♣
// 1161 6♥ 4♠
// 1162 6♥ 4♥
// 1163 6♥ 4♦
// 1164 6♥ 4♣
// 1165 6♥ 3♠
// 1166 6♥ 3♥
// 1167 6♥ 3♦
// 1168 6♥ 3♣
// 1169 6♥ 2♠
// 1170 6♥ 2♥
// 1171 6♥ 2♦
// 1172 6♥ 2♣
// 1173 6♦ 6♣
// 1174 6♦ 5♠
// 1175 6♦ 5♥
// 1176 6♦ 5♦
// 1177 6♦ 5♣
// 1178 6♦ 4♠
// 1179 6♦ 4♥
// 1180 6♦ 4♦
// 1181 6♦ 4♣
// 1182 6♦ 3♠
// 1183 6♦ 3♥
// 1184 6♦ 3♦
// 1185 6♦ 3♣
// 1186 6♦ 2♠
// 1187 6♦ 2♥
// 1188 6♦ 2♦
// 1189 6♦ 2♣
// 1190 6♣ 5♠
// 1191 6♣ 5♥
// 1192 6♣ 5♦
// 1193 6♣ 5♣
// 1194 6♣ 4♠
// 1195 6♣ 4♥
// 1196 6♣ 4♦
// 1197 6♣ 4♣
// 1198 6♣ 3♠
// 1199 6♣ 3♥
// 1200 6♣ 3♦
// 1201 6♣ 3♣
// 1202 6♣ 2♠
// 1203 6♣ 2♥
// 1204 6♣ 2♦
// 1205 6♣ 2♣
// 1206 5♠ 5♥
// 1207 5♠ 5♦
// 1208 5♠ 5♣
// 1209 5♠ 4♠
// 1210 5♠ 4♥
// 1211 5♠ 4♦
// 1212 5♠ 4♣
// 1213 5♠ 3♠
// 1214 5♠ 3♥
// 1215 5♠ 3♦
// 1216 5♠ 3♣
// 1217 5♠ 2♠
// 1218 5♠ 2♥
// 1219 5♠ 2♦
// 1220 5♠ 2♣
// 1221 5♥ 5♦
// 1222 5♥ 5♣
// 1223 5♥ 4♠
// 1224 5♥ 4♥
// 1225 5♥ 4♦
// 1226 5♥ 4♣
// 1227 5♥ 3♠
// 1228 5♥ 3♥
// 1229 5♥ 3♦
// 1230 5♥ 3♣
// 1231 5♥ 2♠
// 1232 5♥ 2♥
// 1233 5♥ 2♦
// 1234 5♥ 2♣
// 1235 5♦ 5♣
// 1236 5♦ 4♠
// 1237 5♦ 4♥
// 1238 5♦ 4♦
// 1239 5♦ 4♣
// 1240 5♦ 3♠
// 1241 5♦ 3♥
// 1242 5♦ 3♦
// 1243 5♦ 3♣
// 1244 5♦ 2♠
// 1245 5♦ 2♥
// 1246 5♦ 2♦
// 1247 5♦ 2♣
// 1248 5♣ 4♠
// 1249 5♣ 4♥
// 1250 5♣ 4♦
// 1251 5♣ 4♣
// 1252 5♣ 3♠
// 1253 5♣ 3♥
// 1254 5♣ 3♦
// 1255 5♣ 3♣
// 1256 5♣ 2♠
// 1257 5♣ 2♥
// 1258 5♣ 2♦
// 1259 5♣ 2♣
// 1260 4♠ 4♥
// 1261 4♠ 4♦
// 1262 4♠ 4♣
// 1263 4♠ 3♠
// 1264 4♠ 3♥
// 1265 4♠ 3♦
// 1266 4♠ 3♣
// 1267 4♠ 2♠
// 1268 4♠ 2♥
// 1269 4♠ 2♦
// 1270 4♠ 2♣
// 1271 4♥ 4♦
// 1272 4♥ 4♣
// 1273 4♥ 3♠
// 1274 4♥ 3♥
// 1275 4♥ 3♦
// 1276 4♥ 3♣
// 1277 4♥ 2♠
// 1278 4♥ 2♥
// 1279 4♥ 2♦
// 1280 4♥ 2♣
// 1281 4♦ 4♣
// 1282 4♦ 3♠
// 1283 4♦ 3♥
// 1284 4♦ 3♦
// 1285 4♦ 3♣
// 1286 4♦ 2♠
// 1287 4♦ 2♥
// 1288 4♦ 2♦
// 1289 4♦ 2♣
// 1290 4♣ 3♠
// 1291 4♣ 3♥
// 1292 4♣ 3♦
// 1293 4♣ 3♣
// 1294 4♣ 2♠
// 1295 4♣ 2♥
// 1296 4♣ 2♦
// 1297 4♣ 2♣
// 1298 3♠ 3♥
// 1299 3♠ 3♦
// 1300 3♠ 3♣
// 1301 3♠ 2♠
// 1302 3♠ 2♥
// 1303 3♠ 2♦
// 1304 3♠ 2♣
// 1305 3♥ 3♦
// 1306 3♥ 3♣
// 1307 3♥ 2♠
// 1308 3♥ 2♥
// 1309 3♥ 2♦
// 1310 3♥ 2♣
// 1311 3♦ 3♣
// 1312 3♦ 2♠
// 1313 3♦ 2♥
// 1314 3♦ 2♦
// 1315 3♦ 2♣
// 1316 3♣ 2♠
// 1317 3♣ 2♥
// 1318 3♣ 2♦
// 1319 3♣ 2♣
// 1320 2♠ 2♥
// 1321 2♠ 2♦
// 1322 2♠ 2♣
// 1323 2♥ 2♦
// 1324 2♥ 2♣
// 1325 2♦ 2♣
// 1326
