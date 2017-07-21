﻿using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Net.Sockets;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace VfioService
{
    public partial class MainForm : Form
    {
        [Flags]
        public enum HotkeyModifiers : uint
        {
            Alt = 0x0001,
            Ctrl = 0x0002,
            Shift = 0x0004,
            Win = 0x0008,
            NoRepeat = 0x4000,
        }

        [DllImport("User32.dll", SetLastError = true)]
        private static extern bool RegisterHotKey(IntPtr hwnd, int id, HotkeyModifiers modifiers, Keys vk);

        public ClientManager ClientManager { get; set; }

        public MainForm()
        {
            InitializeComponent();
        }

        public string RegisterHotKey(int id, int mods, int keys)
        {
            HotkeyModifiers modifiers = (HotkeyModifiers)mods;
            Keys? key = (Keys)keys;

            if (!RegisterHotKey(Handle, id, modifiers, key.Value))
            {
                var exception = new Win32Exception();
                if ((uint)exception.HResult == 0x80004005)
                    // Hot key is already registered, so we can ignore.
                    return null;
                return "bind error: " + exception;
            }

            return null;
        }
        
        public string GetClipboardText()
        {
            if (!Clipboard.ContainsText())
                return null;

            return Clipboard.GetText(TextDataFormat.UnicodeText);
        }

        public Image GetClipboardImage()
        {
            if (!Clipboard.ContainsImage())
                return null;

            return Clipboard.GetImage();
        }

        public object SetClipboradText(string data)
        {
            Clipboard.SetText(data);
            return null;
        }

        public object SetClipboardImage(Image image)
        {
            Clipboard.SetImage(image);
            return null;
        }

        private const int WmPowerBroadcast = 0x0218;
        private const int WmHotkey = 0x0312;

        private const int PbtApmSuspend = 0x04;
        private const int PbtApmResume = 0x12;

        protected override void WndProc(ref Message m)
        {
            switch (m.Msg)
            {
                case WmHotkey:
                    lock (ClientManager.WriteLock)
                    {
                        ClientManager.SendCommand(CommandOut.HotKey);
                        ClientManager.SendData(BitConverter.GetBytes(m.WParam.ToInt32()));
                    }
                    break;
                case WmPowerBroadcast:
                    switch (m.WParam.ToInt64())
                    {
                        case PbtApmSuspend:
                            ClientManager.SendCommand(CommandOut.Suspending);
                            break;
                        case PbtApmResume:
                            ClientManager.SendCommand(CommandOut.ReportBoot);
                            break;
                    }
                    break;
            }

            base.WndProc(ref m);
        }
    }
}
