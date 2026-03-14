using Avalonia.Controls;

namespace Galaga;

public partial class MainWindow : Window
{
    public MainWindow()
    {
        InitializeComponent();
        Opened += (_, _) => (Content as Control)?.Focus();
    }
}