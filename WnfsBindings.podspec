Pod::Spec.new do |s|  
    s.name              = 'WnfsBindings' # Name for your pod
    s.version           = '1.0.0'
    s.summary           = 'Swift bindings for the rust WNFS library'
    s.homepage          = 'https://github.com/functionland/wnfs-swift-bindings'

    s.author            = { 'Homayoun Heidarzadeh' => 'hhio618@gmail.com' }
    s.license = { :type => 'MIT', :file => 'LICENSE' }

    s.platform          = :ios
    # change the source location
    s.source            = { :http => "https://github.com/functionland/wnfs-swift-bindings/releases/download/v#{s.version}/cocoapods-bundle.zip" } 
    s.ios.deployment_target = '13.0'
    s.vendored_framework = 'WnfsBindings.xcframework'
    s.static_framework = true
    s.user_target_xcconfig = { 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'arm64' }
    s.pod_target_xcconfig = { 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'arm64' }
end 
