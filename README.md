Dưới đây là nội dung chuẩn cho file `README.md` dành cho dự án **PaySlip** của bạn. File này không chỉ giới thiệu dự án mà còn ghi chú lại cấu trúc chuẩn để đảm bảo mã nguồn Backend (Rust) và Frontend (React) luôn được tách biệt, tránh gặp lại lỗi build trong tương lai.

Bạn hãy tạo một file tên là `README.md` ở thư mục gốc dự án (`/workspace/project/README.md`) và dán nội dung này vào:

```markdown
# PaySlip — Bảng Lương Token Hóa Trên Stellar 🚀

PaySlip là một ứng dụng phi tập trung (dApp) chạy trên mạng lưới Stellar. Nền tảng này giúp các nhà tuyển dụng thanh toán lương cho người lao động một cách minh bạch, nhanh chóng bằng USDC. Đồng thời, hệ thống tự động phát hành (mint) token WORK với tỷ lệ 1:1 nhằm làm bằng chứng thu nhập on-chain, giúp người lao động dễ dàng chứng minh tài chính hoặc tiếp cận các dịch vụ vay vốn.

## ✨ Tính Năng Chính

* **Thanh Toán Tức Thì:** Trả lương bằng USDC với chi phí giao dịch cực thấp và thời gian hoàn thành dưới 5 giây.
* **Chứng Minh Thu Nhập (WORK Token):** Tự động mint token WORK tương đương với số USDC được nhận.
* **Hoàn Tác (Clawback):** Nhà tuyển dụng có quyền hoàn tác giao dịch thanh toán trong vòng 24 giờ kể từ lúc gửi nếu có sai sót.
* **Lịch Sử Giao Dịch Minh Bạch:** Toàn bộ lịch sử nhận lương được lưu trữ on-chain và có thể tra cứu thông qua địa chỉ ví.
* **Xuất PDF:** Người lao động có thể xuất bảng lương dạng PDF trực tiếp từ giao diện để sử dụng làm hồ sơ.

## 📂 Cấu Trúc Dự Án

Dự án được chia thành 2 phần biệt lập hoàn toàn để đảm bảo quá trình biên dịch (build) diễn ra thành công:

```text
/workspace/project/
├── contracts/               # 🦀 BACKEND: Chứa mã nguồn Smart Contract (Rust)
│   └── payslip/
│       ├── Cargo.toml       # Cấu hình thư viện Rust
│       └── src/
│           └── lib.rs       # Logic cốt lõi của hợp đồng thông minh
│
└── frontend/                # ⚛️ FRONTEND: Chứa giao diện người dùng (React/JS)
    ├── package.json         # Cấu hình thư viện Node.js
    └── src/
        └── App.jsx          # Giao diện PaySlip tương tác với ví Freighter
```

> **⚠️ Lưu ý quan trọng:** Tuyệt đối không đặt code JavaScript, React, hoặc HTML vào trong thư mục `contracts/`. Trình biên dịch của Rust sẽ không thể đọc được và gây ra lỗi Token/Syntax error.

## 🛠 Yêu Cầu Cài Đặt (Prerequisites)

Để chạy dự án này trên máy thực tế, bạn cần cài đặt:

1. **Rust & Cargo:** Để build Smart Contract.
2. **Stellar CLI (`stellar-cli`):** Công cụ dòng lệnh của Stellar/Soroban để biên dịch và deploy contract.
3. **Node.js & npm/yarn:** Để chạy giao diện React Frontend.
4. **Ví Freighter:** Cài đặt extension [Freighter Wallet](https://freighter.app/) trên trình duyệt để kết nối và ký giao dịch.

## 🚀 Hướng Dẫn Khởi Chạy

### Bước 1: Biên dịch Smart Contract (Backend)

Di chuyển vào thư mục chứa hợp đồng thông minh và sử dụng công cụ của Stellar để build mã Rust thành file WebAssembly (`.wasm`):

```bash
cd contracts/payslip
stellar contract build
```
*Nếu thành công, file `.wasm` sẽ được tạo ra trong thư mục `target/wasm32-unknown-unknown/release/`.*

### Bước 2: Khởi chạy Giao Diện (Frontend)

Mở một terminal (cửa sổ dòng lệnh) mới, di chuyển vào thư mục frontend và khởi chạy máy chủ React:

```bash
cd frontend
npm install   # Cài đặt các thư viện cần thiết
npm start     # (Hoặc `npm run dev` tùy thuộc vào công cụ như Vite hay Create React App)
```

Giao diện sẽ được mở tại địa chỉ `http://localhost:3000` (hoặc cổng tương ứng). 

## 🔗 Tài Liệu Tham Khảo
* [Tài liệu Stellar Smart Contracts (Soroban)](https://developers.stellar.org/docs/smart-contracts)
* [Tài liệu Freighter API](https://docs.freighter.app/)
```
