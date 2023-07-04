from setuptools import setup
import os
import glob
 
#NOTE: MAKE SURE YOUR LAUNCH FILE IS INSIDE <pkg_name>/launch
 
#GLOB STARTS FROM THE PACKAGE DIRECTORY WHEN USING setup.py
 
package_name = 'model_pkg'
 
setup(
    name=package_name,
    version='0.0.0',
    packages=[package_name],
    data_files=[
        ('share/ament_index/resource_index/packages',
            ['resource/' + package_name]),
        ('share/' + package_name, ['package.xml']),
                (os.path.join('share', package_name, 'model_pkg'), glob.glob('model_pkg/*')),
                (os.path.join('share', package_name, 'resource'), glob.glob('resource/*')),
                (os.path.join('share', package_name, 'rviz'), glob.glob('rviz/*')),
                (os.path.join('share', package_name, 'test'), glob.glob('test/*')),
                (os.path.join('share', package_name, 'launch'), glob.glob('launch/*')),
                (os.path.join('share', package_name, 'models'), glob.glob('models/*')),
                (os.path.join('share', package_name, 'worlds'), glob.glob('worlds/*')),
                (os.path.join('share', package_name, 'urdf'), glob.glob('urdf/*')),
    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='placeholder',
    maintainer_email='placeholder@gmail.com',
    description='TODO: Package description',
    license='TODO: License declaration',
    tests_require=['pytest'],
    entry_points={
        'console_scripts': [
            'model = model_pkg.model:main'
        ],
    },
)
