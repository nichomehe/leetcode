struct Solution {}

impl Solution {
  pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
    if arr.len() == 1 {
      if arr[0] == 1 {
        return arr[0] + k;
      } else if arr[0] > k {
        return k;
      } else if arr[0] < k {
        return k + 1;
      } else if arr[0] == k {
        return arr[0] + 1;
      }
    }
    let max_val = arr[arr.len() - 1];
    let mut res = vec![];
    for i in 1..=max_val {
      if arr.contains(&i) {
        continue;
      } else {
        res.push(i);
      }
    }
    if res.len() == 0 {
      return max_val + k;
    } else if k > res.len() as i32 {
      return max_val + k - res.len() as i32;
    } else {
      return res[(k - 1) as usize];
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    println!(
      "{:?}",
      Solution::find_kth_positive(
        vec![
          1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15, 16, 17, 20, 21, 22, 23, 24, 25, 27, 28,
          29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 42, 43, 44, 46, 47, 48, 49, 50, 51, 52,
          53, 54, 55, 56, 57, 58, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75,
          76, 77, 78, 80, 81, 82, 83, 85, 87, 88, 91, 92, 93, 94, 96, 97, 100, 101, 102, 103, 104,
          106, 107, 108, 110, 111, 113, 114, 115, 116, 117, 119, 120, 121, 122, 123, 124, 125, 126,
          127, 128, 129, 131, 132, 133, 134, 135, 136, 137, 139, 140, 141, 143, 144, 145, 146, 147,
          148, 149, 150, 152, 153, 155, 156, 158, 159, 160, 161, 163, 164, 166, 167, 168, 169, 171,
          172, 173, 174, 175, 177, 178, 179, 180, 181, 182, 184, 185, 186, 188, 189, 190, 192, 193,
          194, 196, 197, 198, 199, 200, 201, 202, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214,
          217, 219, 220, 221, 222, 223, 224, 226, 227, 228, 229, 230, 232, 233, 234, 235, 236, 237,
          240, 241, 243, 244, 245, 247, 248, 249, 250, 251, 252, 253, 254, 255, 256, 257, 258, 259,
          260, 261, 262, 263, 264, 265, 266, 267, 269, 270, 271, 272, 273, 274, 275, 276, 277, 278,
          279, 280, 281, 282, 283, 285, 286, 288, 289, 290, 291, 292, 293, 294, 296, 297, 298, 299,
          300, 301, 302, 303, 304, 305, 306, 308, 309, 310, 311, 312, 313, 314, 316, 317, 318, 319,
          320, 321, 322, 323, 324, 326, 327, 328, 329, 330, 331, 332, 333, 335, 336, 337, 338, 339,
          340, 341, 342, 343, 344, 346, 348, 349, 350, 352, 354, 355, 357, 359, 360, 361, 364, 365,
          366, 367, 368, 369, 370, 371, 372, 373, 375, 376, 377, 378, 380, 381, 382, 383, 384, 385,
          386, 390, 391, 392, 394, 395, 396, 398, 399, 400, 401, 402, 403, 404, 405, 406, 407, 409,
          410, 411, 413, 414, 415, 416, 417, 420, 421, 422, 423, 424, 425, 426, 427, 428, 429, 430,
          431, 432, 433, 435, 436, 438, 439, 440, 441, 442, 446, 447, 448, 449, 451, 452, 453, 454,
          455, 457, 458, 459, 460, 461, 462, 463, 465, 466, 467, 468, 469, 470, 471, 472, 474, 475,
          476, 477, 478, 479, 480, 481, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 495, 496,
          497, 498, 499, 500, 501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 512, 514, 515, 517,
          518, 519, 520, 521, 522, 523, 524, 526, 527, 528, 529, 531, 532, 533, 534, 535, 537, 538,
          539, 541, 543, 544, 545, 546, 548, 550, 551, 552, 553, 554, 555, 556, 557, 559, 560, 561,
          562, 563, 564, 565, 566, 568, 569, 570, 571, 572, 574, 575, 576, 577, 578, 579, 580, 581,
          582, 584, 586, 587, 588, 589, 591, 592, 593, 594, 595, 596, 597, 598, 600, 601, 602, 603,
          604, 605, 606, 607, 608, 609, 610, 611, 612, 613, 614, 615, 616, 620, 621, 622, 623, 624,
          626, 627, 628, 629, 630, 631, 632, 633, 634, 635, 636, 637, 638, 639, 640, 641, 642, 643,
          644, 645, 646, 647, 650, 651, 652, 653, 654, 655, 656, 659, 660, 661, 662, 665, 666, 667,
          668, 669, 671, 672, 673, 674, 675, 676, 677, 678, 679, 680, 682, 684, 685, 686, 687, 688,
          689, 690, 691, 693, 694, 695, 696, 697, 698, 700, 701, 702, 703, 705, 706, 707, 708, 709,
          710, 712, 713, 714, 715, 716, 717, 718, 719, 720, 722, 723, 724, 725, 726, 727, 728, 729,
          730, 732, 733, 735, 736, 737, 739, 740, 741, 742, 743, 744, 745, 746, 747, 748, 749, 750,
          751, 752, 753, 755, 757, 758, 759, 760, 761, 762, 763, 764, 766, 767, 768, 769, 770, 771,
          772, 773, 774, 775, 777, 778, 779, 780, 781, 782, 783, 784, 785, 786, 788, 789, 790, 791,
          792, 795, 796, 797, 798, 799, 800, 801, 803, 804, 805, 809, 810, 812, 813, 814, 816, 817,
          821, 822, 823, 824, 825, 826, 828, 829, 830, 831, 832, 833, 835, 836, 837, 838, 839, 841,
          842, 843, 844, 845, 846, 848, 849, 850, 851, 853, 855, 856, 857, 860, 861, 862, 863, 864,
          865, 866, 867, 868, 870, 871, 872, 873, 874, 876, 877, 878, 879, 880, 881, 882, 883, 887,
          889, 890, 891, 892, 893, 894, 895, 896, 897, 898, 900, 902, 904, 905, 907, 908, 909, 910,
          911, 913, 914, 915, 916, 918, 919, 920, 921, 922, 924, 926, 927, 929, 930, 931, 932, 933,
          934, 935, 936, 937, 938, 939, 942, 943, 944, 945, 946, 947, 948, 949, 950, 951, 952, 953,
          954, 956, 957, 959, 960, 961, 962, 963, 964, 965, 966, 967, 968, 970, 971, 972, 973, 974,
          975, 976, 977, 979, 980, 981, 982, 983, 984, 985, 986, 987, 989, 990, 991, 992, 994, 995,
          996, 997, 998, 999
        ],
        205
      )
    );
  }
}
