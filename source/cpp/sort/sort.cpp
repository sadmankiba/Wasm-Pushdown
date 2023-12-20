#include <iostream>
#include <cstring>
#include <cassert>
#include <ctime>
#include <vector>
#include <chrono>

static long time_diff(timespec start, timespec end) {
    time_t sec = end.tv_sec - start.tv_sec;
    long nano = end.tv_nsec - start.tv_nsec;
    return (long) sec * 1000 * 1000 * 1000 + nano;
}

static void swap(std::vector<std::string>& arr, int lo, int hi) {
    std::string tmp = arr[lo];
    arr[lo] = arr[hi];
    arr[hi] = tmp;
}

static void wmerge(std::vector<std::string>& arr, int lo1, int hi1, int lo2, int hi2, int w) {
    while ((lo1 < hi1) && (lo2 < hi2)) {
        swap(arr, w++, (strcmp(arr[lo1].c_str(), arr[lo2].c_str()) <= 0) ? lo1++ : lo2++);
    }
    while (lo1 < hi1) {
        swap(arr, w++, lo1++);
    }
    while (lo2 < hi2) {
        swap(arr, w++, lo2++);
    }
}

static void imsort(std::vector<std::string>& arr, int lo, int hi);

static void wsort(std::vector<std::string>& arr, int lo, int hi, int w) {
    if ((hi - lo) > 1) {
        int m = (lo + hi) / 2;
        imsort(arr, lo, m);
        imsort(arr, m, hi);
        wmerge(arr, lo, m, m, hi, w);
    } else if (lo != hi) {
        swap(arr, lo, w);
    }
}

void imsort(std::vector<std::string>& arr, int lo, int hi) {
    if ((hi - lo) > 1) {
        int m = (lo + hi) / 2;
        int w = lo + hi - m;
        wsort(arr, lo, m, w);
        while ((w - lo) > 2) {
            int n = w;
            w = (lo + n + 1) / 2;
            wsort(arr, w, n, lo);
            wmerge(arr, lo, lo + n - w, n, hi, w);
        }
        for (int i = w; i > lo; i--) {
            for (int j = i; (j < hi) && (strcmp(arr[j].c_str(), arr[j - 1].c_str()) < 0); j++) {
                swap(arr, j, j - 1);
            }
        }
    }
}

static void permute(std::vector<std::string>& l, int n, int m, int pos) {
    if (n == 0) {
        l[0][pos] = '\0';
        return;
    }
    int size = 1;
    for (int i = 0; i < n - 1; i++) {
        size *= m;
    }
    for (int i = 0; i < m; i++) {
        for (int j = 0; j < size; j++) {
            l[i * size + j][pos] = 'z' - i;
        }
        permute(l, n - 1, m, pos + 1);
    }
}

static std::vector<std::string> gen_array(int n, int m, int& size) {
    timespec t0, t1;
    size = 1;
    for (int i = 0; i < n; i++) {
        size *= m;
    }
    std::vector<std::string> l(size);
    for (int i = 0; i < size; i++) {
        l[i] = std::string(n, ' ');
    }
    clock_gettime(CLOCK_MONOTONIC, &t0);
    permute(l, n, m, 0);
    clock_gettime(CLOCK_MONOTONIC, &t1);
    // std::cout << "[info] permute: " << time_diff(t0, t1) << " ns" << std::endl;
    return l;
}

static bool verify_array(std::vector<std::string>& l, int size) {
    for (int i = 1; i < size; i++) {
        if (strcmp(l[i - 1].c_str(), l[i].c_str()) > 0) {
            return false;
        }
    }
    return true;
}

void sort_bench(int n, int m) {
    timespec t0, t1, t2;
    int size;
    clock_gettime(CLOCK_MONOTONIC, &t0);
    std::vector<std::string> l = gen_array(n, m, size);
    if (size <= 100) {
        for (int i = 0; i < size; i++) {
            std::cout << l[i] << " ";
        }
    }
    // std::cout << std::endl;
    clock_gettime(CLOCK_MONOTONIC, &t1);
    imsort(l, 0, size);
    clock_gettime(CLOCK_MONOTONIC, &t2);
    if (size <= 100) {
        for (int i = 0; i < size; i++) {
            std::cout << l[i] << " ";
        }
    }
    // std::cout << std::endl;
    // std::cout << "[info] gen_array: " << time_diff(t0, t1) << " ns" << std::endl;
    // std::cout << "[info] sort: " << time_diff(t1, t2) << " ns" << std::endl;
    assert(verify_array(l, size));
}

int main(int argc, char* argv[]) {
    if (argc < 3) {
      std::cout << "Error: need 2 params" << std::endl;
      return 0;
    }

    // auto start = std::chrono::high_resolution_clock::now();
    sort_bench(atoi(argv[1]), atoi(argv[2]));
    // auto end = std::chrono::high_resolution_clock::now();

    // auto exec_time = std::chrono::duration_cast<std::chrono::microseconds>(end - start).count();

	// std::chrono::microseconds total_time = std::chrono::microseconds::zero();
    // for(int i = 0; i < 10; i++) {
	// 	start = std::chrono::high_resolution_clock::now();
	// 	sort_bench(atoi(argv[1]), atoi(argv[2]));
	// 	end = std::chrono::high_resolution_clock::now();
	// 	total_time += std::chrono::duration_cast<std::chrono::microseconds>(end - start);
	// }

	// // Print the duration
	// std::cout << exec_time * 1.0 / 1000 << "," << (total_time.count() * 1.0 / (1000 * 10)) << std::endl;

    
    return 0;
}
